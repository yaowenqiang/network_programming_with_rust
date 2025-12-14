use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::Request;
use std::net::TcpListener;
#[derive(Debug)]
pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {
    
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
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

                   match stream.read(&mut buffer) {
                       Ok(_) => {
                           println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                           match Request::try_from(&buffer[..]) {
                               Ok(request) => {},
                               Err(e) => println!("Failed to parse a request: {}", e),
                           }
                           //let res :&Result<Request, _> = &buffer[..].try_into();
                       },
                       Err(e) => println!("Failed to read from conneciont: {}", e),
                   }
               },
               Err(e) => {
                   println!("Failed to establish  a connection: {:?}", e);
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


