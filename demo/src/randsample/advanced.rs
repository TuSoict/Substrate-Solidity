#![allow(unused_imports, dead_code)]
use rand::distributions::{Alphanumeric, Distribution, Standard, Uniform, WeightedIndex};
use rand::prelude::*;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen::<(i32, i32)>();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

#[derive(Debug)]
pub enum Hero {
    Common,
    Rare,
    Legendary,
}

impl Distribution<Hero> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hero {
        let index: u8 = rng.gen_range(1..=10);
        match index {
            1..=6 => Hero::Common,
            7..=9 => Hero::Rare,
            10 => Hero::Legendary,
            _ => unreachable!(),
        }
    }
}

pub fn execute() {
    let mut rng = rand::thread_rng();
    // {
    //     let pass: String = Alphanumeric
    //         .sample_iter(&mut rng)
    //         .take(30)
    //         .map(char::from)
    //         .collect();
    //     println!("Password gen: {}", pass);
    // }

    // {
    //     let rand_tuple = rng.gen::<(i32, bool, f64)>();
    //     let rand_point: Point = rng.gen();
    //     println!("Random tuple: {:?}", rand_tuple);
    //     println!("Random Point: {:?}", rand_point);
    // }

    {
        let x: Hero = rng.gen();
        println!("Random Hero: {:?}", x);
    }

    // {
    //     let choices = [('a', 2), ('b', 1), ('c', 1)];
    //     let mut rng = thread_rng();
    //     // 50% chance to print 'a', 25% chance to print 'b', 25% chance to print 'c'
    //     println!(
    //         "Random with weight: {:?}",
    //         choices.choose_weighted(&mut rng, |item| item.1).unwrap().0
    //     );
    // }

    // {
    //     let choices = ['a', 'b', 'c'];
    //     let weights = [2, 1, 1];
    //     let dist = WeightedIndex::new(&weights).unwrap();
    //     let mut rng = thread_rng();
    //     for _ in 0..10 {
    //         // 50% chance to print 'a', 25% chance to print 'b', 25% chance to print 'c'
    //         println!("{}", choices[dist.sample(&mut rng)]);
    //     }

    //     let items = [('a', 0), ('b', 3), ('c', 7)];
    //     let dist2 = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
    //     for _ in 0..10 {
    //         // 0% chance to print 'a', 30% chance to print 'b', 70% chance to print 'c'
    //         println!("{}", items[dist2.sample(&mut rng)].0);
    //     }
    // }
}