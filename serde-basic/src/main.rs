#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    workers: u64,
    ignore: bool,
    auth_server: Option<String>
}

fn main() {
    let config = ServerConfig {
        workers:100,
        ignore: false,
        auth_server: Some("auth.server.io".to_string())
    };
    {
        println!("To and from YAML");
        let serializeed = serde_yaml::to_string(&config).unwrap();
        println!("{}", serializeed);

        let deserializeed: ServerConfig = serde_yaml::from_str(&serializeed).unwrap();
        println!("{:?}", deserializeed);
    }

    {
        println!("To and from JSON");
        let serializeed = serde_json::to_string(&config).unwrap();
        println!("{}", serializeed);

        let deserializeed: ServerConfig = serde_json::from_str(&serializeed).unwrap();
        println!("{:?}", deserializeed);
    }
}
