/*
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}
*/

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

fn main() {

    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let messageInstance = Message::Write(String::from("This is a message"));
    messageInstance.call();
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);
}

// fn route(ip_kind: IpAddrKind) {
//    println!("{:#?}", ip_kind);
// }
