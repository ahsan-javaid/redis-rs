use std::str::FromStr;

pub enum RedisCmd {
  Ping,
  Unsupported
}

impl FromStr for RedisCmd {
  type Err = ();
  
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input.to_lowercase().as_str() {
      "ping\n" => Ok(RedisCmd::Ping),
      "*1\r\n$4\r\nping\r\n" => Ok(RedisCmd::Ping),
      _ => Ok(RedisCmd::Unsupported)
    }
  }
}

impl RedisCmd {
  pub fn response(&self) -> &'static str {
    match self {
      Self::Ping => "+PONG\r\n",
      Self::Unsupported => "unsupported\r\n"
    }
  }
}