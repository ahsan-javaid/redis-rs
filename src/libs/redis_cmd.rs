use std::str::FromStr;

#[derive(Debug)]
pub enum RedisCmd {
  Ping,
  Echo,
  SET,
  GET,
  Info,
  Config,
  Replconf,
  Psync,
  Unsupported
}

impl FromStr for RedisCmd {
  type Err = ();
  
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    if input.to_lowercase().contains("ping") {
      Ok(RedisCmd::Ping)
    } else if input.to_lowercase().contains("echo") {
      Ok(RedisCmd::Echo)
    } else if input.to_lowercase().contains("get") {
      Ok(RedisCmd::GET) 
    } else if input.to_lowercase().contains("set") {
      Ok(RedisCmd::SET)
    } else if input.to_lowercase().contains("info") {
      Ok(RedisCmd::Info)
    } else if input.to_lowercase().contains("config") {
        Ok(RedisCmd::Config)  
    } else if input.to_lowercase().contains("replconf") {
        Ok(RedisCmd::Replconf)
    } else if input.to_lowercase().contains("psync") {
        Ok(RedisCmd::Psync)
    } else {
      Ok(RedisCmd::Unsupported)
    }
  }
}