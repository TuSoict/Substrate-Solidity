
#[derive(Debug)]
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(String),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

enum Status {
    Rich,
    Poor,
}

pub fn enum_example() {
    println!("Enum example");

    let press = WebEvent::KeyPress("alt".to_owned());
    println!("Key Press event :: {:?}", press);
    inspect(press)

}

fn  inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(s) => println!("you pressed '{}'", s),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}