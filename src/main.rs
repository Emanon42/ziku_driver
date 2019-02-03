use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;

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

    // old fast prototype code
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 
    // for stream in listener.incoming(){
    //     let stream = stream.unwrap();
    //     // currently no limit for thread amount. will implement thread pool.
    //     thread::spawn(|| {
    //         handle_connection(stream);
    //     });
    //     println!("connection established!")
    // }
}


// old fast prototype code
// fn handle_connection(mut stream: TcpStream){
//     let mut db: HashMap<String, String> = HashMap::new();
//     let write_to_stream = |mut stream: &TcpStream, buf: &[u8]| {
//         stream.write(buf).unwrap();
//         stream.flush().unwrap();
//     };
//     let exit = b"exit";
//     loop{
//         // possible solution: declare buffer outside the loop and bitwise AND 0 for this array.
//         let mut buffer = [0; 512]; // potential high memory operation cost: Rust would free buffer and re-allocate in every loop.
//         write_to_stream(&stream, b"> ");
//         stream.read(&mut buffer).unwrap();
//         let content = String::from_utf8_lossy(&buffer[..]);
//         println!("Request content: {:?}", content); 
//         let cmd_vec:Vec<&str> = content.split(' ').collect();
//         assert_eq!(cmd_vec.len(), 3);
//         //println!("{}", cmd_vec[0]);
//         // if cmd_vec[0] == "set"{
//         //     db.insert(String::from(cmd_vec[1]), String::from(cmd_vec[2]));
//         // }
//         match cmd_vec[0]{
//                 "set" => {
//                     db.insert(String::from(cmd_vec[1]), String::from(cmd_vec[2]));
//                     for (key, value) in &db {
//                         println!("{:?}: {:?}", key, value);
//                     }
//                     write_to_stream(&stream, b"write done\n");
//                 },
//                 "get" => {
//                     let k:String = String::from(cmd_vec[1]);
//                     //println!("{:?}", cmd_vec[1]);
//                     let v = match db.get(&k){
//                         None => "KEY_NOT_EXIST\n",
//                         Some(i) => i,
//                     };
//                     write_to_stream(&stream, format!("{}\n", v).as_bytes());
//                 },
//                 "exit" => {
//                     break;
//                 },
//                 _ => (),
//             }
//         }
// }

