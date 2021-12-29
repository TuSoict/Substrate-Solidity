use std::collections::HashMap;


pub fn run1() -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get("https://httpbin.org/ip1234")?;

    let body = resp.text();


    println!("{:#?}", body);

    Ok(())

}


pub fn run2() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://www.google.com/")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}


#[tokio::main]
pub async fn run3() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://docs.google.com").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}