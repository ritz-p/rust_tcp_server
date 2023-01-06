extern crate tcp_server;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use dotenv::dotenv;
use tcp_server::html_tags;
use std::env;
use std::fs::File;
use std::thread;
use std::time::Duration;
use tcp_server::ThreadPool;

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
    let contents = return_html();
    // file.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}",status_line,contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}

pub fn return_html() -> String{
    let html = html_tags::html::Tag{
        name: "html"
    };
    let body = html_tags::html::Tag{
        name: "body"
    };
    let title = html_tags::html::Tag{
        name: "title"
    };
    html_tags::html::PairTag::bind(&html,html_tags::html::PairTag::bind(&body,html_tags::html::PairTag::bind(&title,"Test!!!".to_owned())))
}

// const html: html_tags::html::Tag = html_tags::html::Tag{
//     name: "html"
// };