pub fn example() {
    fn xx(i: i32) -> i32 { i + 1 }   // a function

// Closures are anonymous, here we are binding them to references
// Annotation is identical to function annotation but is optional
// as are the `{}` wrapping the body. These nameless functions
// are assigned to appropriately named variables.
    let closure_annotated = |input: i32| -> i32 { input + 1 };
    let closure_inferred = |input| input + 1;

    let i = -2;

    //Call function
    println!("function: {}", xx(i));

    //Call closures
    println!("closure_inferred: {}", closure_inferred(i));
    println!("closure_annotated: {}", closure_annotated(i));


// A closure taking no arguments which returns an `i32`.
// The return type is inferred.
    let one = || String::new();
    println!("closure returning one: {}", one());
}
