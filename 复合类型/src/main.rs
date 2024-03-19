fn main() {
    println!("Hello, world!");

    // str_test();
    str2_test();
}


fn str_test() {
    // let s: str = "hello world";

    let st: &str = "hello world";

    greetings(st);
}


fn greetings(s: &str) {
    println!("Hello, {}!", s);
}

fn str2_test() {
    println!("demo 2 mut str");

    let mut s = String::from("hello world");
    s.push_str(" and hello rust");
    s.push_str("!");

    println!("{}", s);
    assert_eq!(s, "hello world and hello rust!");
}