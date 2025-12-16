#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use http::Method;
use http::Request;
use server::Server;

mod server;
mod http;

fn main() {
    /*
    let string = String::from("127.0.0.1:8080");
    //let string_slice = &string[10..14];
    //let string_slice = &string[0..3];
    //
    //

    let string_slice = &string[10..];
    dbg!(&string);
    dbg!(string_slice);

    let string_borrow: &str = &string;
    dbg!(string_borrow);

    let string_literal = "1234";
    dbg!(string_literal);
    */

    //let get = Method::GET;
    //let delete = Method::DELETE;
    //let post = Method::POST;
    //let put = Method::PUT;
    
    let mut server = Server::new("127.0.0.1:8180".to_string());
    server.run();
}
