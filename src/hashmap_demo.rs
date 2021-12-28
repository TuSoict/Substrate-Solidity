
use std::collections::HashMap;

pub fn hashmap_run () {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Yellow")).or_insert(40);
    scores.entry(String::from("tu")).or_insert(50);

    println!("{:?}", scores);
}