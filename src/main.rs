
mod traits_demo;
mod file_demo;
mod model;
mod hashmap_demo;

//use serde_json::{Result,Value};

fn main() 
{
   hashmap_demo::hashmap_run();

   //traits_demo::run_sample();

//   let json_string = file_demo::run_example();
//   println! ("\n JSON : {} " ,json_string);

 // Parser json string to person
//   let person = parser(json_string);

//   //Save person to vector
//   let mut persons: Vec<model::Person> = Vec::new();
//   persons.push(person);

}

//fn parser (json_string:String) -> model::Person{
 // Parse the string of data into serde_json::Value.

//     let data = json_string.clone().into_boxed_str();

//     let u:Value = serde_json::from_str(&data).unwrap();
//     println!("{:#?}", u);


//      let u: model::Person = serde_json::from_str(&data).unwrap();
//      println!("{:#?}", u);

//  // Access parts of the data by indexing with square brackets.
//     println!("Please call {} at the number {}",u.name, u.phones[0]);

//     u
//}



