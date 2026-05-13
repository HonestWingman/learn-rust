// lesson_10_error_handling.rs
//
// 主题：panic、Result、Option、? 运算符、自定义错误类型。
// 运行：cargo run --bin lesson_10_error_handling

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

#[derive(Debug)]
enum AppError {
    InvalidFormat,
    EmptyName,
    ParseAge(ParseIntError),
    TooYoung(u8),
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidFormat => write!(formatter, "expected format name:age"),
            AppError::EmptyName => write!(formatter, "name cannot be empty"),
            AppError::ParseAge(error) => write!(formatter, "invalid age: {error}"),
            AppError::TooYoung(age) => write!(formatter, "age {age} is younger than 18"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::ParseAge(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::ParseAge(error)
    }
}

fn main() -> Result<(), AppError> {
    // panic! 表示不可恢复错误，会让当前线程崩溃。业务错误通常不要用 panic。
    // panic!("something went terribly wrong");

    for raw in ["Alice:20", "NoSeparator", "Tom:17", ":30", "Bob:not-number"] {
        match parse_user(raw) {
            Ok(user) => println!("valid user: {user:?}"),
            Err(error) => println!("input {raw:?} failed: {error}"),
        }
    }

    // ? 会在 Err 时提前返回，并自动调用 From 做错误类型转换。
    let user = parse_user("Carol:28")?;
    println!("question mark parsed user = {user:?}");
    println!("{} is {} years old", user.name, user.age);

    // Option 可以用 ok_or / ok_or_else 转成 Result。
    let config = Some("debug");
    let mode = read_config(config)?;
    println!("mode = {mode}");

    Ok(())
}

fn parse_user(input: &str) -> Result<User, AppError> {
    let (name, age_text) = input.split_once(':').ok_or(AppError::InvalidFormat)?;

    if name.trim().is_empty() {
        return Err(AppError::EmptyName);
    }

    let age: u8 = age_text.parse()?;
    if age < 18 {
        return Err(AppError::TooYoung(age));
    }

    Ok(User {
        name: String::from(name),
        age,
    })
}

fn read_config(value: Option<&str>) -> Result<&str, AppError> {
    value.ok_or(AppError::InvalidFormat)
}
