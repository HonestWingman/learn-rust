// lesson_13_lifetimes.rs
//
// 主题：生命周期、生命周期标注、结构体中的引用、生命周期省略、'static。
// 运行：cargo run --bin lesson_13_lifetimes

#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn level(&self) -> usize {
        1
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("announcement: {announcement}");
        self.part
    }
}

fn main() {
    let left = String::from("short");
    let right = String::from("much longer");
    let result = longest(left.as_str(), right.as_str());
    println!("longest = {result}");

    // 生命周期标注不会延长引用的生命，只是描述多个引用之间的关系。
    // 下面这种写法不会通过编译，因为 temp 离开作用域后 result 会悬垂：
    // let result;
    // {
    //     let temp = String::from("temporary");
    //     result = longest(left.as_str(), temp.as_str());
    // }
    // println!("{result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("there should be a sentence");
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!("excerpt = {excerpt:?}, level = {}", excerpt.level());
    println!(
        "returned part = {}",
        excerpt.announce_and_return_part("checking excerpt")
    );

    // 生命周期省略规则让很多函数不需要显式写生命周期。
    let first = first_word("lifetime elision rules");
    println!("first word = {first}");

    // 字符串字面量拥有 'static 生命周期，整个程序运行期间都有效。
    let message: &'static str = static_message();
    println!("static message = {message}");
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

// 这个函数没有显式生命周期，因为输入只有一个引用，输出引用自然来自这个输入。
fn first_word(value: &str) -> &str {
    value.split_whitespace().next().unwrap_or("")
}

fn static_message() -> &'static str {
    "I live for the entire program"
}
