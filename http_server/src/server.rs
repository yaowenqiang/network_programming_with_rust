use std::net::TcpListener;
#[derive(Debug)]
pub struct Server {
    addr: String,
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
               Ok((stream, addr)) => {
                   let a = 5;
                   print!("Ok");
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


