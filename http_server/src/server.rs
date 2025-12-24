use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {e}");
        Response::new(StatusCode::BadRequest, None)
    }
}

#[derive(Debug)]
pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&mut self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    //let a = [1,2,3,4, 5,7 ];
                    //arr(&a[1..3]);

                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
                            println!("Received {} bytes", bytes_read);
                            println!("Raw request: {:?}", &request_str[..request_str.chars().count().min(200)]);
                            let response = match Request::try_from(&buffer[..bytes_read]) {
                                Ok(request) => {
                                    //dbg!(request);
                                    //let response = Response::new(StatusCode::NotFound, None);
                                    handler.handle_request(&request)

                                    /*
                                    Response::new(
                                        StatusCode::OK,
                                        Some("<h1>It works!</h1>".to_string()),
                                    )
                                    */
                                }
                                Err(e) => {
                                    //println!("Failed to parse a request: {e}");
                                    //Response::new(StatusCode::BadRequest, None)

                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {e}"),
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {e:?}");
                    //Response::new(StatusCode::BadRequest, None).send(&mut stream);
                }
            }
            /*
            let res = listener.accept();

            if res.is_err() {
                continue;
            }

            let (stream, addr) = res.unwrap();
            */
        }
    }
}
