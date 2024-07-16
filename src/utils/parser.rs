// Return tuple of host and port 
pub fn parse_replica(args: &[String]) -> Option<(String, u16)> {
  //format: --replicaof host port
  args.iter().position(|item| item == "--replicaof").map(|i| {
    (args.get(i+1).unwrap().clone(), args.get(i+2).unwrap().parse::<u16>().unwrap())
  })
}

// parse port
// if let Some(index) = args.iter().position(|x| x.contains("port")) {
//   if index < args.len() - 1 {
//       port_num = args[index + 1].clone();
//   }
// }
pub fn parse_port(args: &[String]) -> Option<String> {
  args.iter().position(|item| item == "--port").map(|i| {
    args.get(i+1).unwrap().clone()
  })
}