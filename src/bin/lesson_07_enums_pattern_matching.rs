// lesson_07_enums_pattern_matching.rs
//
// 主题：枚举、Option、Result、match、if let、while let、模式匹配。
// 运行：cargo run --bin lesson_07_enums_pattern_matching

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("addresses: {localhost:?}, {loopback:?}");

    println!("{}", describe_ip(localhost));
    println!("{}", describe_ip(loopback));

    let messages = [
        Message::Quit,
        Message::Move { x: 0, y: 0 },
        Message::Move { x: 10, y: -3 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 64, 0),
    ];

    for message in messages {
        println!("{}", handle_message(message));
    }

    // Option<T> 表达“有值或无值”，避免空指针。
    let maybe_number = Some(7);
    match maybe_number {
        Some(n @ 1..=10) => println!("number in range 1..=10: {n}"),
        Some(n) => println!("number outside range: {n}"),
        None => println!("no number"),
    }

    // if let 适合只关心一种匹配情况。
    let maybe_name = Some("Rust");
    if let Some(name) = maybe_name {
        println!("name = {name}");
    }

    // while let 适合反复匹配，直到模式不再成立。
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped = {top}");
    }

    // Result<T, E> 表达成功或失败。
    let parsed: Result<i32, _> = "42".parse();
    match parsed {
        Ok(value) => println!("parsed value = {value}"),
        Err(error) => println!("parse error = {error}"),
    }
}

fn describe_ip(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(a, b, c, d) => format!("IPv4 address: {a}.{b}.{c}.{d}"),
        IpAddr::V6(address) => format!("IPv6 address: {address}"),
    }
}

fn handle_message(message: Message) -> String {
    match message {
        Message::Quit => String::from("quit"),
        Message::Move { x: 0, y: 0 } => String::from("move to origin"),
        Message::Move { x, y } if x == y => format!("move diagonally to ({x}, {y})"),
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        Message::Write(text) => format!("write text: {text}"),
        Message::ChangeColor(red, green, blue) => format!("rgb({red}, {green}, {blue})"),
    }
}
