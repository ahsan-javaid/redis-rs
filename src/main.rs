use std::{ net::TcpListener};
mod libs;
mod tests;
use libs::stream_handler::StreamHandler;
use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime};


fn main() {
    println!("Logs from your program will appear here!");
    // create a store here 
    let hmap: HashMap<String, (String, Option<SystemTime>)> = HashMap::new();

    let store = Arc::new(Mutex::new(hmap));

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store_clone = store.clone();
                thread::spawn(move || {
                    let mut hander = StreamHandler::new(&stream);
                    hander.handle(&store_clone);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
