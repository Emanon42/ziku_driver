extern crate regex;
use crate::interface;
use regex::Regex;




pub fn parse(c: String) -> interface::Command{
    //let purifier = Regex::new(r"[a-zA-Z0-9\s]").unwrap();
    let give_error = | | -> interface::Command{
        interface::Command{
            cmd: interface::CmdType::SyntaxError,
        }
    };
    let after = c.replace("\u{0}", "").replace("\r\n", "");
    let tokens_vec: Vec<&str> = after.split(' ').collect();
    //println!("{:?}", after);
    let instr = match tokens_vec[0]{
        "set" => {
            if tokens_vec.len() < 3 {give_error()} else {
                interface::Command{
                    cmd: interface::CmdType::Set(tokens_vec[1].to_string(), tokens_vec[2].to_string()),
                }
            }
        },
        "get" => {
            if tokens_vec.len() < 2 {give_error()} else {
                interface::Command{
                    cmd: interface::CmdType::Get(tokens_vec[1].to_string()),
                }
            }
        },
        "delete" => {
            if tokens_vec.len() < 2 {give_error()} else {
                interface::Command{
                    cmd: interface::CmdType::Delete(tokens_vec[1].to_string()),
                }
            }
        },
        "scan" => {
            if tokens_vec.len() < 2 {give_error()} else {
                interface::Command{
                    cmd: interface::CmdType::Scan(tokens_vec[1].to_string()),
                }
            }
        },
        _ => {
            give_error()
        },
    };
    instr
}

#[test]
fn test_parser(){
    // TODO: implement unit test for parser
}