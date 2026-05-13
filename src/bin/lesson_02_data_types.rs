// lesson_02_data_types.rs
//
// 主题：Rust 的基础数据类型，包括标量、复合类型、字符串切片和 String。
// 运行：cargo run --bin lesson_02_data_types

fn main() {
    // 整数类型：
    // 有符号：i8 i16 i32 i64 i128 isize
    // 无符号：u8 u16 u32 u64 u128 usize
    let signed: i32 = -42;
    let unsigned: u32 = 42;
    let pointer_sized: usize = 3;
    println!("signed = {signed}, unsigned = {unsigned}, usize = {pointer_sized}");

    // 整数字面量可以用下划线分隔，也支持不同进制。
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("decimal={decimal}, hex={hex}, octal={octal}, binary={binary}, byte={byte}");

    // 浮点数默认是 f64，也可以显式使用 f32。
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x = {x}, y = {y}");

    // 布尔值只有 true 和 false。
    let is_rust_fun = true;
    let is_zero = 0 == 1;
    println!("is_rust_fun = {is_rust_fun}, is_zero = {is_zero}");

    // char 是 Unicode 标量值，不只是 ASCII 字符。
    let letter = 'R';
    let chinese = '语';
    let emoji = '🦀';
    println!("chars: {letter}, {chinese}, {emoji}");

    // 元组 tuple：长度固定，每个位置可以有不同类型。
    let tuple: (i32, f64, char) = (500, 6.4, 'z');
    let (a, b, c) = tuple;
    println!("destructured tuple = {a}, {b}, {c}");
    println!("tuple fields = {}, {}, {}", tuple.0, tuple.1, tuple.2);

    // 数组 array：长度固定，每个元素类型相同。
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let repeated = [0; 4];
    println!("numbers = {numbers:?}, repeated = {repeated:?}");

    // 直接下标越界会 panic。安全读取可以用 get，返回 Option。
    match numbers.get(2) {
        Some(value) => println!("numbers[2] = {value}"),
        None => println!("index out of range"),
    }

    // 切片 slice：引用集合中的连续一段，不拥有数据。
    let middle: &[i32] = &numbers[1..4];
    println!("middle slice = {middle:?}");

    // &str 是字符串切片，通常指向程序中的字符串字面量或 String 的一部分。
    let borrowed: &str = "hello Rust";
    println!("borrowed str = {borrowed}");

    // String 是堆上可增长、拥有所有权的字符串。
    let mut owned = String::from("hello");
    owned.push(' ');
    owned.push_str("Rust");
    println!("owned String = {owned}");

    // Rust 字符串是 UTF-8，不能按字节下标直接取字符。
    // 如果需要遍历字符，用 chars；如果需要字节，用 bytes。
    for ch in owned.chars() {
        print!("[{ch}]");
    }
    println!();
}
