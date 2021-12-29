mod randsample;
mod randsample2;
// absolute path

mod strsample {
    pub fn execute() {
        {
            let mut hello = String::from("hello");
            hello.push('w');
            hello.push_str("orld!");
            println!("{}", hello);
        }
        
        {
            let mut s = String::from("bar");
            s.insert(1, 'a'); //baar
            s.insert_str(0, "foo"); //foobaar
            s.insert(4, 'g'); //foobgaar
            println!("{}", s);
        }
        
        {
            let mut s = String::from("1234567890");
            println!("s pop: {}", s.pop().unwrap());
            println!("s remove 4: {}", s.remove(4));
            println!("is empty: {}", s.is_empty());
            println!("{}", s);
        }

        {
            let hello = String::from("hello");
            println!("Length: {}", hello.len());
            println!("Capacity: {}", hello.capacity());
        }
        
        {
            let mut s = String::with_capacity(11);
            println!("Capacity: {}", s.capacity());
            for _ in 0..12 {
                s.push('a');
            }
            println!("Capacity: {}", s.capacity());
            s.shrink_to(16);
        
            println!("Capacity: {}", s.capacity());
            // s.shrink_to(0);
            s.shrink_to_fit();
            println!("Capacity: {}", s.capacity());
        }
    }
}

fn main() {
    // simple::execute();
    // advanced::execute();
    // strsample::execute();
    crate::randsample2::hello::execute();
}

