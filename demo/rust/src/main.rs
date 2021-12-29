// use std::env;
// use dotenv::dotenv;
mod http_request;
mod reqwest;
mod regular_expression;
mod option_enum;


fn main() {
    // regular_expression::run();
    // option_enum::run3();
    // reqwest::run1();
    // Env: 
    // dotenv().ok();
    // for i in env::vars(){
    //     println!("{:?}",i);
    // }
    http_request::run();
}
