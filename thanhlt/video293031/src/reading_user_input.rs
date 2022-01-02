use std::io;
pub fn reading_user_input(){

    let mut input = String::new();
    
    println!("Chao moi nguoi");

    match io::stdin().read_line(&mut input){
        Ok(_)=>{

            println!("Wow! Ban noi: {}",input.to_uppercase());
        },
        Err(e)=>println!("Oops! Toang roi ban oi: {}",e)  // chua tim duoc truong hop loi 
    }


}