use std::convert::From;

#[derive(Debug)]
struct UserInfo {
    age: i32,
    name : String,
}

impl From<i32> for UserInfo {
    fn from(age: i32) -> Self {
        UserInfo {
            age,
            name: ("Nam le le ").to_string()
        }
    }
}

pub  fn example() {
    let my_str = "hello";
    let my_string = String::from(my_str);   // convert str to String


    let user_info = UserInfo::from(30);
    println!("UserInfo is {:?}", user_info);
}
