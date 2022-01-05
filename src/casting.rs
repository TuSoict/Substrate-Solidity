
//Casting : giống như ép kiểu

pub fn example() {
    let decimal = 65.4321_f32;
    let integer = decimal as u8;

    println!("casting from decimal to integer {:}",integer);  //  decimal to integer
    println!("1000 as a u16 is: {}", 1000 as u16);
    // println!("1000 as a u8 is : {}", 1000 as u8); // error because u8 is from 0 max is 255;

    println!("1000 as a string is: {}", 1000.to_string());  // to string

    // println!(" 232 as a i8 is : {}", 232 as i8);  // error because u8 is from 0 max is 255;

    println!("300.0 is {}", 300.0_f32 as u8);  // max u8 is 255 -> so the result is 255


}