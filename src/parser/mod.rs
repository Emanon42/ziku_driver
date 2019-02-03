use crate::interface;



pub fn parse(c: String) -> interface::Command{
    //TODO: implement this
    interface::Command{
        cmd: interface::CmdType::SyntaxError,
    }
}

#[test]
fn test_parser(){
    // TODO: implement unit test for parser
}