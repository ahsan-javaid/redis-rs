use std::{ net::TcpListener};
mod libs;
mod tests;
use libs::stream_handler::StreamHandler;
use std::thread;

fn main() {
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    let mut hander = StreamHandler::new(&stream);
                    hander.handle();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
