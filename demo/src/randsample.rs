#![allow(unused_imports)]
pub mod advanced;
pub mod test;

pub mod simple {
    use rand::Rng;
    use rand::random;
    use rand::prelude::*;

    pub fn execute() {
        let mut rng = rand::thread_rng();
        // {
        //     let x: u8 = random();
        //     // println!("Fast random: {}", random::<f64>());
        // }
        // {
        //     let x: f64 = rng.gen_range(-40.0..=40.0); // [0; 1)
        //     println!("Float range random: {}", x);
        // }         

        // {
        //     // let x = rng.gen_bool(0.2);
        //     let x = rng.gen_ratio(1, 6); //a/b
        //     println!("Bool random with : {}", x);
        // }

        // {
        //     let x = rng.gen_range(1..7); // [1,7)
        //     println!("Specific range random: {}", x);
        // }

        {
            let mut nums = [1, 2, 3, 4, 5];
            nums.shuffle(&mut rng);
            println!("Some numbers: {:?}", nums);
        }
    }
}