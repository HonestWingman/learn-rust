// lesson_03_functions.rs
//
// 主题：函数、参数、返回值、表达式、语句、闭包和函数指针。
// 运行：cargo run --bin lesson_03_functions

fn main() {
    greet("Rust learner");

    let sum = add(20, 22);
    println!("20 + 22 = {sum}");

    print_labelled_measurement(5, 'm');

    // 代码块也是表达式。最后一行没有分号时，它就是代码块的返回值。
    let squared = {
        let x = 8;
        x * x
    };
    println!("squared = {squared}");

    // 返回 Option 可以表达“可能没有值”，比用特殊数字更清楚。
    match divide(10.0, 2.0) {
        Some(value) => println!("10 / 2 = {value}"),
        None => println!("cannot divide by zero"),
    }

    // 闭包 closure：轻量的匿名函数，常用于迭代器、回调、临时转换。
    let add_one = |n: i32| n + 1;
    println!("add_one(41) = {}", add_one(41));

    // 普通函数也可以作为函数指针传递。
    let result = apply_twice(3, double);
    println!("double twice from 3 = {result}");
}

fn greet(name: &str) {
    println!("hello, {name}");
}

fn add(left: i32, right: i32) -> i32 {
    // 表达式结尾不写分号，表示返回该值。
    left + right
}

fn print_labelled_measurement(value: i32, unit: char) {
    // 没有显式返回类型时，函数返回 unit 类型：()
    println!("measurement: {value}{unit}");
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn double(n: i32) -> i32 {
    n * 2
}

fn apply_twice(value: i32, f: fn(i32) -> i32) -> i32 {
    f(f(value))
}
