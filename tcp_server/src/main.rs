use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use dotenv::dotenv;
use std::env;
use std::thread;
use std::time::Duration;
use tcp_server::ThreadPool;
pub mod html_tags;
pub mod constant;
fn main() {
    dotenv().ok();
    let host = env::var("HOST").expect("host must be set");

    let listener = TcpListener::bind(host).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(1000) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
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
    let contents = "asdfasdfadsfasfd";
    // file.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}",status_line,contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}