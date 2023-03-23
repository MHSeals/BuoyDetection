use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

pub struct WebServer {
    server: JoinHandle<()>,
    transmitter: Sender<String>,
}

impl WebServer {
    pub fn new() -> Self {
        let (transmitter, receiver) = mpsc::channel::<String>();
        let server = thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
            let mut last_message = String::from("");
            for stream in listener.incoming() {
                let server_message = receiver.try_iter();
                let message = match server_message.last() {
                    Some(msg) => {
                        last_message = format!("{}", msg.clone());
                        msg
                    }
                    None => last_message.clone(),
                };
                let stream = stream.unwrap();
                handle_connection(stream, message);
            }
        });
        return Self {
            server,
            transmitter,
        };
    }
    pub fn send_message(&self, message: String) {
        self.transmitter.send(message).unwrap();
    }
}

fn handle_connection(mut stream: TcpStream, message: String) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let status_line = "HTTP/1.1 200 OK";
    let len = message.len();
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{message}");
    stream.write_all(response.as_bytes()).unwrap();
}
