#![allow(dead_code, unused_variables)]

use std::{
    // error::Error,
    fs::{self, File},
    io::{self, Read, Write},
    thread::sleep,
    time::Duration,
};

// -> Result<(), Box<dyn Error>>
pub fn run() {
    // unrecoverable_errors();
    // library_panic();

    // let username = read_username_from_file_question_shortest(String::from("~/Desktop/lol.txt"))?;
    // // println!("{}", username);
    // if username == "Maurice".to_string() {
    //     println!("Success");
    // } else {
    //     panic!("YOU DONE FUCKED UP!!");
    // };

    // Ok(())
}

fn unrecoverable_errors() {
    print!("Loading... [");
    for _ in 0..20 {
        sleep(Duration::from_millis(100));
        print!("=");
        _ = io::stdout().flush();
    }
    println!("=]\n");
    sleep(Duration::from_secs(1));

    panic!("OH BOY, IT'S GETTING HOT HERE");
}

fn library_panic() {
    let v = vec![1, 2, 3];
    v[99];
}

fn _file_error_panic_useless() {
    let _f = match File::open("~/titanic.csv") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {:?}", error),
    };
}

fn file_error_handling() {
    let _f = match File::open("~/Downloads/titanic.csv") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create("~/Downloads/titanic.csv") {
                Ok(f) => f,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

fn read_username_from_file(file_path: String) -> Result<String, io::Error> {
    let f = File::open(file_path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_question(file_path: String) -> Result<String, io::Error> {
    let mut f = File::open(file_path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_question_shorter(file_path: String) -> Result<String, io::Error> {
    let mut s = String::new();
    let f = File::open(file_path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_question_shortest(file_path: String) -> Result<String, io::Error> {
    // Reading from a file to a string is such a common occurence
    // that there's a one liner for it
    // It returns a Result containing
    // the String or the Error
    fs::read_to_string(file_path)
}

fn unwrap_when_err_is_impossible() {
    use std::net::IpAddr;
    // 127.0.0.1 is a vaild address, no need to check for error, so unwrap is acceptable
    let home = "127.0.0.1".parse::<IpAddr>().unwrap();
}
