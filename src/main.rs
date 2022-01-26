use std::{collections::HashMap};
mod utils;
pub use crate::utils::check::login_check;
use manager::{edit_student, remove_student};

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

pub fn get_int() -> Option<i32>{
    println!("Enter age, score of Student");
    let input = match get_input(){
        Some(input) => input,
        None => return None,
    };
    let parsed_input: Result<i32, _> = input.parse();
    match parsed_input{
        Ok(input)=>Some(input),
        Err(_)=>None,
    }
}

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub age: i32,
    pub score: i32,
}

pub struct Students{
    class:HashMap<String, Student>
}

impl Students {
    fn new() -> Self{
        Self { class: HashMap::new()}
    }
    fn add(&mut self, student: Student){
        self.class.insert(student.name.to_string(),student);
    }
    fn view_all(&self) -> Vec<&Student>{
        self.class.values().collect()
    }
    pub fn remove(&mut self, name: &str) -> bool{
        self.class.remove(name).is_some()
    }
    pub fn edit(&mut self, name:&str,score:i32, age:i32)->bool{
        match self.class.get_mut(name){
            Some(name) => {
                name.age = age;
                name.score = score;
                true
            },
            None => false,
        }
    }
}

mod manager {
    // use std::cmp::Ordering;

    use crate::{Student, Students, get_input, get_int };

    pub fn add_student(students: &mut Students){
        println!("Enter name of Student");
        let name = match get_input(){
            Some(input) => input,
            None => return,
        };
        let age = match get_int(){
            Some(number) => number,
            None=> return,
        };
        let score = match get_int(){
            Some(number) => number,
            None=> return,
        };
        let student = Student{name, age, score}; 
        students.add(student)
    }
    pub fn view(students: &mut Students){
        for students in students.view_all(){
            println!("{:?}", students);
        }
    }
    pub fn remove_student(students: &mut Students){
        for student in students.view_all(){
            println!("{:?}", student);
        }
        let name = match get_input(){
            Some(input)=>input,
            None=> return
        };
        if students.remove(&name){
            println!("remove student");
        }else{
            println!("not found");
        }
    }

    pub fn edit_student(students: &mut Students){
        for student in students.view_all(){
            println!("{:?}", student);
        }
        let name = match get_input(){
            Some(input)=>input,
            None=> return
        };
        let age = match get_int(){
            Some(input)=>input,
            None=>return,
        };
        let score = match get_int(){
            Some(input)=>input,
            None=>return,
        };
        if students.edit(&name, score, age){
            println!("student has edit")
        }else{
            println!("not found")
        }
    }
}

pub fn manager_student(){
    let mut students = Students::new();
    let x:i32 = login_check();
    if x==1 {
    loop {
            Manager::show();
            let input = get_input().expect("Please enter your data");
            match Manager::choice(&input.as_str()){
                Some(Manager::AddStudent) => manager::add_student(&mut students),
                Some(Manager::ViewStudent) => manager::view(&mut students),
                Some(Manager::EditStudent) => (edit_student(&mut students)),
                Some(Manager::DeleteStudent) => (remove_student(&mut students)),
                None=>break,
            }
        }
    }else{
        println!("Error username or password");
    }
}

mod login_regirter {
    use crate::{manager_student, utils::check::register_check};

    pub fn login(){
        manager_student();
    }
    pub fn register(){
        #[warn(unused_must_use)]
        register_check();
        manager_student();
    }
}

fn run_program(){
    LoginRegister::show();
    let input = get_input().expect("Please enter your data");
    match LoginRegister::choice(&input.as_str()){
        Some(LoginRegister::Login) => login_regirter::login(),
        Some(LoginRegister::Register) => login_regirter::register(),
        None=>(),
    }
}

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

enum LoginRegister {
    Login,
    Register,
}

impl LoginRegister {
    fn show(){
        println!("");
        println!("=======================");
        println!("== Login / Register ==");
        println!("=======================");
        println!("");
        println!("1. Login");
        println!("2. Register");
        println!("");
        println!("Please Enter Your Choice: ");
        println!("");
    }

    fn choice(input:&str) ->Option<LoginRegister>{
        match input{
            "1" => Some(LoginRegister::Login),
            "2" => Some(LoginRegister::Register),
            _ => None,
        }
    }
}

impl Manager {
    fn show(){
        println!("");
        println!("===================");
        println!("== Manager Panel ==");
        println!("===================");
        println!("");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("");
        println!("Please Enter Your Choice: ");
        println!("");
    }

    fn choice(input:&str) ->Option<Manager>{
        match input{
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}


fn main() {
    run_program();
}
