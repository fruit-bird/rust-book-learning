#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

pub fn run() {
    // simple_construction();
    // construction_with_collect();
    // owned_types_moving();
    // accesssing_hashmap_values();
    // insert_if_key_has_no_value();
    // updating_based_on_the_old_value();
    magic_hashmap_creation();
}

fn simple_construction() -> HashMap<String, i32> {
    // Compiler was able to infer the types of the hashmap from the .insert() method
    let mut scores = HashMap::new();
    scores.insert(String::from("Purple"), 10);
    scores.insert(String::from("Yellow"), 20);
    scores
}

fn construction_with_collect() {
    let teams = vec![String::from("Purple"), String::from("Yellow")];
    let init_scores = vec![10, 20];

    // Compiler can infer types of the hashmap, so we use _ as a "placeholder"
    // We iterate through the keys in teams and we put them im a tuple with a value
    // from init_scores using .zip(), then we put them together in a HashMap collection
    // using .collect()
    let scores = teams.iter().zip(init_scores).collect::<HashMap<_, _>>();

    println!("{:#?}", scores);
}

fn owned_types_moving() {
    // String is an owned type, so when a string is given to a hashmap,
    // it will be moved and the hashmap will be its owner
    let field_name = String::from("Favorite Food");
    let field_value = String::from("Beans");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Attempting to use field_name or field_value will result in a borrow of moved value error
    // println!("{}: {}", field_name, field_value);
}

fn accesssing_hashmap_values() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Purple"), 10);
    scores.insert(String::from("Yellow"), 20);

    let color = String::from("Purple");
    // .get() method return an Option<&V>
    let score = scores.get(&color);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn insert_if_key_has_no_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Purple"), 10);

    // Will not insert 30 as value since key "Purple"'s value exists
    scores.entry(String::from("Purple")).or_insert(30);
    // Will insert 50 as value since key "Yellow"'s value doesn't exists
    scores.entry(String::from("Yellow")).or_insert(50);
}

fn updating_based_on_the_old_value() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        // println!("{}", count);
    }
    println!("{:?}", map);
}

fn magic_hashmap_creation() {
    let map = vec![(1, 5), (2, 6), (3, 7), (4, 8), (5, 9)]
        .into_iter()
        .collect::<HashMap<_, _>>();
    println!("{:#?}", map);
}
