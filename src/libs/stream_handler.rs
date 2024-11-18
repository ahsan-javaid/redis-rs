use crate::libs::redis_cmd::RedisCmd;
use crate::utils::parser::{ parse_message };
use std:: {
  io:: { BufRead, BufReader, BufWriter, Result, Write },
  net:: { TcpStream },
  str:: FromStr, hash::Hash,
};
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::env;

#[derive(Debug, Clone)]
pub enum Message {
  SimpleString(String),
  BulkString(String),
  Array(Vec<Message>)
}

impl Message {
  pub fn serialize(self) -> String {
    match self {
      Message::SimpleString(s) => format!("+{}\r\n", s),
      Message::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
      Message::Array(arr) => {
        let mut s = String::from("");

        arr.iter().skip(1).for_each(|x| {
          
          let sj = x.clone().serialize();

          s.push_str(sj.as_str());
        });

        return s;
      }
      _ => panic!("unsupported value for serialize")
    }
  }
}
pub struct StreamHandler<'a> {
  reader: BufReader<TcpStream>,
  writer: BufWriter<&'a TcpStream>
}

impl<'a> StreamHandler <'a> {

  pub fn new(stream:&'a TcpStream) -> Self {
    Self {
      reader: BufReader::new(stream.try_clone().unwrap()),
      writer: BufWriter::new(stream)
    }
  }

  pub fn handle(&mut self, store: &Arc<Mutex<HashMap<String, (String, Option<SystemTime>)>>>,
  config: &Arc<Mutex<HashMap<String, String>>>
  ) {
    loop {
      let input_value: String = self._read().ok().unwrap();
      
      if input_value.len() == 0 {
        break;
      }

      let output: RedisCmd = RedisCmd::from_str(input_value.as_str()).ok().unwrap();

      match output {
        RedisCmd::Ping => self._write("+PONG\r\n".to_string()),
        RedisCmd::Echo => {
          let response = parse_message(input_value.clone());
          match response {
            Ok(v) => {
               self._write(v.serialize())
            },
            Err(_) => {
              println!("Cannot write anything to output")
            }
          }
        },
       RedisCmd::GET => {
        let response = parse_message(input_value.clone());
        println!("get msg: {:?}", response);
        match response {
          Ok(v) => {
            let mut key_values = vec![];
            if let Message::Array(ref arr) = v {
              arr.iter().skip(1).for_each(|x| {
                if let Message::BulkString(blk) = x {
                  key_values.push(blk);
                }
              })
            }
            
            let mut map = store.lock().unwrap();

            if map.contains_key(key_values[0]) {
              let val = map.get(key_values[0]).unwrap();

              // check expiry
              match val.1 {
                Some(j) => {
                  if j > SystemTime::now() {
                    // valid
                    self._write(format!("${}\r\n{}\r\n", val.0.len(), val.0));
                  } else {
                    map.remove(key_values[0]);
                    self._write("$-1\r\n".to_string());
                  }
                },
                None => {
                  self._write(format!("${}\r\n{}\r\n", val.0.len(), val.0));
                }
              }
            } else {
              self._write("$-1\r\n".to_string());
            }
          },
          Err(_) => {
            println!("Cannot write anything to output")
          }
        }
       },
       RedisCmd::SET => {
        let response = parse_message(input_value.clone());
        match response {
          Ok(v) => {
            let mut key_values = vec![];
            if let Message::Array(ref arr) = v {
              arr.iter().skip(1).for_each(|x| {
                if let Message::BulkString(blk) = x {
                  key_values.push(blk);
                }
              })
            }

            let mut map = store.lock().unwrap();

            if !map.contains_key(key_values[0]) {


              if key_values.get(2) != None { // px
                let raw_time = key_values.get(3).unwrap();
                let time_int = raw_time.parse::<u64>();

                let expiry_time = SystemTime::now() + Duration::from_millis(time_int.unwrap());

                map.insert(key_values[0].clone(), (key_values[1].clone(), Some(expiry_time)));

              } else {
                map.insert(key_values[0].clone(), (key_values[1].clone(), None));

              }

            }

            self._write("+OK\r\n".to_string());
          },
          Err(_) => {
            println!("Cannot write anything to output")
          }
        }
       },
       RedisCmd::Config => {
        let response = parse_message(input_value.clone());

        match response {
          Ok(v) => {
            // check if its get dir or dbfilename
            let res = format!(
              "*2\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
              "key".len(),
              "value",
              "value".len(),
              "value"
          );
          
          self._write(res);
          },
          Err(_) => {
            println!("Cannot write anything to output")
          }
        }
       },
       RedisCmd::Replconf => {
        self._write("+OK\r\n".to_string());
       },
       RedisCmd::Psync => {
        self._write("+OK\r\n".to_string());
      },
       RedisCmd::Info => { 
        let response = parse_message(input_value.clone());
        match response {
          Ok(v) => {
            let args: Vec<String> = env::args().collect();
            println!("all args: {:?}", args);
            let reply = if let Some(_) = args.iter().position(|x| x.contains("replicaof")) {
               "role:slave"
            } else {
               "role:master" 
            };
            
            // hardcoded values
            let replica = "master_replid:8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb";
            let offset = "master_repl_offset:0";
            
            self._write(format!("${}\r\n{}{}{}\r\n", reply.len() + replica.len() + offset.len(), reply, replica, offset));
          },
          Err(_) => {
            println!("Cannot write anything to output")
          }
        }
       }, 
       _ => {
          println!("Unsupported redis command")
        }
      }
    }
  }

  pub fn _read(&mut self) -> Result<String> {
    let received: Vec<u8> = self.reader.fill_buf()?.to_vec();

    self.reader.consume(received.len());

    String::from_utf8(received).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Couldn't parse received string as utf8",
        )
    })
  }

  pub fn _write(&mut self, val: String) {
    self.writer.write_all(val.as_bytes()).unwrap();
    self.writer.flush().unwrap();
  }
}