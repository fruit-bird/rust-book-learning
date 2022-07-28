#![allow(dead_code, unused_variables)]
pub fn run() {
    // let s = "hello";
    // println!("{}", s);

    // let mut s = String::from("Hello");
    // s.push_str(", Worl");
    // s.push('d');

    // println!("{}", s);

    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    let a = String::from("Hello");
    // a is out of scope now and cannot be used as it is "moved" into b
    let b = a;
}