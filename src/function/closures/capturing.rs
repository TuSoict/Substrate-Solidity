pub fn example() {
    use std::mem;

    let color = String::from("green");
    let print = || println!("`get color`: {}", color);
    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume();  // `consume` consumes the variable so this can only be called once.
}