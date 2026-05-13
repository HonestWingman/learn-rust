// lesson_08_collections.rs
//
// 主题：常见集合 Vec、String、HashMap、HashSet、VecDeque 及常用操作。
// 运行：cargo run --bin lesson_08_collections

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    // Vec<T>：连续内存中的可增长数组。
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("numbers = {numbers:?}");

    // vec! 宏可以快速创建 Vec。
    let mut scores = vec![90, 80, 70];
    scores[1] = 85;
    println!("scores = {scores:?}");

    // 安全读取推荐 get；直接下标越界会 panic。
    if let Some(score) = scores.get(1) {
        println!("second score = {score}");
    }

    for score in &scores {
        println!("borrowed score = {score}");
    }

    for score in &mut scores {
        *score += 1;
    }
    println!("after bonus = {scores:?}");

    // String：UTF-8 字符串集合，底层类似 Vec<u8>。
    let mut message = String::from("Rust");
    message.push(' ');
    message.push_str("collections");
    println!("message = {message}");

    let prefix = &message[..4];
    println!("prefix slice = {prefix}");

    for ch in "数据".chars() {
        println!("unicode char = {ch}");
    }

    // HashMap<K, V>：键值映射。
    let mut points = HashMap::new();
    points.insert(String::from("Alice"), 10);
    points.insert(String::from("Bob"), 20);
    points
        .entry(String::from("Alice"))
        .and_modify(|value| *value += 5);
    points.entry(String::from("Carol")).or_insert(30);
    println!("points = {points:?}");

    let alice_score = points.get("Alice").copied().unwrap_or(0);
    println!("Alice score = {alice_score}");

    // 统计词频是 HashMap 的典型用法。
    let words = ["rust", "cargo", "rust", "borrow", "cargo", "rust"];
    let mut counts = HashMap::new();
    for word in words {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("word counts = {counts:?}");

    // HashSet<T>：只关心元素是否存在，不关心顺序和重复。
    let mut tags = HashSet::new();
    tags.insert("rust");
    tags.insert("systems");
    tags.insert("rust");
    println!("tags = {tags:?}, contains rust = {}", tags.contains("rust"));

    // VecDeque<T>：双端队列，适合从两端高效 push/pop。
    let mut queue = VecDeque::new();
    queue.push_back("task-1");
    queue.push_back("task-2");
    queue.push_front("urgent");
    while let Some(task) = queue.pop_front() {
        println!("processing {task}");
    }
}
