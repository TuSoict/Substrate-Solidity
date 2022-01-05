use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub fn example() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    let thread1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread1"),
        ];
        for val in vals {
            tx.send(val).unwrap();
        }
    });

    let thread2 = thread::spawn(move || {
        let value2 = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread2"),
        ];
        for val in value2 {
            tx1.send(val).unwrap();
        }
        println!("running thread 2 !");
    });

    let thread3 = thread::spawn(move || {
        let received = rx;

        for v in received {
            println!("get value : {}", v)
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
}