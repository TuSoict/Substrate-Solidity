//Ví dụ ở dưới : tạo một vector . lúc này compiler sẽ chưa biết chính xác kiểu của vector
//Sau khi push vào 1 phần tử có kiểu u8, compiler kết luận rằng đây là vecter u8


pub  fn example() {

    let elem = 5u8;
    let elem1 = 2;


// Create an empty vector (a growable array).
    let mut vec = Vec::new();
// At this point the compiler doesn't know the exact type of `vec`, it
// just knows that it's a vector of something (`Vec<_>`).

// Insert `elem` in the vector.
    vec.push(elem);
//     vec.push(elem1);
// Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
// TODO ^ Try commenting out the `vec.push(elem)` line

    println!("vec after push {:?}", vec);
}
