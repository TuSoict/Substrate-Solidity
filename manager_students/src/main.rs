
use std::collections::HashMap;
mod utils;

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    class_name: String,
    math_score: f32,
    literature_score: f32,
    gpa: f32,
}

pub struct Students {
    class: HashMap<String, Student>,
}

impl Students {
    fn new() -> Self {
        Self {
            class: HashMap::new(),
        }
    }

    fn add(&mut self, student: Student) {
        self.class.insert(student.name.to_string(), student);
    }

    fn get_list(&self) -> Vec<&Student> {
        self.class.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.class.remove(name).is_some()
    }

    fn edit(&mut self, name: &str, class_name: &str, math_score: f32, literature_score: f32, gpa: f32) -> bool {
        match self.class.get_mut(name) {
            Some(name) => {
                name.class_name = class_name.to_string();
                name.math_score = math_score;
                name.literature_score = literature_score;
                name.gpa = gpa;
                true
            }
            None => false,
        }
    }
}

mod manager {
    use crate::*;
    pub fn add_student(students: &mut Students) {

        utils::check_role();

        println!("Enter name of student : ");
        let name = match utils::get_input() {
            Some(input) => input,
            None => return,
        };

        println!("Enter class of student {} :", name);
        let class_name = match utils::get_input() {
            Some(number) => number,
            None => return,
        };

        println!("Enter score of maths {} : ",  name);
        let math_score = match utils::get_float() {
            Some(number) => number,
            None => return,
        };

        println!("Enter score of literature {} : ",  name);
        let literature_score = match utils::get_float() {
            Some(number) => number,
            None => return,
        };

        let gpa :f32 = (math_score + literature_score ) / 2 as f32;


        let student = Student { name, class_name, math_score, literature_score, gpa };

        students.add(student)
    }

    pub fn view(students: &Students) {
        for student in students.get_list() {
            println!("{:?}", student);
        }
    }

    pub fn remove_student(students: &mut Students) {
        for student in students.get_list() {
            println!("{:?}", student);
        }
        let name = match utils::get_input() {
            Some(input) => input,
            None => return,
        };
        if students.remove(&name) {
            println!("Removed student");
        } else {
            println!("N ot found");
        }
    }

    pub fn edit_student(students: &mut Students) {

        //get all students
        for student in students.get_list() {
            println!("{:?}", student);
        }

        println!("Enter name of student wanna edit : ");
        let name = match utils::get_input() {
            Some(input) => input,
            None => return,
        };

        println!("Enter class of student {} wanna edit : ", name);
        let class_name = match utils::get_input() {
            Some(input) => input,
            None => return,
        };

        println!("Enter score of math {} wanna edit : ", name);
        let math_score = match utils::get_float() {
            Some(number) => number,
            None => return,
        };

        println!("Enter score of literature {} wanna edit : ", name);
        let literature_score = match utils::get_float() {
            Some(number) => number,
            None => return,
        };

        let gpa :f32 = (math_score + literature_score ) / 2 as f32;


        if students.edit(&name, &class_name, math_score, literature_score,gpa) {
            println!("Student {} has edit", name);
        } else {
            println!("Not found");
        }
    }
}


enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager {
    fn show() {
        println!("");
        println!("--------QUAN LY SINH VIEN----------");
        println!("");
        println!("1. Add student");
        println!("2. View students");
        println!("3. Edit student");
        println!("4. Delete student");
        println!("5. Sort student by GPA score");
        println!("");
        println!("Please enter your choice:");
        println!("");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn run_program() {
    let mut students = Students::new();
    loop {
        Manager::show();
        let input = utils::get_input().expect("Please enter your data");
        match Manager::choice(&input.as_str()) {
            Some(Manager::AddStudent) => manager::add_student(&mut students),
            Some(Manager::ViewStudent) => manager::view(&students),
            Some(Manager::EditStudent) => manager::edit_student(&mut students),
            Some(Manager::DeleteStudent) => manager::remove_student(&mut students),
            None => break,
        }
    }
}

fn main() {
    run_program();
    println!("exit program");
}