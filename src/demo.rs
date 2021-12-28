

use rand::Rng;
//use std::cmp::Ordering;
//use std::cmp::Ordering;
//use std::io;

 pub struct Profile {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}


 struct User {
    name: String,
    class: i32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn change(&mut self) {
        self.width = 500
    }
}


enum SimpleEnum {
    FirstVariant,
    SecondVariant,
    ThirdVariant,
}

enum Location {
    Unknown,
    Anonymous,
}


enum FEMAIL {
    a(i32)
}

enum Gioitinh {
    MALE (String),
    M(FEMAIL),
}



fn main() 
{

let a:Gioitinh = Gioitinh::MALE(String::from("tesst"));
let b = Gioitinh::M(FEMAIL::a(1));



    let opt = Option::None::<usize>;
let x = match opt {
    Some(int) => int,
    None => 10,
};
assert_eq!(x, 10);

let a_number = Option::Some(10);
match a_number {
    Some(x) if x <= 5 => {
        println!("0 to 5 num = {}", x)
    } ,
    Some(x @ 6..=10) => println!("6 to 10 num = {}", x),
    None => panic!(),
    // all other numbers
    _ => panic!(),
}


    println!("Guessing the number");

    let mut rng = rand::thread_rng();

    let secret_number = rng.gen_range(0,10);
    
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess");

    const MIN :&str = "TUTM";

    let s1 = String::from("hello");
    //s1.push('1');
    let num = calculate_length(s1);

   // println!("{}",s1);

    let num2 = calculate_length1(MIN);

    println!("tutmt {}",MIN);




    // let s2 = s1.clone();

    // println!("{:p} - {}",&s2,s2);

    // let s0 = &s1;
    // println!("{:p} - {}",&s0,s0);

    // println!("{}",s1)

    // println!("{} - {}", s2, num);


    let mut s4 = String::from("hello world");
    let num = first_word(&s4);

    s4.clear();

    println!("num {}",num);

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    &rect1.change();

    println!("rect continude {}",rect1.height);
    println!("rect continude {}",rect1.width);
    println!("rect1 is {:#?}", rng);



    let mut x = 10;

    let dow = &mut x;

    *dow += 1;

    println!("xx is {}",x);

   
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;
    
    assert_eq!([1, 2], &array[1..]);
    
    // This loop prints: 0 1 2
    for (i, &item) in array.iter().enumerate() {
        print!("{} ", x);
    }


   
   

}
fn calculate_length(s: String) -> usize {

// let mut s3 = String::from(s.clone());
// s3.push('s');
    s.len()
}

fn calculate_length1(s: &str) -> usize {
    //     println!("{:p}",&s);
    
    // let mut s3 = String::from(s.clone());
    // s3.push('s');
        s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    let t = b' '; // convert chart to  bytes 
    
    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }

    s.len()
}

fn build_user(email: String, username: String) -> Profile {
    Profile {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}