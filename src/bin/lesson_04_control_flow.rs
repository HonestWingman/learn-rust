// lesson_04_control_flow.rs
//
// 主题：if、loop、while、for、match、let-else 等流程控制。
// 运行：cargo run --bin lesson_04_control_flow

fn main() {
    let number = 7;

    if number < 5 {
        println!("number is smaller than 5");
    } else if number == 5 {
        println!("number equals 5");
    } else {
        println!("number is greater than 5");
    }

    // if 是表达式，因此可以把结果赋给变量。
    let size = if number % 2 == 0 { "even" } else { "odd" };
    println!("{number} is {size}");

    // loop 是无限循环，可以用 break 跳出。
    // break 后面也可以带值，让 loop 成为表达式。
    let mut counter = 0;
    let ten = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("loop returned {ten}");

    // while 适合条件循环。
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}!");
        countdown -= 1;
    }

    // for 常用于遍历集合或范围，避免手写下标。
    let names = ["Ada", "Grace", "Linus"];
    for name in names {
        println!("name = {name}");
    }

    for n in (1..=3).rev() {
        println!("range item = {n}");
    }

    // match 必须穷尽所有可能。下划线 _ 表示“其他情况”。
    let command = "start";
    match command {
        "start" => println!("starting"),
        "stop" => println!("stopping"),
        other => println!("unknown command: {other}"),
    }

    // let-else 适合从 Option/Result 等类型中提前取值，否则提前返回或退出。
    let maybe_port = Some(8080);
    let Some(port) = maybe_port else {
        println!("missing port");
        return;
    };
    println!("server port = {port}");
}
