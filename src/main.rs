use std::io::Read;
use std::{io::Write, net::TcpListener, net::TcpStream};
mod libs;
mod tests;
mod utils;
use libs::stream_handler::StreamHandler;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;
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
    let _role = if let Some(addr) = utils::parser::parse_single_arg(&args, "--replicaof") {
        let chunk: Vec<&str> = addr.split(' ').collect();
        let host = chunk.first();
        let port = chunk.last();

        if chunk.len() == 2 && host.is_some() && port.is_some() {
            let mut stream = TcpStream::connect(format!("{}:{}", host.unwrap(), port.unwrap()))
                .expect("failed to connect to master server");

            match stream.write_all(b"*1\r\n$4\r\nping\r\n") {
                Ok(_) => println!("sent"),
                Err(e) => eprintln!("Failed to send first message: {}", e),
            }

            stream.flush();

            // First wait for message from master
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer);

            let mut bytes: usize = 0;

            match bytes_read {
                Ok(value) => {
                    bytes = value;
                }
                Err(_) => {}
            }
            println!("Bytes  read {:?}", bytes_read);

            let res = String::from_utf8_lossy(&buffer[..bytes]);

            println!("Final response: {res}");

            let s = format!(
                "*3\r\n$8\r\nREPLCONF\r\n$14\r\nlistening-port\r\n$4\r\n{}\r\n",
                port_num
            )
            .as_bytes()
            .to_vec();

            match stream.write_all(&s) {
                Ok(_) => println!("sent"),
                Err(e) => eprintln!("Failed to send first message: {}", e),
            }

            stream.flush();

            let _ = stream.read(&mut buffer);

            let s = b"*3\r\n$8\r\nREPLCONF\r\n$4\r\ncapa\r\n$6\r\npsync2\r\n";

            match stream.write_all(s) {
                Ok(_) => println!("sent"),
                Err(e) => eprintln!("Failed to send first message: {}", e),
            }

            stream.flush();

            let _ = stream.read(&mut buffer);

            let psync_str =b"*3\r\n$5\r\nPSYNC\r\n$1\r\n?\r\n$2\r\n-1\r\n";
            
            match stream.write_all(psync_str) {
                Ok(_) => println!("sent"),
                Err(e) => eprintln!("Failed to send first message: {}", e),
            }
            stream.flush();
            
            let _ = stream.read(&mut buffer);
            // let _ = stream.read(&mut buffer);
        }
        "slave"
    } else {
        "master"
    };

    // config handling

    let config = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    config.lock().unwrap().insert("dir".to_string(), dir);

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
