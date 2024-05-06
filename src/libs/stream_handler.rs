use crate::libs::redis_cmd::RedisCmd;
use std:: {
  io:: { BufRead, BufReader, BufWriter, Result, Write },
  net:: { TcpStream },
  str:: FromStr,
};
use std::io::Error;

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

  pub fn handle(&mut self) {
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

pub fn parse_message(input: String) -> Result<Message> {
  match input.chars().next().unwrap() {
    '+' => parse_simple_string(input),
    '*' => parse_array(input),
    '$' => parse_bulk_string(input),
    _ => panic!("error parse message: no matcher found!")
  }
}

pub fn read_until_crlf(input: String) -> Option<String> {
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
  if let Some(v) = read_until_crlf(input.clone()) {
    let num_str = v.trim();

    let arr_len = num_str.parse::<i64>();
    // Convert the lines into a vector of owned strings
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut items: Vec<Message> = vec![];

    let mut inc = 0;
    for cursor in 0..arr_len.unwrap() {
             
      let x = format!("{}\r\n{}", lines[cursor as usize + 1 + inc], lines[cursor as usize + 2 + inc]);
      let result = parse_message(x);
          
      items.push(result.unwrap());
      inc = inc + 1;
    }
    return Ok(Message::Array(items));
  } 

  return Err(Error::other("oh no!"));
}

fn parse_bulk_string(input: String) -> Result<Message> {
  if let Some(v) = read_until_crlf(input.clone()) {
    let num_str = v.trim();

    let _ = num_str.parse::<i64>();

    for (i, line) in input.lines().enumerate() {
      if i > 0 {
        return Ok(Message::BulkString(line.to_string()));
      }

    }
  } 

  return Err(Error::other("oh no!"));
}