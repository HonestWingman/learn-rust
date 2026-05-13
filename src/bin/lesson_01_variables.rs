// lesson_01_variables.rs
//
// 主题：变量、可变性、常量、遮蔽、作用域。
// 运行：cargo run --bin lesson_01_variables

const MAX_POINTS: u32 = 100_000;
static APP_NAME: &str = "rust_lessons";

fn main() {
    // Rust 中变量默认不可变。不可变变量让代码更容易推理，也能避免意外修改。
    let answer = 42;
    println!("answer = {answer}");

    // 如果确实需要修改变量，使用 mut 显式标记。
    let mut counter = 0;
    counter += 1;
    counter += 1;
    println!("counter = {counter}");

    // 类型通常可以由编译器推断，也可以显式标注。
    let inferred_integer = 10;
    let annotated_integer: i64 = 10;
    println!("inferred = {inferred_integer}, annotated = {annotated_integer}");

    // 遮蔽 shadowing：可以用同名 let 重新绑定变量。
    // 这不是修改原变量，而是创建了一个新变量，因此可以改变类型。
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces length = {spaces}");

    // 遮蔽也常用于分步转换，让每一步都保持不可变。
    let score = 60;
    let score = score + 10;
    let score = format!("final score: {score}");
    println!("{score}");

    // 作用域：变量只在声明它的代码块内有效。
    {
        let local_name = "inside block";
        println!("local_name = {local_name}");
    }
    // println!("{local_name}"); // 这里会编译失败，因为 local_name 已离开作用域。

    // const 必须显式写类型，并且只能绑定编译期常量表达式。
    println!("MAX_POINTS = {MAX_POINTS}");

    // static 表示整个程序生命周期都存在的全局值。
    // 大多数情况下优先使用 const；需要固定内存地址或全局共享状态时才考虑 static。
    println!("APP_NAME = {APP_NAME}");
}
