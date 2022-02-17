mod manager;
mod model;

use std::io;

fn main() {

    println!("**************************** 2022 *************************");
    println!("\n Hello, Welcome everyone!");

    let mut students: Vec<model::student::Student> = manager::file_helper::read_students_data();
    let mut users: Vec<model::student::User> = manager::file_helper::read_user_data();

    /*
     * Step 1: Login
     */
    let is_login = login(&mut users);
    if is_login {
        println!("\n Nice to meet you, Can I help you ? \n");

        /* 
        Step 2: show features 
        */
        while true {
            let do_action = run_feature(&mut students);
            if do_action == false {
                break
            }
        }

    }
}

fn login(users: &mut Vec<model::student::User>) -> bool {
    println!(
        "Please select 
    \n 1. Register 
    \n 2. Login 
    \n 3. Cancel \n"
    );

    let input = read_input_from_user();
    if input.contains("1") {
        //register
        println!(
            "\n Register: Please input 
    \n 1. username"
        );
        let name = read_input_from_user().trim().to_string();

        println!(
            "Please input 
     \n 2. password"
        );
        let pw = read_input_from_user().trim().to_string();

        if name.len() > 0 && pw.len() > 0 {
            println!("\n {} registed successfull", name);
        }

        let user = model::student::User {
            username: name,
            password: pw,
        };

        users.push(user);
        manager::file_helper::save_users(users.clone());
        return true;
    } else if input.contains("2") {
        if users.len() == 0 {
            println!("Account infor not found");
            return true;
        }

        println!(
            "Login: Please input 
    \n 1. username"
        );
        let name = read_input_from_user().trim().to_string();

        println!(
            "Please input 
     \n 1. password"
        );

        let pw = read_input_from_user().trim().to_string();

        if name.len() > 0 && pw.len() > 0 {
            for elem in users {
                if elem.username.eq(&name) && elem.password.eq(&pw) {
                    println!("{} Login successfull", name);
                    return true;
                }
            }
        }
    }
    return false;
}

fn run_feature(students: &mut Vec<model::student::Student>) -> bool {
    println!(
        "\n Please select features
    \n 1. Review the students list
    \n 2. Sort to MSSV 
    \n 3. Sort to averate score
    \n 4. Adding new students
    \n 5. Removing last students 
    \n 6. Update infor student
    \n 7. Cancel \n"
    );

    let input = read_input_from_user().trim().to_string();
    let my_int = input.parse::<i32>().unwrap();

    match my_int {
        1 => {
            /*
            Print default list students
             */
            manager::school_manager::students_print(
                students.to_vec(),
                manager::school_manager::OrderType::DEFAULTOrderType,
            );
            return true;
        }

        2 => {
            manager::school_manager::students_print(
                students.to_vec(),
                manager::school_manager::OrderType::MSSVOrderType,
            );
            return true;
        }

        3 => {
            manager::school_manager::students_print(
                students.to_vec(),
                manager::school_manager::OrderType::SCOREOrderType,
            );
            return true;
        }

        4 => {
            manager::school_manager::add_student(students.to_vec());
            return true;
        }

        5 => {
            manager::school_manager::del_student(students.to_vec());
            return true;
        }

        6 => {
            update_infor_student(students);
        }

        _ => {
            println!("Finished");
            return false;
        }
    }

    return false;
}

fn update_infor_student(students: &mut Vec<model::student::Student>) -> bool {
    println!("\n Please input mssv: \n");

    let mssv = read_input_from_user().trim().to_string();
    let my_mssv = mssv.parse::<u32>().unwrap();

    if (manager::school_manager::find_student(my_mssv, students.to_vec())) {
        println!(
            "\n Please select field what you will update
        \n 1. Class_name
        \n 2. email
        \n 3. Cancel \n"
        );
    }

    let input = read_input_from_user().trim().to_string();
    let my_int = input.parse::<i32>().unwrap();

    if input.contains("1") {
        println!("\n Plz input new class name \n");
    } else if input.contains("2") {
        println!("\n Plz input new email \n");
    } else {
        return false;
    }

    let update_infor = read_input_from_user().trim().to_string();
    match my_int {
        1 => {
            manager::school_manager::update_student(
                students.to_vec(),
                my_mssv,
                update_infor,
                String::from(""),
            );
            return true;
        }

        2 => {
            manager::school_manager::update_student(
                students.to_vec(),
                my_mssv,
                String::from(""),
                update_infor,
            );
            return true;
        }

        _ => return false,
    }

    true
}

fn read_input_from_user() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return input,
        Err(_e) => {
            return String::from("Cancel");
        }
    }
}
