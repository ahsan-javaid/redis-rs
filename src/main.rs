use std::{ net::TcpListener};
mod libs;
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

    if let Some(index) = args.iter().position(|x| x.contains("port")) {
        if index < args.len() - 1 {
            port_num = args[index + 1].clone();
        }
    }

    if let Some(index) = args.iter().position(|x| x.contains("dir")) {
        if index < args.len() - 1 {
            dir = args[index + 1].clone();
        }
    }

    if let Some(index) = args.iter().position(|x| x.contains("dbfilename")) {
        if index < args.len() - 1 {
            dbfilename = args[index + 1].clone();
        }
    }

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
