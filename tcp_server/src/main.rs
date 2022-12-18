use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use dotenv::dotenv;
use std::env;
use std::fs::File;
fn main() {
    dotenv().ok();
    let host = env::var("HOST").expect("host must be set");

    let listener = TcpListener::bind(host).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "test.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();    
    let response = format!("{}{}",status_line,contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}