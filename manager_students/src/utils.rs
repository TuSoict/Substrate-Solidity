use std::io;

pub fn get_input() -> Option<String> {

    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Pls try again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

pub fn get_int() -> Option<i32> {
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };

    let parsed_input: Result<i32, _> = input.parse();
    match parsed_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}

pub fn get_float() -> Option<f32> {
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };

    let parsed_input: Result<f32, _> = input.parse();
    match parsed_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}

pub fn check_role()  {

    println!("Pls enter password admin: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let password :String = String::from("123456");

    if input.trim() != password {
        panic!("password not wrong");
    }

}

