use dotenv::dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::env;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct ABC{
    id: i32,
    title: String,
    
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // load .env file into std::env
    dotenv().ok();
    // pull outt token and key from env variable
    let api_key = env::var("ABC_API_KEY")?;
    let api_token = env::var("ABC_API_TOKEN")?;
    // create a new header map to be used as reqwest header
    let mut headers = HeaderMap::new();
    // populate headers map with token and key
    // parse the string into  HeaderValue using parse()
    headers.insert(AUTHORIZATION, format!("Bearer {}",api_token).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());
    
    
    let view = headers.get_all("host");

    let mut iter = view.iter();
    println!("{:?}",iter);


    let client = reqwest::Client::new();
    let resp = client
        .get("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/ABC?select=*")
        .headers(headers)
        .send()
        .await?;

    println!("{:#?}", resp);
    //  deserializing the response into a vector of ABC
    let resp_json = resp.json::<Vec<ABC>>().await?;
    println!("Response Json:{:#?}",resp_json);
    Ok(())

}