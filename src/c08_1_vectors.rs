#![allow(dead_code, unused_variables)]

pub fn run() {
    indexing_vec_with_get(true);
    indexing_vec_with_get(false);
    dereferencing_vec();
    vec_with_multiple_types();
}

fn indexing_vec_with_get(index_exists: bool) {
    let v = if index_exists {
        vec![1, 2, 3, 4, 4]
    } else {
        Vec::new()
    };

    // v.get() method returns an Option<&T>
    // v.get() takes an index as an argument
    match v.get(2) {
        Some(x) => println!("The third value is {}\n", x),
        None => println!("There is no third element\n"),
    }
}

fn iterating_over_vec() {
    println!("Iterating over a vec with a for loop");
    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        print!("{}, ", i);
    }
}

fn dereferencing_vec() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("Before dereferencing\n{:?}\n", v);

    for i in &mut v {
        *i *= 10;
    }
    println!("After dereferencing\n{:?}", v);
}

// Vecs store values of a single type.
// In order to create vecs of different types, enums can store different types
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vec_with_multiple_types() {
    let multi_vector = vec![
        SpreadsheetCell::Text(String::from("Silly Billy")),
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(9.0),
    ];
    println!("{:?}", multi_vector);
}