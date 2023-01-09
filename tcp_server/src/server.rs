pub mod thread_pool;

use std::net::TcpStream;
use std::io::prelude::*;
use std::time::Duration;
use std::thread;
use http_status;

pub fn handle_connection(mut stream: TcpStream,contents: String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    
    let (status,res) = if buffer.starts_with(get){
        (200,contents)
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(150));
        (403,contents)
    }else{
        (404,"404".to_string())
    };
    let status_line = http_status::get_http_status(status);
    println!("{}",res);
    let response = format!("{}{}",status_line,res);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}