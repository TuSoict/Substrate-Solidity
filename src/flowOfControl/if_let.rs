pub fn example() {
    let number = Some(7);
    let letter: Option<i32> = Some(0);
    let emoticon: Option<i32> = None;

    // If you need to specify a failure, use an else:
    if let None = letter {
        print!("NONE");
    } else {
        print!("{:?}",letter);
    }
}