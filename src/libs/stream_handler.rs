use crate::libs::redis_cmd::RedisCmd;
use std:: {
  io:: { BufRead, BufReader, BufWriter, Result, Write },
  net:: { TcpStream },
  str:: FromStr
};

pub enum Value {
  SimpleString(String),
  BulkSttring(String),
  Array(Vec<Value>)
}
impl Value {
  pub fn serialize(self) -> String {
    match self {
      Value::SimpleString(s) => format!("+{}\r\n", s),
      Value::BulkSttring(s) => format!("${}\r\n{}\r\n", s.chars().count(), s),
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
    loop {
      let values: Vec<String> = Vec::new();

      let input: String = self._read().ok().unwrap();
     
      if input.len() == 0 {
        break;
      }
      

      for line in input.lines() {
        if line.trim().len() > 0 {

          // parser start 
          match line.chars().next().unwrap() {
            '+' => {
              // parse simple strings

              let mut temp: String = String::from(line.to_string());

              // remove + sign
              temp.remove(0);

              let parsed_value = Value::SimpleString(temp);
              // write back result
            }
            '*' => {
              // parse arrays

            }
            '$' => {
              // parse bulk strings

            }
            _ => {
              // panic
            }
          }
          // parser end

          let output: RedisCmd = RedisCmd::from_str(line.trim()).ok().unwrap();

          match output {
            RedisCmd::Ping => self._write(output),
            RedisCmd::Echo => {
              //  "*2\r\n$4\r\necho\r\n$9\r\nraspberry\r\n"
              // process echo 
              loop {
                let input: String = self._read().ok().unwrap();
     
                if input.len() == 0 {
                  break;
                }
                for l in input.lines() {

                  let resp = format!("$3\r\n{l}\r\n");
                  self.writer.write_all(resp.as_bytes()).unwrap();
                  self.writer.flush().unwrap();
                }
              }
            }
            RedisCmd::Unsupported => {}
          }
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

  pub  fn _write(&mut self, cmd: RedisCmd) {
     let response = cmd.response();
     if response.len() > 0 {
      self.writer.write_all(response.as_bytes()).unwrap();
      self.writer.flush().unwrap();
     }
  }
}