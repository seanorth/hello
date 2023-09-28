use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use actix_web::{get,web,App,HttpServer,Responder};

use hello::ThreadPool;

fn main2() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        pool.execute(move || handle_connection(stream));
    }
    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // println!("Request: {}",String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder{
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        App::new().service(greet)
    })
    .bind(("127.0.0.1",7878))?
    .run()
    .await
}