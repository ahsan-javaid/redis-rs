use crate::libs::stream_handler::Message;
use crate::libs::stream_handler::read_until_crlf;
use std::{ io::Result };
use std::io::Error;

/// To parse host port arguments
/// Generic enough to parse two args
/// Tuple of host and port
pub fn parse_multi_arg(args: &[String], arg_name: &str) -> Option<(String, u16)> {
  //format: --replicaof host port
  args.iter().position(|item| item == arg_name).map(|i| {
    (args.get(i+1).unwrap().clone(), args.get(i+2).unwrap().parse::<u16>().unwrap())
  })
}

/// To parse single arg
/// Generic enough to parse any arg
/// Option of type arg
pub fn parse_single_arg(args: &[String], arg_name: &str) -> Option<String> {
  args.iter().position(|item| item == arg_name).map(|i| {
    args.get(i+1).unwrap().clone()
  })
}

pub fn parse_bulk_string(input: String) -> Result<Message> {
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