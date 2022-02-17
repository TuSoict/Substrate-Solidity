use crate::model::student;

use std::fs::File;
use std::io::Write;
use std::io::{BufReader, Read};

const STUDENT_PATH: &str = "src/resources/students.json";
const USER_PATH: &str = "src/resources/users.json";

/**
 * Public class
*/

pub fn read_students_data() -> Vec<student::Student> {
    let mut students: Vec<student::Student> = Vec::new();

    let data = _read_data_from_file(STUDENT_PATH.to_string());
    if data.len() > 0 {
        let value: serde_json::Value = serde_json::from_str(&data).unwrap();
        students = serde_json::from_value(value).unwrap();
    }
    students
}

pub fn read_user_data() -> Vec<student::User> {
    let mut user: Vec<student::User> = Vec::new();

    let data = _read_data_from_file(USER_PATH.to_string());
    if data.len() > 0 {
        let value: serde_json::Value = serde_json::from_str(&data).unwrap();
        user = serde_json::from_value(value).unwrap();
    }
    user
}

pub fn save_students(students: Vec<student::Student>) {
    //open lock 
    if students.len() > 0 {
        let json = serde_json::to_string(&students).unwrap();
        _write_data_to_file(json, STUDENT_PATH.to_string())
    }
    //endlock 
}

pub fn save_users(users: Vec<student::User>) {
    if users.len() > 0 {
        let json = serde_json::to_string(&users).unwrap();
        _write_data_to_file(json, USER_PATH.to_string())
    }
}

/**
 * private class
*/

fn _read_data_from_file(path: String) -> String {
    let mut data = String::new();
    let f = File::open(path).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn _write_data_to_file(data: String, path: String) {
    if data.len() <= 0 {
        return;
    }
    let mut f = File::create(path).expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}
