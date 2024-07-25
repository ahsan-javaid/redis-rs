use std::{ net::TcpListener, net::TcpStream, io::{Write} };
mod libs;
mod utils;
mod tests;
use libs::stream_handler::StreamHandler;
use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime};
use std::env;


fn main() {
    println!("Logs from your program will appear here!");
    // env //
    let args: Vec<String> = env::args().collect();
    
    let mut port_num = "6379".to_string();
    let mut dir = "".to_string();
    let mut dbfilename = "".to_string();

    if let Some(port) = utils::parser::parse_single_arg(&args, "--port") {
        port_num = port;
    }

    if let Some(dir_path) = utils::parser::parse_single_arg(&args, "--dir") {
        dir = dir_path;
    }

    if let Some(db_file_name) = utils::parser::parse_single_arg(&args, "--dbfilename") {
        dbfilename = db_file_name;
    }

    // parse replica
    let _role = if let Some((host, port)) = utils::parser::parse_multi_arg(&args, "--replicaof") {
        let mut stream = TcpStream::connect(format!("{}:{}", host, port))
        .expect("failed to connect to master server");

        stream.write_all(b"*1\r\n$4\r\nping\r\n").expect("failed to ping master server");

        "slave"
    } else {
        "master"
    };

    // config handling

    let config = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    config
    .lock()
    .unwrap()
    .insert("dir".to_string(), dir);

    config
    .lock()
    .unwrap()
    .insert("dbfilename".to_string(), dbfilename);

    let bind_address = format!("127.0.0.1:{port_num}");

    println!("listening on: {}", bind_address);

    // create a store here 
    let hmap: HashMap<String, (String, Option<SystemTime>)> = HashMap::new();

    let store = Arc::new(Mutex::new(hmap));

    let listener = TcpListener::bind(bind_address).unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store_clone = store.clone();
                let config_clone = config.clone();
                thread::spawn(move || {
                    let mut hander = StreamHandler::new(&stream);
                    hander.handle(&store_clone, &config_clone);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
