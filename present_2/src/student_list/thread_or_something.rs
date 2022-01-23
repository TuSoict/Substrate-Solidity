use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

pub fn get_average_point(vec_a: Vec<f64>) -> f64 {
    let (tx,rx): (Sender<f64>, Receiver<f64>) = mpsc::channel();
    
    for i in 0..3{
        let thread_tx = tx.clone();
        let vec_a = vec_a.clone();
        thread::spawn( move || {
            let mut sum :f64= 0.0; 
            for value in vec_a[vec_a.len()*i/3..(vec_a.len()*(i+1)/3  )].iter() {
                sum += value;
            }
            
            thread_tx.send(sum).unwrap();
        });
        
    }
   
    let mut recieve : f64 = 0.0;
    for _ in 0..3{
        let recieved = rx.recv().unwrap(); 
        
        recieve += recieved
    }
    // println!("recieved: {}", recieve);
    let len = vec_a.len() as f64;
    recieve/len 

}