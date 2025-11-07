use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use webapp_r::ThreadPool;

const LOCAL_HOST: &str = "127.0.0.1:7878";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let get = "GET / HTTP/1.1".as_bytes();
    let sleep = "GET /sleep HTTP/1.1".as_bytes();
    let sleep_10 = "GET /sleep10 HTTP/1.1".as_bytes();
    stream.read(&mut buffer).unwrap();
    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep_10) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 Not Found", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
fn main() {
    let listener = TcpListener::bind(LOCAL_HOST).unwrap();
    println!("Listening on {}", LOCAL_HOST);
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        println!("Incoming connection!");
        let stream = stream.unwrap();
        // handle_connection(stream);
        pool.execute(|| handle_connection(stream));
    }
}
