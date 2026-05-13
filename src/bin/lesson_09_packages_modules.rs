// lesson_09_packages_modules.rs
//
// 主题：包 package、crate、模块 module、可见性 pub、use、重导出。
// 运行：cargo run --bin lesson_09_packages_modules
//
// 概念速记：
// - package：包含 Cargo.toml 的项目，一个 package 可以有多个 crate。
// - crate：Rust 编译单元。src/main.rs 是二进制 crate，src/lib.rs 是库 crate。
// - module：crate 内部的命名空间，用 mod 定义，用来组织代码和控制可见性。
// - src/bin/*.rs 中每个文件都是一个独立二进制 crate。

mod garden {
    pub mod vegetables {
        #[derive(Debug)]
        pub struct Asparagus {
            pub name: String,
            kind: Kind,
        }

        #[derive(Debug)]
        enum Kind {
            Green,
            White,
        }

        impl Asparagus {
            pub fn green(name: &str) -> Self {
                Self {
                    name: String::from(name),
                    kind: Kind::Green,
                }
            }

            pub fn white(name: &str) -> Self {
                Self {
                    name: String::from(name),
                    kind: Kind::White,
                }
            }

            pub fn describe(&self) -> String {
                format!("{} is {:?}", self.name, self.kind)
            }
        }
    }

    pub(crate) mod tools {
        pub fn water() -> &'static str {
            "watering can"
        }
    }

    fn private_helper() -> &'static str {
        "private garden helper"
    }

    pub fn garden_note() -> String {
        format!("garden uses {}", private_helper())
    }
}

mod kitchen {
    use crate::garden::vegetables::Asparagus;

    pub fn cook(item: &Asparagus) -> String {
        format!("cook {}", item.name)
    }
}

// use 可以把长路径引入当前作用域，也可以起别名。
use garden::vegetables::Asparagus as Veg;

// pub use 可以重导出，让外部使用更短路径访问某个项。
pub use garden::garden_note;

fn main() {
    let green = Veg::green("Mary Washington");
    let white = garden::vegetables::Asparagus::white("Gijnlim");

    println!("{}", green.describe());
    println!("{}", white.describe());
    println!("{}", kitchen::cook(&green));
    println!("tool = {}", garden::tools::water());
    println!("{}", garden_note());

    // green.kind 是私有字段，模块外不能访问。
    // println!("{:?}", green.kind);
}
