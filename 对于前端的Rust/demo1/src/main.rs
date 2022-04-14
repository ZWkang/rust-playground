// part1
// pub struct Rectangle {
//     height: usize,
//     width: usize,
// }

// impl Rectangle {
//     pub fn new(height: usize, width: usize) -> Self {
//         Self { height, width }
//     }

//     pub fn height(&self) -> usize {
//         self.height
//     }
//     pub fn width(&self) -> usize {
//         self.width
//     }

//     pub fn setHeight(&mut self, height: usize) -> usize {
//         self.height = height;
//         self.height
//     }

//     pub fn part(self) -> (usize, usize){
//         (self.height, self.width)
//     }
// }

// part2
// #[derive(Debug)]
// struct User {
//     name: String,
//     age: Option<u32>
// }

// part3
fn todo(use_case_str: &str) {
    println!("useString {}", use_case_str)
}

fn panic(use_case_str: &str) {
    println!("useString {}", use_case_str)
}

use std::fs;
use async_std::fs;
// fn read_readme_file(path: &str) {
//     match fs::read_to_string(&str) {
//         Ok(file) => println!("read {} chars", file.len()),
//         Err(e) => println!("{}", e)
//     }
// }
#[async_std::main]
async fn main() {

    // part1
    // println!("Hello, world!");/

    // let angle = new Rectangle(123, 123);


    // part2
    // let mut user = User {
    //     name: "zwkang".to_string(),
    //     age: None
    // };

    // println!("user is {:?}", user);

    // user.age = Some(123);

    // println!("user is {:?}", user);


    // let num = 1;
    // match num {
    //     0 => todo!("handle case 0"),
    //     1 => todo!("handle case 1"),
    //     _ => panic!("oops")
    // }

    // read_readme_file("./README.md");

    // sync read
    // match fs::read_to_string("./src/README.md") {
    //     Ok(file) => println!("read {} chars", file),
    //     Err(e) => eprintln!("{}", e), // 处理错误
    // };
    // async read
    match fs::read_to_string("./src/README.md").await {
        Ok(file) => println!("read {} chars", file.len()),
        Err(e) => eprintln!("{}", e),
    };
}
