use std::collections::HashMap;
pub fn hash_map(){

    let mut TheThao = HashMap::new();
    let value_a = 10;
    let key_b = "Key";
        // add value // duoc luu vao dau??
    TheThao.insert("Bong Chuyen", 5); //  so1
    TheThao.insert("Tenis", 6);
    TheThao.insert("Cau Long", 7);  // so 2
    TheThao.insert("Da Cau", 8);
    TheThao.insert("Run", 10);
    TheThao.insert(key_b, value_a); 

    
    println!("key_b: {} value_a: {}",key_b,value_a);
    // // find length hashmap

     println!("Co bao nhieu mon the thao vay anh trai?  {}  Em nhe!",TheThao.len());


    // // get a single value

    // match TheThao.get("Da Bong"){
    //     Some(TheThao)=>println!("Ban co {} diem Da Bong",TheThao),
    //     None=>println!("Ban khong hoc mon Da Bong")

    // }
    // match TheThao.get("Da Cau"){
    //     Some(TheThao)=>println!("Ban co {} diem Da Cau",TheThao),
    //     None=>println!("Ban khong hoc mon Da Bong")

    // }
    // match TheThao.get("Da Cau"){
    //     Some(TheThao)=>println!("Ban co {} diem Da Cau",TheThao),
    //     None=>println!("Ban khong hoc mon Da Bong")

    // }
    // // remove a value

    //TheThao.remove("Cau Long");

    // // Loop through Hashmap (key,value)
//let mut iii =3;

    for(bomon,mon) in TheThao.iter_mut(){
        *mon = *mon +2;
        println!("Mon '{}' cua toi la {} !",bomon,mon);

    }

    println!("TheThao gia tri:  {:?} ",TheThao);
    // // check for Value

    // println!("Bo mon Da Bong co khong ? {}", TheThao.contains_key("Da Bong"));
    // println!("Bo mon Run co khong ? {}", TheThao.contains_key("Run"));

}

// chot:  (ghi de gia tri) or (lay element va tang gia  tri cho no)!!!