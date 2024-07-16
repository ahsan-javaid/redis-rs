// Return tuple of host and port 
pub fn parse_replica(args: &[String]) -> Option<(String, u16)> {
  //format: --replicaof host port
  args.iter().position(|item| item == "--replicaof").map(|i| {
    (args.get(i+1).unwrap().clone(), args.get(i+2).unwrap().parse::<u16>().unwrap())
  })
}

pub fn parse_port(args: &[String]) -> Option<String> {
  args.iter().position(|item| item == "--port").map(|i| {
    args.get(i+1).unwrap().clone()
  })
}