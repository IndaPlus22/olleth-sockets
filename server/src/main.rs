
// use std::{
//     fs,
//     io::{prelude::*, BufReader},
//     net::{TcpListener, TcpStream},
// };

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         handle_connection(stream);
//     }
// }

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND", "404.html")
//     };

//     let contents = fs::read_to_string(filename).unwrap();
//     let length = contents.len();

//     let response =
//         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
// }

extern crate ws;

use std::fs;
use ws::{listen, Handler, Message, Request, Response, Result, Sender};

struct Server { // Server struct for implementing the websocket
    out: Sender,
}


impl Handler for Server { // Implement websocket handler
    // Handling requests and routes
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            // Implement the websocket route
            "/ws" => {
                println!("Received request to WebSocket route");
                Response::from_request(req)
            },

            // The main route where we will serve our html file
            "/" => Ok(
                Response::new(
                    200,
                    "OK",
                    fs::read_to_string("chat.html")
                    .expect("Something went wrong reading the file")
                    .as_bytes()
                    .to_vec()
                )
            ),

            // Handle invalid routes
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    // Handle messages recieved on /ws
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Broadcast the received msg to all clients
        self.out.broadcast(msg)
    }
}

fn main() {
    // Listen on 127.0.0.1 (localhost) at port 8000 and make a Server struct for each client that gets connected
    listen("127.0.0.1:8000", |out| Server { out }).unwrap()

}