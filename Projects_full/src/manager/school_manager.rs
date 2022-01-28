use std::io;
use crate::model::student;

    pub fn add_student(students: &mut student::Students){
        println!("Enter name of student");
        let name = match get_input(){
            Some(input) => input,
            None => return,
        };

        println!("Enter Age of Student");
        let age = match get_int(){
            Some(number ) => number,
            None => return,
        };
        
        println!("Enter Math of Student");
        let math = match get_int(){
            Some(number ) => number,
            None => return,
        };

        println!("Enter Literature of Student");
        let literature = match get_int(){
            Some(number ) => number,
            None => return,
        };

        let student = student::Student {name, age, math, literature};

        students.add(student)
    }

    pub fn view(students: &student::Students){
        for student in students.view_all(){
            println!("{:?}", student); 
        }
    }

    pub fn remove_student(students: &mut student::Students) {
        for student in students.view_all(){
            println!("{:?}", student); 
        }
        let name = match get_input() {
            Some(input) => input,
            None => return
        };
        if students.remove(&name){
            println!("remove student"); 
        } else {
            println!("not found");
        }
    }

    pub fn edit_student(students: &mut student::Students) {
        for student in students.view_all() {
            println!("{:?}", student);
        }
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let age = match get_int() {
            Some(input) => input,
            None => return,
        };
        if students.edit(&name, age) {
            println!("student has edit");
        } else {
            println!("not found");
        }
    }

pub fn get_input() -> Option<String>{
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please try again");
    }
    
    let input = buffer.trim().to_owned();
    if &input == ""{
        None
    }   else {
        Some(input)
    }
}
pub fn get_int() -> Option<i32> {
    let input = match get_input(){
        Some(input) => input,
        None => return None
    };

    let parsed_input: Result<i32,_> = input.parse();
    match parsed_input {
        Ok(input) => Some(input),
        Err(_) => None,
    } 
}