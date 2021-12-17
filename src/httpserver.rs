use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
use ws::connect;
use ws::CloseCode;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    play_music();
}

fn play_music() {
    connect("ws://127.0.0.1:3012", |out| {
        out.send("play").unwrap();
        move |_| {
            out.close(CloseCode::Normal)
        }
    }).unwrap()
}