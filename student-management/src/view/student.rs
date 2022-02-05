use std::io;
use crate::model::student::{self, Student};

pub fn menu() {
    loop{
        println!("================================================================");
        println!("1. Get student by ID");
        println!("2. Create student");
        println!("3. Update student by ID");
        println!("4. Delete student by ID");
        println!("5. Get all students");
        println!("6. Sort students");
        println!("0. Logout");
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
            0 => {
                logout();
                continue;
            },
            1 => {
                println!("================================================================");
                read();
                println!("================================================================");
                continue;
            },
            2 => { 
                println!("================================================================");
                create();
                println!("================================================================");
                continue;
            },
            3 => {
                println!("================================================================");
                update();
                println!("================================================================");
                continue;
            },
            4 => {  
                println!("================================================================");
                delete();
                println!("================================================================");
                continue;
            },
            5 => {
                println!("================================================================");
                read_all();
                println!("================================================================");
                continue;
            },
            6 => {
                println!("================================================================");
                sort_score();
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

fn create() {
    println!("Name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Can not read name");
    println!("Score:");
    let mut input_score = String::new();
    io::stdin().read_line(&mut input_score).expect("Can not read score");
    println!("Input Score: {}", input_score);
    let score: u8 = input_score.trim().parse::<u8>().unwrap();
    student::create(name.trim_end().parse().unwrap(), score);
}

fn update() {
    println!("Identifier:");
    let mut input_ident = String::new();
    io::stdin().read_line(&mut input_ident).expect("Can not read identifier");
    let identifier: u8 = input_ident.parse::<u8>().unwrap();
    println!("Name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Can not read name");
    println!("Score:");
    let mut input_score = String::new();
    io::stdin().read_line(&mut input_score).expect("Can not read score");
    let score: u8 = input_score.trim().parse::<u8>().unwrap();
    student::update(identifier, name, score);}

fn delete() {
    println!("Identifier:");
    let mut input_ident = String::new();
    io::stdin().read_line(&mut input_ident).expect("Can not read identifier");
    let identifier: u8 = input_ident.trim().parse::<u8>().unwrap();
    student::delete(identifier);
}

fn read() {
    println!("Identifier:");
    let mut input_ident = String::new();
    io::stdin().read_line(&mut input_ident).expect("Can not read identifier");
    let identifier: u8 = input_ident.trim().parse::<u8>().unwrap();
    let item = student::read(identifier);
    println!("Id: {}, name: {}, score: {}", item.identifier, item.name, item.score);
}

fn read_all() {
    let students: Vec<Student> = student::read_all();
    for item in students.iter() {
        println!("Id: {}, name: {}, score: {}", item.identifier, item.name, item.score);
    }
}

fn sort_score() {
    let students: Vec<Student> = student::sort_score();
    for item in students.iter() {
        println!("Id: {}, name: {}, score: {}", item.identifier, item.name, item.score);
    }
}

fn logout() {
    crate::view::user::menu();
}
