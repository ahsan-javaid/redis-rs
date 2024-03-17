use std::{ net::TcpListener};
mod libs;
use libs::stream_handler::StreamHandler;

fn main() {
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut hander = StreamHandler::new(&stream);
                hander.handle();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
