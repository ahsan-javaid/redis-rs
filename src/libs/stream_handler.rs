use crate::libs::redis_cmd::RedisCmd;
use std:: {
  io:: { BufRead, BufReader, BufWriter, Result, Write },
  net:: { TcpStream },
  str:: FromStr,
};
use std::io::Error;

#[derive(Debug)]
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

  pub fn handle(&mut self) {

    // new code 
    loop {
      let input_value: String = self._read().ok().unwrap();
      
      if input_value.len() == 0 {
        break;
      }

      let output: RedisCmd = RedisCmd::from_str(input_value.lines().next().unwrap()).ok().unwrap();

      match output {
        RedisCmd::Ping => self._write_v2("+PONG\r\n".to_string()),
        RedisCmd::Echo => {
          let response = parse_message(input_value.clone());
          match response {
            Ok(v) => {
              println!("value: {:?}", v);
              println!("response: {:?}", v);

               self._write_v2(v.serialize())
            },
            Err(_) => {
              println!("Cannot write anything to output")
            }
          }
        },
        _ => {
          println!("Cannot write anything to output")
        }
      }
    }



    // old code
    // loop {
    //   let values: Vec<String> = Vec::new();

    //   let input: String = self._read().ok().unwrap();
     
    //   if input.len() == 0 {
    //     break;
    //   }
      

    //   for line in input.lines() {
    //     if line.trim().len() > 0 {

    //        // *2\r\n$4\r\necho\r\n$3\r\nhey\r\n
    //       // parser start 
    //       match line.chars().next().unwrap() {
    //         '+' => {
    //           // parse simple strings

    //           let mut temp: String = String::from(line.to_string());

    //           // remove + sign
    //           temp.remove(0);

    //           let parsed_value = Value::SimpleString(temp);
    //           // write back result
    //         }
    //         '*' => {
    //           // parse arrays

    //         }
    //         '$' => {
    //           // parse bulk strings

    //         }
    //         _ => {
    //           // panic
    //         }
    //       }
    //       // parser end

    //       let output: RedisCmd = RedisCmd::from_str(line.trim()).ok().unwrap();

    //       match output {
    //         RedisCmd::Ping => self._write(output),
    //         RedisCmd::Echo => {
    //           //  "*2\r\n$4\r\necho\r\n$9\r\nraspberry\r\n"
    //           // process echo 
    //           loop {
    //             let input: String = self._read().ok().unwrap();
     
    //             if input.len() == 0 {
    //               break;
    //             }
    //             for l in input.lines() {

    //               let resp = format!("$3\r\n{l}\r\n");
    //               self.writer.write_all(resp.as_bytes()).unwrap();
    //               self.writer.flush().unwrap();
    //             }
    //           }
    //         }
    //         RedisCmd::Unsupported => {}
    //       }
    //     }
    //   }
    // }
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

  pub  fn _write(&mut self, cmd: RedisCmd) {
     let response = cmd.response();
     if response.len() > 0 {
      self.writer.write_all(response.as_bytes()).unwrap();
      self.writer.flush().unwrap();
     }
  }

  pub fn _write_v2(&mut self, val: String) {
    self.writer.write_all(val.as_bytes()).unwrap();
    self.writer.flush().unwrap();
  }
}

fn parse_message(input: String) -> Result<Message> {
  match input.chars().next().unwrap() {
    '+' => parse_simple_string(input),
    '*' => parse_array(input),
    '$' => parse_bulk_string(input),
    _ => panic!("error parse message: no matcher found!")
  }
}

fn read_until_crlf(input: String) -> Option<String> {
   let line = input.lines().next();
   
   match line {
    Some(v) => {
      return Some(v[1..].into());
    },
    None => {
      return None;
    }
   }
}

fn parse_simple_string(input: String) -> Result<Message> {
  if let Some(v) = read_until_crlf(input) {
   
    return Ok(Message::SimpleString(v));
  } else {
    return Err(Error::other("oh no!"));
  }
}

fn parse_array(input: String) -> Result<Message> {
  // *2\r\n$5\r\nhello\r\n$5\r\nworld\r\n
  if let Some(v) = read_until_crlf(input.clone()) {
    let num_str = v.trim();

    let arr_len = num_str.parse::<i64>();


    let mut items: Vec<Message> = vec![];

    for (i, line) in input.lines().enumerate() {
      if i > 0 {
        let result = parse_message(line.to_string());

        items.push(result.unwrap());
      }

    }

   
    return Ok(Message::Array(items));
  } 

  return Err(Error::other("oh no!"));
}

fn parse_bulk_string(input: String) -> Result<Message> {
  // *2\r\n$5\r\nhello\r\n$5\r\nworld\r\n
  if let Some(v) = read_until_crlf(input.clone()) {
    let num_str = v.trim();

    let bulk_len = num_str.parse::<i64>();

    print!("bulk_len {:?}", bulk_len);
    for (i, line) in input.lines().enumerate() {
      if i > 0 {
        return Ok(Message::BulkString(line.to_string()));
      }

    }
  } 

  return Err(Error::other("oh no!"));
}