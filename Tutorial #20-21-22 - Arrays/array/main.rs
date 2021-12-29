// fn main() {
//    // bAI 20 -Array

//    let numbers = [1, 2 ,34 , 6, 5];

//      // gia tri mac dinh cho mang

//      let arraydefault = [4 10];
 

//    // let arrays = ["1", 6 ,"34" , "fhfh", 5];  // ==> loi  expected `&str`, found integer

//    /*
//    expected `&str`, found integer ==> no se lay kieu du lieu cua phan tu dau tien de doi sanh 
//    cho cac phan tu con lai co cung lieu du lieu hy  ko
//    */

//    // println!("{}", arrays[1]);

//    println!("{}", numbers[4]);

//    // in ra cac phan tu mang voi vong lap for
//    for n in numbers.iter() {
//         println!("{}", n);
//    }

//    for n in arraydefault.iter() {
//         println!("{}", n);
//     }

//     /*
//         ket qua in ra 10 phan tu gia tri mac dinh la 4
//     */


//    // truy cap mang bang chi muc

//    for i in 0..numbers.len() {
//        println!("truy cap mang bang chi muc: {}", numbers[i]);
//    }

//    // khai bao kieu du lieu cho mang
   
//    let number1: [i32; 5] = [1, 2 ,34 , 6, 5];  // mang i 32  voi chieu dai la 5 phan tu

// }


  // Bai 21: Impl Keyword

   /*
    Thwm phuong thuc cho struct bang tu khoa Impl
   */

   struct Rectanrgular {
       width: u32,
       height: u32
   }
   
   impl Rectanrgular {
       fn print_description (&self) {
           println!("Rectangular: {} x {}", self.width, self.height);
       }

       fn is_square (&self) -> bool {
            self.width == self.height
        }
   }

     // Bai 22: String



    // fn main() {
    //     let my_rect = Rectanrgular {width: 54, height: 54};

    //     my_rect.print_description();

    //     println!("Rectangle is squar: {}", my_rect.is_square());



    //     // Bai 22: khai bao string

    //     let mut my_string = String::from("I am Rust developer");

    //     // mot so method voi string

    //     println!("Chuoi co do dai la {} ky tu", my_string.len());

    //     // Kiem tra chuoi co rong hay khong

    //     println!("Chuoi co rong khong: {}", my_string.is_empty());

    //     // let string_array = my_string.split_whitespace(); 

    //     // println!("phan tach chuoi: {}", string_array[0]); => khong tach dc

    //     for token in my_string.split_whitespace() {
    //         println!("Phan tach chuoi boi khoang trang {}", token);
    //     }

    //     // Kiem tra 1 chuoi co bao gom trong chuoi hay khong

    //     println!("Chuoi bao gom trong chuoi: {}", my_string.contains("developer"));

    //     // Noi chuoi 

    //     my_string.push_str(". Day la chuoi da dc noi");

    //     println!("{}", my_string);

    //     let test_str = "khong khai bao gi";

    //     println!("{}", test_str.len());

    //     for token in test_str.split_whitespace() {
    //         println!("test_str {}", token);
    //     }

    //     // test_str.push_str(" fgdgdd"); // method not found in `&str`

    //     // println!("{}",test_str );
    // }


   // Bai 23: Implementing Traits

   // stra

   struct Person {
       name: String,
       age: u8
   }

   impl ToString for Person {
       fn to_string(&self) -> String {
            return format!("My name is {} and  i am {} year old", self.name, self.age);
       }
   }

      // Bai 24: VECTO

    fn main() {
        let dom = Person {name: String::from("Dometic"), age: 21};

        println!("{}", dom.to_string());

        // khai bao 1 vecto

        // cach 1

        let my_vector1: Vec<i32> = Vec::new();

         // cach 2

        let mut my_vector = vec![2, 4, 6, 7];  // 1 vecto kieu du lieu la i32 luu 1 so gia tri mac dinh so nguyen

        // khac nhau giuaa vector va array la gi ? sao phai them kieu du lieu vecto

        println!("{}", my_vector[2]);

        // mot so phuong thuc

        // de day 1 phan tu vao 1 vecto thi can cap quyen cho vecto de co the mutable

        my_vector.push(56);

        // [2, 4, 6, 7, 56];

        println!("Day 1 phan tu vao vector: {}", my_vector[4]);

        //remove

        my_vector.remove(1);

         // [2, 6, 7, 56];

        println!("Remove 1 phan tu khoi 1 vector: {}", my_vector[1]);

        // in cac phan tu cua 1 vector

        for number in my_vector.iter() {
            println!("Phan tu vec to: {}", number);
        }
    }

    


