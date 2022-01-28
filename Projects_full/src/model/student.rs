use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub age: i32,
    pub math: i32,
    pub literature: i32,
}

pub struct Students{
    pub class: HashMap<String, Student>,
}

impl Students {
    pub fn new() -> Self{
        Self {class: HashMap::new()}
    }
    
    pub fn add(&mut self, student: Student) {
        self.class.insert(student.name.to_string(), student);
    }

    //view and sắp xếp theo điểm trung bình
    pub fn view_all(&self) -> Vec<&Student>{
        let mut sapxep: Vec<&Student> =  self.class.values().collect();//value() dùng để lấy tất cả giá trị
        sapxep.sort_by(|a: &&Student, b: &&Student|{
            let dtb_a= (a.math + a.literature)/2;
            let dtb_b= (b.math + b.literature)/2;

            if dtb_a == dtb_b {
                return Ordering::Equal;
            } if dtb_a > dtb_b {
                Ordering::Less
            } else {
                Ordering::Greater
            }   
        });

        sapxep
    }
    pub fn remove(&mut self, name: &str) -> bool{
        self.class.remove(name).is_some()       //is_some(): nếu có giá trị trả về là true, k có gtri trả về false
    }

    pub fn edit(&mut self, name: &str, age: i32) -> bool {
        match self.class.get_mut(name){               //get_mut() dữ liệu trả về là một biến có thể thay đổi được 
            Some(name) => {
                name.age = age; 
                true
            }
            None => false,
        }              
    }
}