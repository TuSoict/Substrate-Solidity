use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::io;

pub fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please try again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

pub fn login_check ()->i32 {
    println!("");
    println!("===========");
    println!("== Login ==");
    println!("===========");
    println!("");
    println!("== username: ==");
    let username = get_input().expect("username");
    println!("== password: ==");
    let password = get_input().expect("password");
    let account = [username, password];
    let joined = account.join("");
    let contents = fs::read_to_string("login.txt").expect("Something went wrong reading the file");

    let con = contents.split(",");
    for s in con {
        if joined == s{
            return 1; 
        }
    }
    return 0;
}

pub fn register_check()->std::io::Result<()>{
    println!("");
    println!("==============");
    println!("== Register ==");
    println!("==============");
    println!("");
    println!("== username: ==");
    let username = get_input().expect("username");
    println!("== password: ==");
    let password = get_input().expect("password");
    let mut file = File::open("login.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut account_new = String::new();
    account_new += &username;
    account_new += &password;
    contents+=",";
    contents+=&account_new;
    let mut buffer = File::create("login.txt")?;
    buffer.write_all(contents.as_bytes())?;
    Ok(())
}