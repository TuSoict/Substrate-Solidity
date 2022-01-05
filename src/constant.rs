const THRESHOLD: i32 = 10;

fn is_bigger(n: i32) -> bool {
    n > THRESHOLD
}

pub fn constant() {
    println!("thresh hold::  {:}", THRESHOLD);

    // THRESHOLD = 12 ;
    //
    // println!("thresh hold::  {:}", THRESHOLD);

    let n = 16;
    println!("{}",is_bigger(n));
}