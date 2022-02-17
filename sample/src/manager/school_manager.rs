use crate::model::student;
use rand::Rng;

#[warn(bindings_with_variant_name)]
pub enum OrderType {
    MSSVOrderType,
    SCOREOrderType,
    DEFAULTOrderType,
}

pub fn students_print(mut students: Vec<student::Student>, orderType: OrderType) {
    match orderType {
        OrderType::MSSVOrderType => {
            students.sort_by_key(|k| k.sort_field());
        }

        OrderType::SCOREOrderType => {
            for elem in students.iter_mut() {
                 elem.caculate_avr_score();
            }
            students.sort_by_key(|k| k.sort_avg_score());
        }
        OrderType::DEFAULTOrderType => {
        //NOTHING TODO
        }
    }

    println!("--------------------START---------------------");
    for elem in students {
        println!("{:?}", elem);
    }

    println!("--------------------END---------------------");

}

pub fn add_student(mut students: Vec<student::Student>) {
    let score_test = vec![3, 3, 4];
    let mut rng = rand::thread_rng();

    let newStudent = student::Student {
        mssv: rng.gen::<u32>(),
        name: "Tran Van A".to_string(),
        class_name: "12D".to_string(),
        email: "abc.gmail.com".to_string(),
        score: score_test,
        avr_score:0
    };
    students.push(newStudent);
    students_print(students.clone(), OrderType::MSSVOrderType);
    crate::manager::file_helper::save_students(students.clone());
}

pub fn update_student(mut students: Vec<student::Student>, mssv: u32, new_email: String, new_class_name:String) {
    for elem in students.iter_mut() {
        if elem.mssv == mssv {
            if new_email.len() > 0 {
                elem.email = new_email.clone();
            }
            if new_class_name.len() > 0 {
                elem.class_name = new_class_name.clone();
            }
        }

   }
    students_print(students.clone(), OrderType::MSSVOrderType);
    crate::manager::file_helper::save_students(students.clone());
}


pub fn del_student(mut students: Vec<student::Student>) {
    let idx = students.len() - 1;
    students.remove(idx);
    students_print(students.clone(), OrderType::MSSVOrderType);
    crate::manager::file_helper::save_students(students.clone());
}

pub fn find_student(mssv : u32, students: Vec<student::Student>) -> bool {
    for elem in students.iter() {
        if elem.mssv == mssv {
            return true
        } 
   }
   false
}
