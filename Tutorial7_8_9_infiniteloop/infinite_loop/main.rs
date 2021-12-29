fn main() {
    let mut n = 0;

    //  infinite loop  
  
    loop {

        n += 1;

        if n > 10 {
            break;
        }


        if n == 7 {
            continue;
        }


        println!("This is value of n: {}", n);
    }

    // khi ra ngoai scope luc nay n se co gia tri cuoi cung dc gan la 11

    //  while loop  
    let mut m = 0;

    while m<= 20 {
        println!("Value of m is: {}", m);

        // if m == 5 {
        //     continue;
        // }

        if m > 15 {
            break;
        }

        m += 1;
    }

        //  For loop  
        let numbers = 34..67;
        for i in numbers {
            println! ("Values of i: {}", i);
        }

        let animals = vec!["Rabbit", "Dog", "Cat"];

        for  (index, a) in animals.iter().enumerate() {
            println!("The index is: {} and animal is: {}", index, a);
        }

        // let mut guess = String::new();
        // io::stdin()
        //         .read_line(&mut guess)
        //         .expect("Failed to read line");

        // let guess: u32 = match guess.trim().parse() {
        //         Ok(num) => num,
        //         Err(_) => continue,
        //     };
        // println!("Your guess: {}", guess);

}
