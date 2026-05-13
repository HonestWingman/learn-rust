// lesson_11_generics.rs
//
// 主题：泛型函数、泛型结构体、泛型枚举、trait bound、const generics。
// 运行：cargo run --bin lesson_11_generics

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Pair<T, U> {
    left: T,
    right: U,
}

#[derive(Debug)]
enum ApiResponse<T, E> {
    Data(T),
    Error(E),
}

fn main() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 3.0, y: 4.0 };
    println!("int_point.x = {}", int_point.x());
    println!("float distance = {}", float_point.distance_from_origin());

    // 如果两个字段可能是不同类型，就使用多个泛型参数。
    let pair = Pair {
        left: "name",
        right: 42,
    };
    println!("pair = {pair:?}");
    println!("pair fields: left={}, right={}", pair.left, pair.right);

    let numbers = vec![3, 8, 2, 10, 4];
    println!("largest copy = {:?}", largest_copy(&numbers));
    println!("largest ref = {:?}", largest_ref(&numbers));

    let words = vec![String::from("rust"), String::from("cargo")];
    if let Some(word) = largest_ref(&words) {
        println!("largest word by lexicographic order = {word}");
    }

    let ok: ApiResponse<&str, &str> = ApiResponse::Data("loaded");
    let err: ApiResponse<&str, &str> = ApiResponse::Error("timeout");
    println!("responses = {ok:?}, {err:?}");
    println!("describe ok = {}", describe_response(&ok));
    println!("describe err = {}", describe_response(&err));

    // const generics：把数组长度这类常量作为泛型参数。
    let repeated = repeat_value::<i32, 4>(9);
    println!("repeated = {repeated:?}");
}

fn largest_copy<T>(items: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    let mut largest = *items.first()?;
    for &item in items.iter().skip(1) {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

fn largest_ref<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut largest = items.first()?;
    for item in items.iter().skip(1) {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

fn repeat_value<T: Copy, const N: usize>(value: T) -> [T; N] {
    [value; N]
}

fn describe_response<T: std::fmt::Debug, E: std::fmt::Debug>(
    response: &ApiResponse<T, E>,
) -> String {
    match response {
        ApiResponse::Data(value) => format!("data: {value:?}"),
        ApiResponse::Error(error) => format!("error: {error:?}"),
    }
}
