#[derive(Debug)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Do something with the method
        println!("{:?}", self);
    }
}

fn plus_one(x: usize) -> Option<usize> {
    match Some(x) {
        Some(i) => Some( i + 1),
        None => None,
    }   
}
fn main() {
    println!("Hello, world!");
    let ipv4 = IPAddr::V4(127,0,0,1);
    let ipv6 = IPAddr::V6(String::from("::1"));
    println!("{:?}", ipv4);
    println!("{:?}", ipv6);
    let message = Message::Write(String::from("hello"));
    message.call();
    println!("{:?}", plus_one(40_usize).unwrap());

    if let Some(5) = plus_one(3) {
        println!("five");
    }
    else {
        println!("Not five!");
    }
}
