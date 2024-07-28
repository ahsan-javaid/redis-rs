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