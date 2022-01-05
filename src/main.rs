#[path = "./structure.rs"]
mod custom_type;

#[path = "./enum.rs"]
mod enum_example;

#[path = "./constant.rs"]
mod constant;

#[path = "./mutability.rs"]
mod mutability;

#[path = "./scopeAndShadowing.rs"]
mod scope_and_shadowing;

#[path = "./casting.rs"]
mod casting;

#[path = "./literal.rs"]
mod literal;

#[path = "./inference.rs"]
mod inference;

#[path = "./fromAndInto.rs"]
mod from_and_into;

#[path = "./tryFromAndInto.rs"]
mod try_from_and_into;

#[path = "./flowOfControl/if_else.rs"]
mod if_else;

#[path = "./flowOfControl/loop.rs"]
mod loop_ex;

#[path = "./flowOfControl/loop_returning.rs"]
mod loop_returning;

#[path = "./flowOfControl/while.rs"]
mod while_ex;

#[path = "./flowOfControl/for.rs"]
mod for_ex;

#[path = "./flowOfControl/match.rs"]
mod match_ex;

#[path = "./flowOfControl/if_let.rs"]
mod if_let;

#[path = "./flowOfControl/while_let.rs"]
mod while_let;

#[path = "./function/index.rs"]
mod function_main;

#[path = "./function/method.rs"]
mod method;

#[path = "./function/closures/closures.rs"]
mod closures;

#[path = "./function/closures/capturing.rs"]
mod closures_capturing;

#[path = "./thread/part-1.rs"]
mod thread_part1;
#[path = "./thread/part-2.rs"]
mod thread_part2;
#[path = "./thread/part-3.rs"]
mod thread_part3;



fn main() {
    // tuples();
    // array();
    // slice();

    // enum_example::enum_example();
    // constant::constant();
    // mutability::example();

    // scope_and_shadowing::scope();

    // casting::example();
    // literal::example();
    // inference::example();
    // from_and_into::example();
    // try_from_and_into::example();

    // if_else::example();
    // loop_ex::example();
    // loop_returning::example();
    // while_ex::example();
    // for_ex::example();
    // if_let::example();
    // while_let::example();


    // function_main::example();
    // method::example();
    // closures::example();
    // closures_capturing::example();

    // thread_part1::example();
    // thread_part2::example();
    thread_part3::example();
}


fn tuples() {
    //a collection that has diff types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // get value of a tuple's index
    println!("long tuple 8's value: {}", long_tuple.8);
    //get value of the tuple
    println!(" tuples: {:?}", long_tuple);


    // destructuring tuple
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("get value of a {:?}", a);
}

fn array() {
    //  has an array of length
    let array: &[u8; 3] = &[1u8, 2, 3];
    println!("{:?}", array);

    //get value of array index
    println!(" array 2: {:}", array[2]);
}

fn slice() {
    //  has unsized
    let slice: &[u8] = &[1u8, 2, 3];
    println!("slice{:?}", slice);

    //get value of array index
    println!(" slice 2: {:}", slice[1]);
}









