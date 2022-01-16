// pub fn run1() {
//     let name = String::from("Domenic");
//     //  Domenic have 7 characters so we can't call character number 8 


//     println!("Character at index 8: {}", 
//         match name.chars().nth(8)
//         {
//             Some(c) => c.to_string(),
//             None => "No character at index 8".to_string(),
//         }
//     );
    
// }


pub fn run2() {
    let five =Some(5);
    let none =plus_one(None);
    let six = plus_one(five);
    if let Some(a) =  none{
        println!("Five plus one equal: {}",a)
    } else{
        println!("None value")
    };
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        Some(i) => Some(i+1),
        None => None,
    }
}

pub fn run3() -> Option<()>{
    let domenic = get_occupation("Domenic")?.to_string();
    println!("Occupation is {}", domenic);
    Some(())
}
pub fn get_occupation(name: &str) -> Option<&str>{
    match name{
        "Domenic" => Some("Sofware Developer"),
        "Michael" => Some("Dentist"),
        _ => None,
    }
}
