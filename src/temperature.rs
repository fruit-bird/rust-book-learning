#![allow(dead_code, unused_variables)]
use std::io;

pub fn run() {
    println!("Temperature Converter!");
    println!("Convert from\n\t1. Celcius\n\t2. Fahrenheit");

    let mut temperature = String::new();
    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Choice Error");
    let choice = match choice.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => return,
    };

    io::stdin()
        .read_line(&mut temperature)
        .expect("Input Error");
    let temperature = match temperature.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => return,
    };

    if choice == 1 {
        println!("{}", celcius2fahrenheit(temperature));
    } else if choice == 2 {
        println!("{}", fahrenheit2celcius(temperature));
    }
}

pub fn celcius2fahrenheit(temp: f32) -> f32 {
    (temp * 9.0 / 5.0) + 32.0
}

pub fn fahrenheit2celcius(temp: f32) -> f32 {
    (temp * 5.0 / 9.0) - 32.0
}