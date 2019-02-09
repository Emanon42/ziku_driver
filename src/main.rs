use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;

mod parser;
mod interface;
mod database;


fn main() {
    let mut kvdb = database::KVdb::new();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();

        // thread::spawn(|| {
        //     let mut handler = interface::Handler::new(&mut kvdb, stream);
        //     handler.session();
        // });

        // since Rust does not allow multi mut reference in same time, it's single thread now.
        // TODO: Mutex lock and thread pool
        println!("connection established!");
        let mut handler = interface::Handler::new(&mut kvdb, stream);
        handler.session();
    }
}

