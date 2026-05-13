// lesson_12_traits.rs
//
// 主题：Trait、默认方法、trait bound、impl Trait、trait object、关联类型。
// 运行：cargo run --bin lesson_12_traits

use std::fmt;

trait Summary {
    fn author(&self) -> &str;

    fn summarize(&self) -> String {
        format!("read more from {}", self.author())
    }
}

#[derive(Debug)]
struct Article {
    author: String,
    title: String,
    body: String,
}

impl Summary for Article {
    fn author(&self) -> &str {
        &self.author
    }

    fn summarize(&self) -> String {
        format!(
            "{} by {} ({} chars)",
            self.title,
            self.author,
            self.body.len()
        )
    }
}

#[derive(Debug)]
struct ShortPost {
    author: String,
    text: String,
}

impl Summary for ShortPost {
    fn author(&self) -> &str {
        &self.author
    }
}

trait Printable: Summary + fmt::Debug {}

impl<T> Printable for T where T: Summary + fmt::Debug {}

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
}

trait Repository {
    type Item;

    fn get(&self, id: u32) -> Option<Self::Item>;
}

struct UserRepository {
    users: Vec<User>,
}

impl Repository for UserRepository {
    type Item = User;

    fn get(&self, id: u32) -> Option<Self::Item> {
        self.users.iter().find(|user| user.id == id).cloned()
    }
}

fn main() {
    let article = Article {
        author: String::from("Ferris"),
        title: String::from("Learning traits"),
        body: String::from("Traits define shared behavior."),
    };

    let post = ShortPost {
        author: String::from("Rustacean"),
        text: String::from("Traits can provide default methods."),
    };

    notify(&article);
    notify(&post);
    println!("short post text length = {}", post.text.len());

    let article_a = Article {
        author: String::from("A"),
        title: String::from("First"),
        body: String::from("body"),
    };
    let article_b = Article {
        author: String::from("B"),
        title: String::from("Second"),
        body: String::from("body"),
    };
    notify_pair(&article_a, &article_b);

    print_debug_summary(&article);

    // trait object 允许把不同具体类型放进同一个集合，代价是动态分发。
    let feed: Vec<Box<dyn Summary>> = vec![Box::new(article), Box::new(post)];
    for item in feed {
        println!("feed item: {}", item.summarize());
    }

    let repo = UserRepository {
        users: vec![
            User {
                id: 1,
                name: String::from("Alice"),
            },
            User {
                id: 2,
                name: String::from("Bob"),
            },
        ],
    };

    if let Some(user) = repo.get(2) {
        println!("repository found user: {user:?}");
        println!("user name = {}", user.name);
    }

    // 孤儿规则 orphan rule：只有当 trait 或类型至少有一个定义在当前 crate 中，
    // 才能为某类型实现某 trait。这样可以避免依赖之间出现冲突实现。
}

fn notify(item: &impl Summary) {
    println!("notification: {}", item.summarize());
}

fn notify_pair<T: Summary>(first: &T, second: &T) {
    println!("pair: {} | {}", first.summarize(), second.summarize());
}

fn print_debug_summary(item: &dyn Printable) {
    println!("debug = {item:?}");
    println!("summary = {}", item.summarize());
}
