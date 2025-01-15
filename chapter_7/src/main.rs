#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{:?}", self);
    }
}

fn main() {
    let four = IpAddrKind::V4(1, 1, 1, 1);
    let six = IpAddrKind::V6;
    route(four);
    let m = Message::Write(String::from("hello"));
    m.call();
    let op = Option::Some(1);
    let op1 = Some('a');
    let x = 2;
    let sum = x + op.unwrap_or(0);
    println!("{sum}")
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}
