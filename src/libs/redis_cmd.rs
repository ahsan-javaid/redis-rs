use std::str::FromStr;

#[derive(Debug)]
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
  }
}