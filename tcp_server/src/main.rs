use dotenv::dotenv;
use std::env;
use std::net::TcpListener;

pub mod server;

use article_base;
use server::thread_pool::ThreadPool;
fn main() {
    dotenv().ok();
    let host = env::var("HOST").expect("host must be set");

    let listener = TcpListener::bind(host).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(1000) {
        let stream = stream.unwrap();

        pool.execute(|| {
            server::handle_connection(stream, article_base::create_article_base());
        });
    }
}
