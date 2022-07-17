#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    sign_in_count: u32,
    active: bool,
}

pub fn run() {
    let user1 = User {
        email: String::from("abc@xyz.com"),
        username: String::from("xxXUltraGamerXxx"),
        sign_in_count: 1,
        active: true,
    };
    let user2 = User {
        email: String::from("lmao@lol.com"),
        username: user1.username,
        ..user1
    };
    println!("{:?}", user2);
    let mut user3 = User::new(
        "poopshitter@hotmail.com".to_string(),
        "disocrd_super_mod".to_string(),
    );
    user3.username_change(String::from("discord_ultra_mod"));
    println!("{:#?}", user3);

    assert_eq!(user2.sign_in_count, user3.sign_in_count);
    assert_eq!(user1.active, user3.active);
}

impl User {
    fn new(email: String, username: String) -> Self {
        User {
            email,
            username,
            sign_in_count: 1,
            active: true,
        }
    }
    fn username_change(&mut self, new_username: String) {
        self.username = new_username;
    }
}
