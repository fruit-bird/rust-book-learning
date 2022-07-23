#![allow(dead_code)]
/*
“Using a hash map and vectors,
create a text interface to allow a user to add employee na&mes to a department in a company.

For example, “Add Sally to Engineering” or “Add Amir to Sales.”

Then let the user retrieve a list of all people in a department or all people in the company by department,
sorted alphabetically.”
*/

use std::{collections::HashMap, io};

pub fn run() {
    let mut database: HashMap<String, String> = HashMap::new();

    let input = return_input();
    // println!("{}", input);
    text_to_query(&mut database, input);
    text_to_query(&mut database, String::from("Add Amir to Sales"));
    text_to_query(&mut database, String::from("Add Joe to Sales"));
    text_to_query(&mut database, String::from("Add Bob to IT"));

    // select_query(&database, None);
    select_query(&database, Some("Sales".to_string()));
}

fn return_input() -> String {
    println!("Add a query");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("lmao");
    input
}

fn text_to_query(map: &mut HashMap<String, String>, text: String) {
    let name = text.split_ascii_whitespace().collect::<Vec<&str>>();

    map.insert(
        name.get(1).unwrap().to_string(),
        name.get(3).unwrap().to_string(),
    );
    // println!("{}: {}", name.get(1).unwrap(), name.get(3).unwrap());
}

fn select_query(map: &HashMap<String, String>, query: Option<String>) {
    if let Some(department) = query {
        // Some(x) selects all people from department x
        println!("\nSHOWING EVERY EMPLOYEE FROM THE {} DEPARTMENT", department);
        for (k, v) in map {
            if *v == department {
                println!("{}: {}", k, v);
            }
        }
    } else {
        // None selects everything
        println!("\nSHOWING EVERY EMPLOYEE FROM EVERY DEPARTMENT");
        println!("{:#?}", map);
    }
}
