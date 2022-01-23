use crate::student_list::{database,sorts,thread_or_something};
use std::{io,thread};
use std::time::{Duration, Instant};

use reqwest::header::{HeaderMap, AUTHORIZATION};

pub fn interface(){
    println!("================================ Manager Panel ================================");
    println!("|                                                                             |");
    println!("|         1. View, insert, delete,update students: Press 1                    |");
    println!("|         2. Sort student by total score: Press 2                             |");
    println!("|         3. Calculate students's average math score: Press 3                 |");
    println!("|         4. Calculate students's average code score: Press 4                 |");
    println!("|         5. Calculate students's average total score: Press 5                |");
    println!("|                                                                             |");
    println!("===============================================================================\n");

    loop{
        println!(" Please enter your choice");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
        let input: u8 = match input.trim().parse() {
            Ok(s) => s,
            Err(_) => continue,
        };
        match input{
            1 => {
                    //  Thread sleep  
                    thread::sleep(Duration::from_millis(300));
                    println!("===================== Manager Panel =====================");
                    println!("|                                                       |");
                    println!("|        1. View students list: Press 1                 |");
                    println!("|        2. Insert into students list: Press 2          |");
                    println!("|        3. Delete from students list: Press 3          |");
                    println!("|        4. Update students list: Press 4               |"); 
                    println!("|                                                       |"); 
                    println!("=========================================================");
                    database();
                    continue;
                },
            2 => { 
                    //  Thread sleep  
                    thread::sleep(Duration::from_millis(300));
                    println!("===================== Manager Panel =====================");
                    println!("|                                                       |");
                    println!("|        1. Use insertion sort: Press 1                 |");
                    println!("|        2. Use selection sort: Press 2                 |");
                    println!("|        3. Use bubble sort: Press 3                    |"); 
                    println!("|        4. Use merge sort: Press 4                     |"); 
                    println!("|        5. Use quick sort: Press 5                     |"); 
                    println!("|        6. Use heap sort: Press 6                      |"); 
                    println!("|                                                       |"); 
                    println!("=========================================================");
                    sorts();
                    continue;
                },
            3 => {
                    math(); 
                    continue;
                },
            4 => {  
                    code();
                    continue;
                },
            5 => {
                    total();
                    continue;
                },        
            _ => {
                    println!("\n Choice not match");
                    continue;},
        }
    }

}

fn database(){
    println!("\nPlease enter your choice your you can press 'q' to quit:");
    let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
    let input = input.trim();
    if input == "q" {
        thread::sleep(Duration::from_millis(300));
        interface();
    } else if input == "1" {
        println!("");
        match database::read(){
            Ok(_) => (),
            Err(e) => panic!("Failed to read because {}",e),
        };
        database();
    } else if input == "2" {
        database_insert();
    } else if input == "3" {
        database_delete()
    } else if input == "4" {
        database_update();
    } 
}
fn sorts(){
    
    println!("\nPlease enter your choice your you can press 'q' to quit:");
    let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
    let input = input.trim();
    
    if input == "q" {
        thread::sleep(Duration::from_millis(300));
        interface();
    } else if input == "1" {    
        let mut arr :Vec<f64> ;
        arr = match get_vector_total_point(){
            Ok(s) => s,
            Err(e) => panic!("{}",e),
        };
        let mut arr = &mut arr[..];
        let size = arr.len();
        let start = Instant::now();
        arr = sorts::insertion_sort(arr, size);
        let duration = start.elapsed();
        println!("Sorted successfully");
        println!("Time spend: {:?}\n", duration);

        println!("  Sorted total point:");
        match print_sorted(arr){
            Ok(_) => (), 
            Err(e) => panic!("Sorted fail because {}",e),
        };
    } else if input == "2" {
        let mut arr :Vec<f64>;
        arr = match get_vector_total_point(){
            Ok(s) => s,
            Err(e) => panic!("{}",e),
        };
        let mut arr = &mut arr[..];

        let start = Instant::now();
        arr = sorts::selection_sort(arr, arr.len());
        let duration = start.elapsed();
        println!("Sorted successfully");
        println!("Time spend: {:?}\n", duration);

        println!("  Sorted total point:");
        match print_sorted(arr){
            Ok(_) => (), 
            Err(e) => panic!("Sorted fail because {}",e),
        };
    } else if input == "3" {
        let mut arr :Vec<f64> ;
        arr = match get_vector_total_point(){
            Ok(s) => s,
            Err(e) => panic!("{}",e),
        };
        let mut arr = &mut arr[..];

        let start = Instant::now();
        arr = sorts::bubble_sort(arr, arr.len());
        let duration = start.elapsed();
        println!("Sorted successfully");
        println!("Time spend: {:?}\n", duration);


        println!("  Sorted total point:");
        match print_sorted(arr){
            Ok(_) => (), 
            Err(e) => panic!("Sorted fail because {}",e),
        };
    } else if input == "4" {
        let mut arr :Vec<f64> ;
        arr = match get_vector_total_point(){
            Ok(s) => s,
            Err(e) => panic!("{}",e),
        };
        let mut arr = &mut arr[..];
        
        let start = Instant::now();
        arr = sorts::merge_sort(arr, 0,arr.len()-1);
        let duration = start.elapsed();
        println!("Sorted successfully");
        println!("Time spend: {:?}\n", duration);


        println!("  Sorted total point:");
        match print_sorted(arr){
            Ok(_) => (), 
            Err(e) => panic!("Sorted fail because {}",e),
        };
    } else if input == "5" {
        let mut arr :Vec<f64> ;
        arr = match get_vector_total_point(){
            Ok(s) => s,
            Err(e) => panic!("{}",e),
        };
        let mut arr = &mut arr[..];

        let start = Instant::now();
        arr = sorts::quick_sort(arr, 0,arr.len()-1);
        let duration = start.elapsed();
        println!("Sorted successfully");
        println!("Time spend: {:?}\n", duration);

        println!("  Sorted total point:");
        match print_sorted(arr){
            Ok(_) => (), 
            Err(e) => panic!("Sorted fail because {}",e),
        };
    } else if input == "6" {
        let mut arr :Vec<f64> ;
        arr = match get_vector_total_point(){
            Ok(s) => s,
            Err(e) => panic!("{}",e),
        };
        let mut arr = &mut arr[..];
        
        let start = Instant::now();
        arr = sorts::heap_sort(arr);
        let duration = start.elapsed();
        println!("Sorted successfully");
        println!("Time spend: {:?}\n", duration);
        
        println!("  Sorted total point:");
        match print_sorted(arr){
            Ok(_) => (), 
            Err(e) => panic!("Sorted fail because {}",e),
        };
    } 

}

fn math(){
    let math :Vec<f64>= match get_vector_math_point(){
        Ok(vec) => vec,
        Err(e) => panic!("Failed to get math point because {}",e),
    };
    println!("\nAverage math point is: {}\n",thread_or_something::get_average_point(math));
}
fn code(){
    let code :Vec<f64>= match get_vector_code_point(){
        Ok(vec) => vec,
        Err(e) => panic!("Failed to get math point because {}",e),
    };
    println!("\nAverage code point is: {}\n",thread_or_something::get_average_point(code));
}
fn total(){
    let total :Vec<f64>= match get_vector_total_point(){
        Ok(vec) => vec,
        Err(e) => panic!("Failed to get math point because {}",e),
    };
    println!("\nAverage total point is: {}\n",thread_or_something::get_average_point(total));
}






















fn database_insert(){

    loop{
        let mut input_id = String::new();
        println!("Please enter student id (An interger):");
        io::stdin()
            .read_line(&mut input_id)
            .expect("Error reading");
        let input_id: i32 = match input_id.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                        println!("You do not enter an interger, please try again"); 
                        continue;},
        };

        let mut input_name = String::new();
        println!("Please enter student name:");
        io::stdin()
            .read_line(&mut input_name)
            .expect("Error reading");
        let input_name = input_name.trim().to_string(); 

        let mut input_age = String::new();
        println!("Please enter student age (An interger):");
        io::stdin()
            .read_line(&mut input_age)
            .expect("Error reading");
        let input_age: u8 = match input_age.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("You do not enter an interger, please try again"); 
                continue;},
        };

        let mut input_math = String::new();
        println!("Please enter student math point (A float number):");
        io::stdin()
            .read_line(&mut input_math)
            .expect("Error reading");
        let input_math: f64 = match input_math.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("You do not enter a float number, please try again"); 
                continue;},
        };

        let mut input_code = String::new();
        println!("Please enter student code point (A float number):");
        io::stdin()
            .read_line(&mut input_code)
            .expect("Error reading");
        let input_code: f64 = match input_code.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("You do not enter a float number, please try again"); 
                continue;},
        };

        match database::insert(input_id, input_name, input_age, input_math, input_code){
            Ok(_) => break,
            Err(e) => panic!("Failed to insert because {}", e),
        };
        
    }

    println!("\nInserted successfully");
    println!("Now your students list is:\n");
    match database::read(){
        Ok(_) => (),
        Err(e) => panic!("Failed to read because {}",e),
    };
    database();
}

fn database_delete(){
    loop {
        let mut input_id = String::new();
        println!("Please enter student id you want to delete(An interger):");
        io::stdin()
            .read_line(&mut input_id)
            .expect("Error reading");
        let input_id: i32 = match input_id.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                        println!("You do not enter an interger, please try again"); 
                        continue;},
        };

        match database::delete(input_id){
            Ok(_) => break,
            Err(e) => panic!("Failed to delete because {}", e),
        };
        
    }
    println!("\nDeleted successfully");
    println!("Now your students list is:\n");
    match database::read(){
        Ok(_) => (),
        Err(e) => panic!("Failed to read because {}",e),
    };
    database();
}


fn database_update() {
    loop{
        let mut input_id = String::new();
        println!("Please enter student id you want to update(An interger):");
        io::stdin()
            .read_line(&mut input_id)
            .expect("Error reading");
        let input_id: i32 = match input_id.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                        println!("You do not enter an interger, please try again"); 
                        continue;},
        };

        let mut input_name = String::new();
        println!("Please enter new student name:");
        io::stdin()
            .read_line(&mut input_name)
            .expect("Error reading");
        let input_name = input_name.trim().to_string(); 

        let mut input_age = String::new();
        println!("Please enter new student age (An interger):");
        io::stdin()
            .read_line(&mut input_age)
            .expect("Error reading");
        let input_age: u8 = match input_age.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("You do not enter an interger, please try again"); 
                continue;},
        };

        let mut input_math = String::new();
        println!("Please enter new student math point (A float number):");
        io::stdin()
            .read_line(&mut input_math)
            .expect("Error reading");
        let input_math: f64 = match input_math.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("You do not enter a float number, please try again"); 
                continue;},
        };

        let mut input_code = String::new();
        println!("Please enter new student code point (A float number):");
        io::stdin()
            .read_line(&mut input_code)
            .expect("Error reading");
        let input_code: f64 = match input_code.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("You do not enter a float number, please try again"); 
                continue;},
        };

        match database::update(input_id, input_name, input_age, input_math, input_code){
            Ok(_) => break,
            Err(e) => panic!("Failed to update because {}", e),
        };
    }

    println!("\nUpdated successfully");
    println!("Now your students list is:\n");
    match database::read(){
        Ok(_) => (),
        Err(e) => panic!("Failed to read because {}",e),
    };
    database();
}


#[tokio::main]
pub async fn print_sorted (arr:&mut[f64]) -> Result<(), Box<dyn std::error::Error>> {
        // pull outt token and key from env variable
    let api_key = database::API_KEY.to_string();
    // create a new header map to be used as reqwest header
    let mut headers = HeaderMap::new();
    // populate headers map with token and key
    // parse the string into  HeaderValue using parse()
    headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());


    let client = reqwest::Client::new();
    let resp = client
        .get("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?select=*")
        .headers(headers)
        .send()
        .await?;

    //  deserializing the response into a vector of Student
    let resp_json = resp.json::<Vec<database::Student>>().await?;
     
    for i in arr.to_owned(){
        for student in &resp_json{ 
            if student.code_point + student.math_point == i {
                println!("{} --    {}   -- {}",student.id,student.name,i);
                break;
            }
        }
    }
    Ok(())
}
#[tokio::main]
pub async fn get_vector_total_point() -> Result<Vec<f64>, Box<dyn std::error::Error>> {
       // pull outt token and key from env variable
    let api_key = database::API_KEY.to_string();
    // create a new header map to be used as reqwest header
    let mut headers = HeaderMap::new();
    // populate headers map with token and key
    // parse the string into  HeaderValue using parse()
    headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());


    let client = reqwest::Client::new();
    let resp = client
        .get("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?select=*")
        .headers(headers)
        .send()
        .await?;

    //  deserializing the response into a vector of Student
    let resp_json = resp.json::<Vec<database::Student>>().await?;
    let mut arr :Vec<f64> = Vec::new();
    for student in resp_json {
        arr.push(student.code_point+student.math_point);
    }
    // let total_points = &mut arr[..];
    // total_points
    Ok(arr)
}


#[tokio::main]
pub async fn get_vector_math_point() -> Result<Vec<f64>, Box<dyn std::error::Error>> {
       // pull outt token and key from env variable
    let api_key = database::API_KEY.to_string();
    // create a new header map to be used as reqwest header
    let mut headers = HeaderMap::new();
    // populate headers map with token and key
    // parse the string into  HeaderValue using parse()
    headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());


    let client = reqwest::Client::new();
    let resp = client
        .get("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?select=*")
        .headers(headers)
        .send()
        .await?;

    //  deserializing the response into a vector of Student
    let resp_json = resp.json::<Vec<database::Student>>().await?;
    let mut arr :Vec<f64> = Vec::new();
    for student in resp_json {
        arr.push(student.math_point);
    }
    // let total_points = &mut arr[..];
    // total_points
    Ok(arr)
}
#[tokio::main]
pub async fn get_vector_code_point() -> Result<Vec<f64>, Box<dyn std::error::Error>> {
       // pull outt token and key from env variable
    let api_key = database::API_KEY.to_string();
    // create a new header map to be used as reqwest header
    let mut headers = HeaderMap::new();
    // populate headers map with token and key
    // parse the string into  HeaderValue using parse()
    headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());


    let client = reqwest::Client::new();
    let resp = client
        .get("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?select=*")
        .headers(headers)
        .send()
        .await?;

    //  deserializing the response into a vector of Student
    let resp_json = resp.json::<Vec<database::Student>>().await?;
    let mut arr :Vec<f64> = Vec::new();
    for student in resp_json {
        arr.push(student.code_point);
    }
    // let total_points = &mut arr[..];
    // total_points
    Ok(arr)
}
