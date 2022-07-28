#![allow(dead_code, unused_variables)]

use self::back_of_house::Breakfast;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added!");
        }
    }
    fn lmao() {}
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

pub fn run() {
    // Absolute path, advised to use
    crate::c07_modules_crates::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    
    let mut summer_breakfast = Breakfast::summer("Rye");
    summer_breakfast.toast = String::from("Wheat");
    // Line below won't works as seasonal_fruit is not a public field in the struct
    // summer_breakfast.seasonal_fruit = ...
}

