use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    match request_line.as_str() {
        "GET / HTTP/1.1" => respond_with_index(stream),
        s if s.contains("/echo") => respond_with_echo(stream, request_line),
        _ => respond_with_not_found(stream),
    }
}

fn respond_with_index(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html><html><head><title>Index</title></head><body><h1>Hello, World!</h1></body></html>";
    stream.write_all(response.as_bytes()).unwrap();
}

fn respond_with_not_found(mut stream: TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n<!DOCTYPE html><html><head><title>Not Found</title></head><body><h1>Not Found</h1></body></html>";
    stream.write_all(response.as_bytes()).unwrap();
}

fn respond_with_echo(mut stream: TcpStream, request_line: String) {
    let echo_message = &request_line.split("/echo").collect::<Vec<&str>>()[1].trim().split(" ").collect::<Vec<&str>>()[0][1..];
    let echo_length = echo_message.len() as u64;
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo_length, echo_message);
    stream.write_all(response.as_bytes()).unwrap();
}