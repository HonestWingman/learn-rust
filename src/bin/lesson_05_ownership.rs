// lesson_05_ownership.rs
//
// 主题：所有权、移动、复制、克隆、借用、可变借用、切片。
// 运行：cargo run --bin lesson_05_ownership

fn main() {
    // String 拥有堆上的数据。赋值给另一个变量时，所有权会移动。
    let s1 = String::from("move happens");
    let s2 = s1;
    println!("s2 = {s2}");
    // println!("{s1}"); // 编译失败：s1 的所有权已经移动给 s2。

    // 实现 Copy 的类型会按位复制，赋值后两个变量都可用。
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // clone 会显式深拷贝堆数据。Rust 让昂贵操作在代码中更明显。
    let original = String::from("clone me");
    let cloned = original.clone();
    println!("original = {original}, cloned = {cloned}");

    // 函数参数也会发生所有权移动。
    let text = String::from("owned by main");
    let text = takes_and_returns_ownership(text);
    println!("text is back = {text}");

    // 借用 reference：只借数据，不取得所有权。
    let length = calculate_length(&text);
    println!("length = {length}, text still usable = {text}");

    // 同一时间可以有多个不可变借用。
    let r1 = &text;
    let r2 = &text;
    println!("immutable borrows: {r1}, {r2}");

    // 或者有一个可变借用。可变借用期间不能同时使用其他借用。
    let mut mutable_text = String::from("hello");
    append_suffix(&mut mutable_text);
    println!("mutable_text = {mutable_text}");

    // 字符串切片是对 String 一部分内容的借用。
    let sentence = String::from("Rust ownership model");
    let word = first_word(&sentence);
    println!("first word = {word}");

    // 借用规则避免悬垂引用。下面这种函数在 Rust 中不会被允许：
    // fn dangling() -> &String {
    //     let s = String::from("temporary");
    //     &s
    // }
}

fn takes_and_returns_ownership(value: String) -> String {
    println!("function owns value temporarily: {value}");
    value
}

fn calculate_length(value: &str) -> usize {
    value.len()
}

fn append_suffix(value: &mut String) {
    value.push_str(", Rust");
}

fn first_word(value: &str) -> &str {
    for (index, byte) in value.bytes().enumerate() {
        if byte == b' ' {
            return &value[..index];
        }
    }
    value
}
