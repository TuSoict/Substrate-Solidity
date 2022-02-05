use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::ErrorKind
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}
static mut LIST: Vec<User> = vec![];
const FILE_PATH: &str = "database/user.json";
pub fn init() {
    let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");

    let list_user:Vec<User> = serde_json::from_str(&data).unwrap();
    unsafe {
        LIST = list_user;
    }
}

pub fn register(username: String, password: String) -> bool {
    let mut result = true;
    unsafe {
        let list_user = &LIST;
        for item in list_user.iter() {
            if item.username == username {
                result = false;
                break;
            }
        }
    }

    if result {
        let data = User { username, password };
        unsafe {
            LIST.push(data);
        }
    }
    save();
    result
}

pub fn login(username: String, password: String) -> bool {
    let mut result = false;
    unsafe {
        let list_user = &LIST;
        for item in list_user.iter() {
            println!("{:?}", item);
            if item.username == username {
                if item.password == password {
                    result = true;
                    break;
                }
            }
        }
    }

    result
}

#[allow(dead_code)]
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
