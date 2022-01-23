mod account;
mod student_list;
pub use crate::student_list::students_list;
pub use crate::account::accounts;


use std::thread;
use std::time::Duration;

fn main() {
    accounts::interface();
    thread::sleep(Duration::from_millis(300));
    // let a = String::from(r#"{"name":"Sinh vien B"}"#);
    // println!("{{ {}",a);
    // let var1 = "Sinh vien C";
    // let json = format!(r#"{{"name":"{}"}}"#, var1);
    // println!("{}",json);
    
    
    // let mut a= [8.5,7.5,5.0,9.0,3.0,5.5];
    // let b = a.len()-1;
    // println!("{:?}",sorts::merge_sort(&mut a,0,b));
    students_list::interface();
    

}
