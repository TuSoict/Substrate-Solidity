use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::Body;
// use reqwest::Body;
use serde::{Serialize, Deserialize};

pub const API_KEY:&str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoic2VydmljZV9yb2xlIiwiaWF0IjoxNjQwMDgzNDY2LCJleHAiOjE5NTU2NTk0NjZ9.AzX8sgU8HrdMccrUECWxuUYDok2UMWrUqm74W7L4sr8";

#[derive(Debug,Serialize,Deserialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub age: u8,
    pub math_point: f64,
    pub code_point: f64,
}
impl  std::fmt::Display for Student{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(
            f, 
            "{} -- {}  -- {} --     {}     --     {}",
        self.id,self.name,self.age,self.math_point,self.code_point
        )
    }
}

#[tokio::main]
pub async fn read() -> Result<(), Box<dyn std::error::Error>> {
    // pull outt token and key from env variable
    let api_key = API_KEY.to_string();
    // create a new header map to be used as reqwest header
    let mut headers = HeaderMap::new();
    // populate headers map with token and key
    // parse the string into  HeaderValue using parse()
    headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());


    let client = reqwest::Client::new();
    let resp = client
        .get("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?select=*")
        .headers(headers)
        .send()
        .await?;

    //  deserializing the response into a vector of Student
    let resp_json = resp.json::<Vec<Student>>().await?;
    
    println!(" Id  -- Student name -- Age -- Math point -- Code point");
    for student in resp_json.iter() {
        println!("{}", student);
    }

    Ok(())

}





#[tokio::main]
pub async fn insert(id:i32,name:String,age:u8,math_point:f64,code_point:f64) -> Result<(), Box<dyn std::error::Error>> {
    

    let student = Student{
        id: id.into(),
        name: name.into(),
        age: age.into(),
        math_point: math_point.into(),
        code_point: code_point.into(),
    };


    let api_key = API_KEY.to_string();
    // create a new header map to be used as reqwest header
    let mut headers = HeaderMap::new();
    // populate headers map with token and key
    // parse the string into  HeaderValue using parse()
    headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());


    let client = reqwest::Client::new();
    client
    .post("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?select=*")
    .json(&student)
    .headers(headers)
        
    // .header("","")
    // .body(Body::from(r#"{"library":"hyper"}"#))
        
    .send()
    .await?;

    Ok(())

}


#[tokio::main]
pub async fn delete(id:i32) -> Result<(), Box<dyn std::error::Error>> {
   
    let api_key = API_KEY.to_string();
    // create a new header map to be used as reqwest header
    let mut headers = HeaderMap::new();
    // populate headers map with token and key
    // parse the string into  HeaderValue using parse()
    headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());
    
    let client = reqwest::Client::new();
    
    client
    .delete(format!("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?id=eq.{}",id))  
    .headers(headers)
        
    // .header("","")
    // .body(Body::from(r#"{"library":"hyper"}"#))
        
    .send()
    .await?;
    Ok(())

}


#[tokio::main]
pub async fn update(id:i32,name:String,age:u8,math_point:f64,code_point:f64) -> Result<(), Box<dyn std::error::Error>> {
   
    let api_key = API_KEY.to_string();
    // create a new header map to be used as reqwest header
    let mut headers = HeaderMap::new();
    // populate headers map with token and key
    // parse the string into  HeaderValue using parse()
    headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());
    
    


    let client = reqwest::Client::new();
    
    client
        .patch(format!("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?id=eq.{}",id))  
        .headers(headers)
        .header("Content-Type","application/json")
        .header("Prefer","return=representation")
        .body(Body::from(format!(r#"{{"name":"{}","age":"{}", "math_point":"{}","code_point":"{}"}}"#, 
                                name,age,math_point,code_point)))

        .send()
        .await?;

    Ok(())

}

















//  Test 


// #[tokio::main]
// pub async fn test_read() -> Result<(), Box<dyn std::error::Error>> {
//     // pull outt token and key from env variable
//     let api_key = API_KEY.to_string();
//     // create a new header map to be used as reqwest header
//     let mut headers = HeaderMap::new();
//     // populate headers map with token and key
//     // parse the string into  HeaderValue using parse()
//     headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
//     headers.insert("apikey", api_key.parse().unwrap());
    
    
//     let view = headers.get_all("host");

//     let iter = view.iter();
//     println!("{:?}",iter);


//     let client = reqwest::Client::new();
//     let resp = client
//         .get("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?select=*")
//         .headers(headers)
//         .send()
//         .await?;
    
//     println!("{:#?}", resp);
//     //  deserializing the response into a vector of Student
//     let resp_json = resp.json::<Vec<Student>>().await?;
    
//     println!(" Id  -- Student name -- Age -- Math point -- Code point");
//     for student in resp_json.iter() {
//         println!("{}", student);
//     }

//     Ok(())

// }


// pub async fn test_insert(id:i32,name:String,age:u8,math_point:f64,code_point:f64) -> Result<(), Box<dyn std::error::Error>> {
    

//     let student = Student{
//         id: id.into(),
//         name: name.into(),
//         age: age.into(),
//         math_point: math_point.into(),
//         code_point: code_point.into(),
//     };


//     let api_key = API_KEY.to_string();
//     // create a new header map to be used as reqwest header
//     let mut headers = HeaderMap::new();
//     // populate headers map with token and key
//     // parse the string into  HeaderValue using parse()
//     headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
//     headers.insert("apikey", api_key.parse().unwrap());
    
    
//     let view = headers.get_all("host");

//     let iter = view.iter();
//     println!("{:?}",iter);


//     let client = reqwest::Client::new();
//     let resp = client
//         .post("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?select=*")
//         .json(&student)
//         .headers(headers)
        
//         // .header("","")
//         // .body(Body::from(r#"{"library":"hyper"}"#))
        
//         .send()
//         .await?;

//     println!("{:#?}", resp);
//     Ok(())

// }


// #[tokio::main]
// pub async fn test_delete(id:i32) -> Result<(), Box<dyn std::error::Error>> {
   
//     let api_key = API_KEY.to_string();
//     // create a new header map to be used as reqwest header
//     let mut headers = HeaderMap::new();
//     // populate headers map with token and key
//     // parse the string into  HeaderValue using parse()
//     headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
//     headers.insert("apikey", api_key.parse().unwrap());
    
    
//     let view = headers.get_all("host");

//     let iter = view.iter();
//     println!("{:?}",iter);

//     let client = reqwest::Client::new();
    
//     let resp = client
//         .delete(format!("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?id=eq.{}",id))  
//         .headers(headers)
        
//         // .header("","")
//         // .body(Body::from(r#"{"library":"hyper"}"#))
        
//         .send()
//         .await?;

//     println!("{:#?}", resp);
//     //  deserializing the response into a vector of ABC
//     let resp_json = resp.json::<Vec<Student>>().await?;
//     println!("Response Json:{:#?}",resp_json);

//     Ok(())

// }


// #[tokio::main]
// pub async fn test_update(id:i32) -> Result<(), Box<dyn std::error::Error>> {
   
//     let api_key = API_KEY.to_string();
//     // create a new header map to be used as reqwest header
//     let mut headers = HeaderMap::new();
//     // populate headers map with token and key
//     // parse the string into  HeaderValue using parse()
//     headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());
//     headers.insert("apikey", api_key.parse().unwrap());
    
    
//     let view = headers.get_all("host");

//     let iter = view.iter();
//     println!("{:?}",iter);

//     let client = reqwest::Client::new();
//     let a = "Sinh vien C";
    
//     let resp = client
//         .patch(format!("https://ynddcvqqfeikgkmcaqtt.supabase.co/rest/v1/Student?id=eq.{}",id))  
//         .headers(headers)
//         .header("Content-Type","application/json")
//         .header("Prefer","return=representation")
//         .body(Body::from(format!(r#"{{"name":"{}"}}"#, a)))
        
//         .send()
//         .await?;

//     println!("{:#?}", resp);
//     //  deserializing the response into a vector of ABC
//     let resp_json = resp.json::<Vec<Student>>().await?;
//     println!("Response Json:{:#?}",resp_json);

//     Ok(())

// }
