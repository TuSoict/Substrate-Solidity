// Bai 17

struct Color {
    red: u8,
    blue: u8,
    green: u8
}

// Bai 18: tuple srtuct

struct Monster (u8 ,i32, u64);




fn main() {
    let mut x = 10;

   // let xr = &x;
    {
        let dom = &mut x;
        *dom += 1;
    }

    println!("x is: {}", x);

    let mut bg = Color {red: 45, blue: 74, green: 57};

    bg.red = 67;

    println!("{}  {} {}", bg.red, bg.blue, bg.green);

    // bai 18 tuple struct

    let mut monster = Monster(43, -54,  7564);

    monster.1 = 35;

    println!("{} {} {}", monster.0, monster.1, monster.2);

    // Bai 19: Pass by Reference

    let  blue = Color {red: 5, blue: 4, green: 5};
    
    print_color(&blue);
    print_color(&blue);


    // bo ky tu & di in 2 lan se bi loi vi gia tri cua bien blue da bi move chuyen quyen so huu sang ham print_color


}

// Bai 19: Pass by Reference

fn print_color (c: &Color) {
    println!("Color: R - {} G - {} B - {}", c.red, c.blue, c.green);
}