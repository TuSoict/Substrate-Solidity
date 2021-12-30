// 1.
// extern crate serde_json;

// use serde_json::Value as JsonValue;

// fn main() {

//     let json_info = r#"
//         {
//             "name" : "Tran Hieu",
//             "age" : 24,
//             "is_male" : true
//         }"#;

//         let res = serde_json::from_str(json_info);

//         if res.is_ok() {
//             let data : JsonValue = res.unwrap();
//                 println!("Name : {}", data["name"].as_str().unwrap());
//                 println!("Age : {}", data["age"]);
//             if(data["is_male"] == true){
//                 println!("Sex : male");
//             }else{
//                 println!("Sex : female");
//             }
//         }else{
//              println!("Sorry, not a json value");
//         }
// }

// // 2.
// extern crate serde;
// extern crate serde_json;

// #[macro_use]
// extern crate serde_derive;

// use serde_json::Value as JsonValue;

// #[derive(Serialize, Deserialize)]

// struct Person {
//     name : String,
//     age : u8,
//     is_male : bool,
// }

// fn main() {

//     let json_info = r#"
//         {
//             "name" : "Tran Hieu",
//             "age" : 24,
//             "is_male" : true
//         }"#;

//         let res = serde_json::from_str(json_info);

//         if res.is_ok() {
//             let data : Person = res.unwrap();
//                 println!("Name : {}", data.name);
//                 println!("Age : {}", data.age);
//             if(data.is_male == true){
//                 println!("Sex : male");
//             }else{
//                 println!("Sex : female");
//             }
//         }else{
//              println!("Sorry, not a json value");
//         }

// }

// 3.
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
use std::fs::File;
use std::io::Read;
use serde_json::json;

#[derive(Serialize, Deserialize)]

struct Student {
    name: String,
    age: u32,
    address: String,
}


// fn checkAge(student: &Student) -> bool {
//     if(student.age > 20){
//      return true ;
//     }
// }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
   use std::fs;

    let data = fs::read_to_string("./../data.json")
        .expect("Unable to read file");

    let json: serde_json::Value = serde_json::from_str(&data)
    .expect("JSON does not have correct format.");

    let json2 = serde_json::json!(json);

    let z = json2.as_array();

    let x = z.unwrap();

    for i in x {
        println!("data {:?}", i["age"]);
      }

}
