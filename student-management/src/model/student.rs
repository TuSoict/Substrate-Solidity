use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::ErrorKind,
};
#[derive(Serialize, Deserialize, Debug, PartialEq, Ord, PartialOrd, Eq, Clone)]
pub struct Student {
    pub identifier: u8,
    pub name: String,
    pub score: u8
}

static mut LIST: Vec<Student> = vec![];
const FILE_PATH: &str = "database/student.json";
pub fn init() {
    let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");

    let list_student:Vec<Student> = serde_json::from_str(&data).unwrap();
    unsafe {
        LIST = list_student;
    }
}

pub fn create(name: String, score: u8) {
    unsafe {
        let mut identifier: u8 = 0;
        if LIST.last() == None {
            identifier = 1;
        } else {
            let last_user = LIST.last().unwrap();
            identifier = last_user.identifier +1;
        }
        let data = Student { identifier, name, score };
        LIST.push(data);
    }
    save();
}

pub fn update(identifier: u8, name: String, score: u8) {
    unsafe {
        for item in LIST.iter_mut() {
            if item.identifier == identifier {
                item.name = name;
                item.score = score;
                break;
            }
        }
    }
    save();
}

pub fn delete(identifier: u8) {
    unsafe {
        for (index, item) in LIST.iter().enumerate() {
            if item.identifier == identifier {
                LIST.remove(index);
                break;
            }
        }
    }
    save();
}

pub fn read(identifier: u8) -> Student {
    unsafe {
        for item in LIST.iter() {
            if item.identifier == identifier {
                return Student { identifier: item.identifier, name: item.name.clone(), score: item.score };
            }
        }
    }
    return Student{identifier: 0, name: "none".to_string(), score: 0};
}

pub fn read_all() -> Vec<Student> {
    unsafe {
        LIST.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        return LIST.clone();
    }
}

pub fn sort_score() -> Vec<Student> {
    unsafe {
        LIST.sort_by(|a, b| b.score.cmp(&a.score));
        return LIST.clone();
    }
}

pub fn save() {
    let f = File::open(FILE_PATH);

    let _ = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILE_PATH) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => {
                panic!("Problem read the file: {:?}", error)
            }
        },
    };

    unsafe {
        let json: String = serde_json::to_string(&LIST).unwrap();
        fs::write(FILE_PATH, &json).expect("Unable to write file");
    }
}
