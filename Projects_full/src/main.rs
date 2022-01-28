mod model;
mod manager;
mod register_login;

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager {
    fn show(){
        println!("");
        println!("== Manager Panel ==");
        println!("");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("");
        println!("Please Enter Your Choice:");
        println!("");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn main() {
    register_login::home();
    let mut students = model::student::Students::new();
    loop {
        Manager::show();
        let input = manager::school_manager::get_input().expect("Please enter your data");
        match Manager::choice(&input.as_str()) {
            Some(Manager::AddStudent) => manager::school_manager::add_student(&mut students),
            Some(Manager::ViewStudent) => manager::school_manager::view(&students),
            Some(Manager::EditStudent) => manager::school_manager::edit_student(&mut students),
            Some(Manager::DeleteStudent) => manager::school_manager::remove_student(&mut students),
            None => return,
        }    
    }
}