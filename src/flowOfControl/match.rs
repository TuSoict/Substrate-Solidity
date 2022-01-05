
pub fn example() {
    let number = 7;
    // TODO ^ Try different values for `number`
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen "),
        _ => println!("Ain't special"),
    }

    match_tuple();
    match_enums();
    match_binding();
}

fn match_binding(){
    fn age() -> u32 {
        15
    }

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }
}

fn match_enums() {
    #[allow(dead_code)]
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
    }
    
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
           println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
    }
}

fn match_tuple() {  // tuples là một tập hợp giá trị với kiểu khác nhau
    let data = (0, -2, "hello world", true);
    match data {
        // Destructure the second and third elements
        (0, y, z, k) => println!("First is `0`, `y` is {:?},  `z` is {:?}  , `k` is {:?}  ", y, z , k),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (x,y,..) if x > y => println!(" {:?} is greater than {:?}",x,y),
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
}