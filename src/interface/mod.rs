use std::io::prelude::*;
use crate::database;
use crate::parser;
use std::net::TcpStream;

pub struct Handler<'a>{
    buffer: [u8; 512],
    stream: TcpStream,
    database: &'a mut database::KVdb,
}

pub enum CmdType{
    SyntaxError,
    Get(String),
    Set(String, String),
    Delete(String),
    Scan(String),

}

pub struct Command{
    pub cmd: CmdType,
}

impl<'a> Handler<'a>{
    pub fn new(db: &mut database::KVdb, IncomStream: TcpStream) -> Handler{
        Handler{
            buffer: [0; 512],
            stream: IncomStream,
            database: db,
        }
    }

    pub fn session(&mut self){
        // closure for write to stream
        let mut write_to_stream = |mut stream: &TcpStream, buf: &[u8]| {
            stream.write(buf).unwrap();
            stream.flush().unwrap();
        };

        // main session process loop
        loop{
            write_to_stream(&self.stream, b"> ");
            self.stream.read(&mut self.buffer).unwrap();
            let raw_content = String::from_utf8_lossy(&self.buffer[..]);
            // 不 等 了
            let instr: Command = parser::parse(raw_content.to_string());
            match instr.cmd{
                CmdType::SyntaxError => write_to_stream(&self.stream, b"syntax error!\n"),
                CmdType::Get(s) => {
                    match self.database.get(&s.to_string()){
                        database::CRUD_Result::KeyNotExist => write_to_stream(&self.stream, b"key does not exist!\n"),
                        database::CRUD_Result::ValueFinded(s2) => write_to_stream(&self.stream, format!("{}\n", s2).as_bytes()),
                        _ => ()
                    }
                },
                CmdType::Set(key, value) => {
                    match self.database.set(&key.to_string(), &value.to_string()){
                        database::CRUD_Result::ValueInserted => write_to_stream(&self.stream, b"write success!\n"),
                        database::CRUD_Result::ValueOverwritted(s) => write_to_stream(&self.stream, format!("key updated, old value: {}\n", s).as_bytes()),
                        _ => ()
                    }
                },
                CmdType::Delete(s) => {
                    match self.database.delete(&s.to_string()){
                        database::CRUD_Result::KeyNotExist => write_to_stream(&self.stream, b"key does not exist!\n"),
                        database::CRUD_Result::EntryDeleted(k, v) => write_to_stream(&self.stream, format!("entry deleted, key: {}, value: {}\n", k, v).as_bytes()),
                        _ => ()
                    }
                },
                CmdType::Scan(s) => {
                    match self.database.scan(&s.to_string()){
                        database::CRUD_Result::KeyNotExist => write_to_stream(&self.stream, b"key does not exist!\n"),
                        database::CRUD_Result::EntriesFinded(vec) => {
                            let res = String::new();
                            for (k, v) in vec{
                                format!("{}\nkey: {}, value: {}", res, k, v);
                            }
                            write_to_stream(&self.stream, format!("{}\n", res).as_bytes())
                        },
                        _ => ()
                    }
                },  
            }
        };
    }
}

#[test]
fn test_session_response(){
    // TODO: implement unit test for session_response
}