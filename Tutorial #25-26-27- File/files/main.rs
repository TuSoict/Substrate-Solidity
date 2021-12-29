// Bai 25: Reading a File

// goi thu vien va module de co the doc va ghi vao 1 file

// Bai 27 - Writing to a File

use std::fs::File;
use std::io::prelude::*; //  doc va ghi vao 1 file

// Bai 26 - Command Line Arguments

// su dung module env

use std::env;

fn main() {
    
    // let mut file = File::open("info.txt").expect("Can't read this file");

    // let mut contents = String::new();

    // file.read_to_string(&mut contents)
    //     .expect("Oop! can't  read this file");

    //     println!("File content: \n\n {}", contents);

    //     // Doc 1 file csv, doc cac cot nhu the nao ? ghi du lieu vao cot ra sao ?


    // ==============================================

     let args: Vec<String> = env::args().collect();

    // // print the vector using the debug formatter, :?

    // println!("Gia tri cua vector: {:?}", args);

    // for arg in args.iter() {
    //     println!("{}", arg);
    // }

   //  println!("Ten cua chuong trinh se chiem gia tri dau tien tai index 0 trong vector: : {}", args[1]);

    // let query = &args[1];
    // let filename = &args[2];

    // println!("Searching for: {}", query);
    // println!("In file {}", filename);

    // ==============================================

    let mut  writefile = File::create("output.txt")
        .expect("Can't write a file");

    //     // ghi data vao file can su dung 1 slice byte de

         writefile.write_all(b"Hello World Rust").expect("Can't write in file");

   //    writefile.write_all(b"Hế ố Rust").expect("Can't write in file");

      //  ghi gia tri tu CLI vao file

    //   let arg = args[1];

    //     writefile.write_all(b"This í value from CLI: ", arg).expect("Can't write in file");

    // ghi 1 file vao cac folder khac nhau

    // ghi UTF-8

    // Ghi gia tri tu CLI vao trong file

    // doc data tu 1 file ghi vao 1 file khac

}
