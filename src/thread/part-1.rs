use std::thread;
use std::time::{Duration, Instant};


pub fn example() {
    // ex();
    ex1();
    ex2();
    // ex3();

    thread::sleep(Duration::from_secs(2));

}

fn ex() {
    let start = Instant::now();

    for i in 1..5 {
        println!("hi number {} from the 1st loop !", i);
        thread::sleep(Duration::from_secs(1));
    }

    for i in 1..5 {
        println!("hi number {} from the 2nd loop !", i);
        thread::sleep(Duration::from_secs(1));
    }

    let duration = start.elapsed();
    println!("---------Time spent : {:?}", duration);
}


fn ex1() {
    let handle = thread::spawn(|| {
        for i in 1..1000 {
            println!("hi number {} from the spawned thread ex1 !", i);
        }
    });
}


fn ex2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread ex2 !", i);
        }
    });
}


fn ex3() {
    let mut x = "a string";

    let handle = thread::spawn(move || {
        let x = "test1";
        println!(" value of x: {:?}", x);
    });

    let handle2 = thread::spawn(move || {
        let x = "test222";
        println!(" value of x: {:?}", x);
    });

    handle.join().unwrap();

    println!("value of x : {:?}", x)
}