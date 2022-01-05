pub fn example(){

    for n in 1..11 {  //     for n in 1..=10 {
        println!("{}",n)
    }

    let names  =   vec!["Bob", "Frank", "Ferris"];


    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }

    }

    
    println!("names: {:?}", names);
}