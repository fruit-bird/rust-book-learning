#![allow(dead_code)]
/*
“Convert strings to pig latin.
The first consonant of each word is moved to the end of the word and “ay” is added,
    so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead
    (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!”
 */

pub fn run() {
    let word1 = String::from("first");
    let word2 = String::from("apple");

    let ye_old_word1 = pig_latin(&word1);
    let ye_old_word2 = pig_latin(&word2);
    
    println!("{} {}", ye_old_word1, ye_old_word2);
}

fn pig_latin(word: &String) -> String {
    let starts_with_vowel = word.starts_with(['a', 'e', 'i', 'o', 'u']);

    let new_string = if starts_with_vowel {
        format!("{}", word.to_owned() + "-hay")
    } else {
        let first_letter = word.get(..1).unwrap().to_string();
        let without_first_letter = word.get(1..).unwrap().to_string();
        format!("{}", without_first_letter + "-" + &first_letter + "ay")
    };
    new_string
}
