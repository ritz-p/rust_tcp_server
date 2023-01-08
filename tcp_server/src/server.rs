pub mod thread_pool;

use std::net::TcpStream;
use std::io::prelude::*;
use std::time::Duration;
use std::thread;

pub fn handle_connection(mut stream: TcpStream,contents: String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    
    let status_line = if buffer.starts_with(get){
        "HTTP/1.1 200 OK\r\n\r\n"
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        "HTTP/1.1 200 OK\r\n\r\n"
    }else{
        "HTTP/1.1 404 NOT FOUND\r\n\r\n"
    };
    // let mut file = File::open(filename).unwrap();
    let contents = contents;
    println!("{}",contents);
    // file.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}",status_line,contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}