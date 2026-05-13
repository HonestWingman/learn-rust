// lesson_06_structs.rs
//
// 主题：结构体、字段、构造、更新语法、元组结构体、单元结构体、方法。
// 运行：cargo run --bin lesson_06_structs

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // 关联函数 associated function：不接收 self，常用于构造器。
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = build_user("someone@example.com", "someusername123");
    println!("user1 = {user1:?}");
    println!(
        "user1 fields: active={}, username={}, email={}, sign_in_count={}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    // 结构体更新语法会移动未显式指定的非 Copy 字段。
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user2 = {user2:?}");

    // 元组结构体适合给多个同类型或简单组合的值一个明确领域名称。
    let black = Color(0, 0, 0);
    println!("black rgb = ({}, {}, {})", black.0, black.1, black.2);

    // 单元结构体没有字段，常用于只需要类型身份的场景。
    let marker = AlwaysEqual;
    println!("marker = {marker:?}");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let square = Rectangle::square(25);

    println!("rect1 area = {}", rect1.area());
    println!("rect1 can hold rect2 = {}", rect1.can_hold(&rect2));
    println!("square = {square:?}, area = {}", square.area());
}

fn build_user(email: &str, username: &str) -> User {
    User {
        active: true,
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 1,
    }
}
