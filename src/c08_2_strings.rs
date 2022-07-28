#![allow(dead_code, unused_variables)]

pub fn run() {
    // concatenation_with_plus();
    // concatenation_with_format_macro();
    string_slices_into_utf8();
}

fn concatenation_with_plus() {
    let s1 = String::from("Wooooooooo, yeaaaaah baby!! ");
    let s2 = String::from("That's what I've been waiting for that's what it's all about!\n");
    let s4 = String::from("Wooooooooooooooooooo!!!".to_uppercase());
    // s1 has been moved here, so it is no longer accessible
    // We can only add an str to a String, we can't add two Strings together
    // The compiler "converts" &s2: &String into &str (deref coersion)
    let s3 = s1 + &s2 + &s4;
    println!("{}", s3);
}

fn concatenation_with_format_macro() {
    let s1 = String::from("Wooooooooo, yeaaaaah baby!!");
    let s2 = String::from("That's what I've been waiting for that's what it's all about!");
    let s4 = String::from("Wooooooooooooooooooo!!!".to_uppercase());
    // The format! macro doesn't take ownership of any value and is easier to read
    let s3 = format!("{} {}\n{}", s1, s2, s4);
    println!("{}", s3);
    println!("s1:\t {}", s1);
}

fn string_slices_into_utf8() {
    let s = String::from("ϣhahaha");
    let slice = &s[0..3];
    println!("{slice}");
}
