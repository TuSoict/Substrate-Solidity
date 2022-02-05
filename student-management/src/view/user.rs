use std::io;
use crate::model::user;
use crate::view::student;

pub fn menu() {
    loop{
        println!("================================================================");
        println!("1. Login");
        println!("2. Register");
        println!("================================================================");
        println!("Please enter your option");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
        let input: u8 = match input.trim().parse() {
            Ok(s) => s,
            Err(_) => continue,
        };
        match input{
            1 => {
                println!("================================================================");
                login();
                println!("================================================================");
                continue;
            },
            2 => { 
                println!("================================================================");
                register();
                println!("================================================================");
                continue;
            },      
            _ => {
                println!("\nOption not found!");
                continue;
            },
        }
    }
}

fn login() {
    println!("Username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Can not read username");
    println!("Password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Can not read password");
    if user::login(username.trim().parse().unwrap(), password.trim().parse().unwrap()) {
        println!("Login successful!");
        student::menu();
    } else {
        println!("Login failed!");
        menu(); 
    }
}

fn register() {
    println!("Username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Can not read username");
    println!("Password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Can not read password");
    if user::register(username.trim().parse().unwrap(), password.trim().parse().unwrap()) {
        println!("Register successful!");
    } else {
        println!("Register failed!");
    }
    menu();
}