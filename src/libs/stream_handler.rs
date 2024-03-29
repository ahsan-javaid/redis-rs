use crate::libs::redis_cmd::RedisCmd;
use std:: {
  io:: { BufRead, BufReader, BufWriter, Result, Write },
  net:: { TcpStream },
  str:: FromStr
};

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
      let input: String = self._read().ok().unwrap();
     
      if input.len() == 0 {
        break;
      }

      for line in input.lines() {
        if line.trim().len() > 0 {
          let output: RedisCmd = RedisCmd::from_str(line.trim()).ok().unwrap();
          match output {
            RedisCmd::Ping => self._write(output),
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