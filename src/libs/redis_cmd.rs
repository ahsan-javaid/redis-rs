use std::str::FromStr;

pub enum RedisCmd {
  Ping,
  Echo,
  Unsupported
}

impl FromStr for RedisCmd {
  type Err = ();
  
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    if input.to_lowercase().contains("ping") {
      Ok(RedisCmd::Ping)
    } else if input.to_lowercase().contains("echo") {
      Ok(RedisCmd::Echo)
    } else {
      Ok(RedisCmd::Unsupported)
    }
    // match input.to_lowercase().as_str() {
    //   "ping\n" | "*1\r\n$4\r\nping\r\n" | "ping" => Ok(RedisCmd::Ping),
    //   _ => Ok(RedisCmd::Unsupported)
    // }
  }
}

impl RedisCmd {
  pub fn response(&self) -> &'static str {
    match self {
      Self::Ping => "+PONG\r\n",
      Self::Echo => "",
      Self::Unsupported => ""
    }
  }
}