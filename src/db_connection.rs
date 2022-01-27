extern crate postgres;

use postgres::{Client, NoTls};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Client {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set").to_owned();
	
    let db_url_str : &str = &database_url[..];

    Client::connect(db_url_str, NoTls).unwrap()
}

