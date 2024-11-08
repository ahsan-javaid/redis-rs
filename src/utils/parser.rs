use crate::libs::stream_handler::Message;
use std::io::Error;
use std::io::Result;

/// To parse host port arguments
/// Generic enough to parse two args
/// Tuple of host and port
pub fn parse_multi_arg(args: &[String], arg_name: &str) -> Option<(String, u16)> {
    //format: --replicaof host port
    println!("multi: {:?}", args);
    args.iter().position(|item| item == arg_name).map(|i| {
        (
            args.get(i + 1).unwrap().clone(),
            args.get(i + 2).unwrap().parse::<u16>().unwrap(),
        )
    })
}

/// To parse single arg
/// Generic enough to parse any arg
/// Option of type arg
pub fn parse_single_arg(args: &[String], arg_name: &str) -> Option<String> {
    args.iter()
        .position(|item| item == arg_name)
        .map(|i| args.get(i + 1).unwrap().clone())
}

pub fn parse_message(input: String) -> Result<Message> {
    match input.chars().next().unwrap() {
        '+' => parse_simple_string(input),
        '*' => parse_array(input),
        '$' => parse_bulk_string(input),
        _ => panic!("error parse message: no matcher found!"),
    }
}

pub fn read_until_crlf(input: String) -> Option<String> {
    let line = input.lines().next();

    match line {
        Some(v) => {
            return Some(v[1..].into());
        }
        None => {
            return None;
        }
    }
}

pub fn parse_simple_string(input: String) -> Result<Message> {
    if let Some(v) = read_until_crlf(input) {
        return Ok(Message::SimpleString(v));
    } else {
        return Err(Error::other("oh no!"));
    }
}

pub fn parse_array(input: String) -> Result<Message> {
    if let Some(v) = read_until_crlf(input.clone()) {
        let num_str = v.trim();

        let arr_len = num_str.parse::<i64>();
        // Convert the lines into a vector of owned strings
        let lines: Vec<String> = input.lines().map(String::from).collect();
        let mut items: Vec<Message> = vec![];

        let mut inc = 0;
        for cursor in 0..arr_len.unwrap() {
            let x = format!(
                "{}\r\n{}",
                lines[cursor as usize + 1 + inc],
                lines[cursor as usize + 2 + inc]
            );
            let result = parse_message(x);

            items.push(result.unwrap());
            inc = inc + 1;
        }
        return Ok(Message::Array(items));
    }

    return Err(Error::other("oh no!"));
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
