#![allow(dead_code, unused_variables)]

pub fn fibonacci(n: i8) -> isize {
    if n == 0 || n == 1 {
        n as isize
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

pub fn run(n: i8) {
    print!("{} ", fibonacci(n));
}