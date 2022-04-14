use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main() {
    let stdouts = stdout();
    let message = String::from("hello world");

    let width = message.chars().count();

    // 方法引用
    let mut writer = BufWriter::new(stdouts.lock());

    say(message.as_bytes(), width, &mut writer);

    println!("Hello, world!");
}
