use std::sync::{Arc, Mutex};
use std::thread;

pub fn example() {
    let mut counter = Arc::new(Mutex::new(10));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("run thread {:?} with value of counter: {:?}",i,counter.lock().unwrap());
            *counter.lock().unwrap() += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}", counter.lock().unwrap());
}
