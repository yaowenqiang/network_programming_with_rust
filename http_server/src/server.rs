use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;
#[derive(Debug)]
pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&mut self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    //let a = [1,2,3,4, 5,7 ];
                    //arr(&a[1..3]);

                    let mut buffer = [0; 1024];

                    match stream.read_exact(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    //let response = Response::new(StatusCode::NotFound, None);
                                    let response = Response::new(StatusCode::OK, 
                                        Some("<h1>It works!</h1>".to_string()));
                                    //write!(stream, "HTTP/1.1 404 Not Found\r\n");
                                    write!(stream, "{}",response);
                                }
                                Err(e) => println!("Failed to parse a request: {e}"),
                            }
                            //let res :&Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connecion: {e}"),
                    }
                }
                Err(e) => {
                    println!("Failed to establish  a connection: {e:?}");
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
