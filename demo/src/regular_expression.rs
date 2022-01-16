extern crate regex;
use regex::Regex;
use std::io;
pub fn run() {
    // // //  Result<Regex, Error> 
    // let re = Regex::new(r"(\w{4})").unwrap();
    // //  \w{5} => word have 5 characters
    // let text = "de3co3de";
    // println!("{}",re.is_match(text));

    // let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    // let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    // for cap in re.captures_iter(text) {
    //     println!("{}",&cap[0]);
    //     println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    // }


    let re = Regex::new(r"0(\d{9})").unwrap();
    loop {
        let mut phone_num = String::new();
        println!("Please input your phone number");
        io::stdin()
            .read_line(&mut phone_num)
            .expect("Failed to read line");
        
        if re.is_match(&phone_num){ 
            println!("Your phone number is: {}",phone_num);
            break; }
    }





    // .           any character except new line (includes new line with s flag)
    // [xyz]       A character class matching either x, y or z.
    // [^xyz]      A character class matching any character except x, y and z.
    // [a-z]       A character class matching any character in range a-z.
    // \d          digit (\p{Nd})
    // \D          not digit

    // x*        zero or more of x (greedy)
    // x+        one or more of x (greedy)
    // x?        zero or one of x (greedy)
    // x*?       zero or more of x (ungreedy/lazy)
    // x+?       one or more of x (ungreedy/lazy)
    // x??       zero or one of x (ungreedy/lazy)
    // x{n,m}    at least n x and at most m x (greedy)
    // x{n,}     at least n x (greedy)
    // x{n}      exactly n x
    // x{n,m}?   at least n x and at most m x (ungreedy/lazy)
    // x{n,}?    at least n x (ungreedy/lazy)
    // x{n}?     exactly n x

    // ^     the beginning of text (or start-of-line with multi-line mode)
    // $     the end of text (or end-of-line with multi-line mode)
    // \A    only the beginning of text (even with multi-line mode enabled)
    // \z    only the end of text (even with multi-line mode enabled)
    // \b    a Unicode word boundary (\w on one side and \W, \A, or \z on other)
    // \B    not a Unicode word boundary


    // a*b* matches any number of a’s followed by any number of b’s, e.g. a, aaab
    // (ab)* matches any number of ab’s, e.g. ab, abababab, 
}

