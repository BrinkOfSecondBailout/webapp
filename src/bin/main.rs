use std::{
    fs::{self, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::{self, sleep},
    time::Duration,
};

use webapp_r::ThreadPool;

const LOCAL_HOST: &str = "127.0.0.1:7878";

fn handle_connection(mut stream: TcpStream) {
    let get = "GET / HTTP/1.1".as_bytes();
    let sleep = "GET /sleep HTTP/1.1".as_bytes();
    let sleep_long = "GET /sleep_long HTTP/1.1".as_bytes();

    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep_long) {
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
    println!("Finished writing to stream");
}

fn main() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind(LOCAL_HOST).unwrap();
    println!("Listening on {}", LOCAL_HOST);
    for stream in listener.incoming().take(2) {
        println!("Incoming...");
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}
