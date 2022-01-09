pub fn p_matching(){
    let number =15;
    let name1 = "Trang Ty";
    let name = String::from("TrangQuynh");
    let name2 = "TrangQuynh".to_string();
    let name3 = "TrangQuynh".to_string();
// giong 1 swith case...

// So  integer

    match number {
           1=> println!("No la so 1 "),
           2=> println!("No la so 2"),
      10|11|13|23 =>println!("No thuoc mot trong cac truong hop 10|11|13_23 "), // chon neu co
      2..=20 => println!("So {} No thuoc day so nay",number),
       _=> println!("Toi khong biet con so do la gi!")
    }
// string :: so sanh gia tri
    match name{
        //&name =>println!("Oops, khong chinh xac roi TrangQuynh2 "),
        name2 =>println!("Oops, khong chinh xac roi TrangQuynh2 "),
        name3 =>println!("Oops, khong chinh xac roi TrangQuynh3 "),
        //"Trang Ty"=> println!("{} Doan chuan roi!",name),
        _=> println!("Toi khong biet chuyen gi")
    }

    let boolean = false;
// boolean
    let binary = match boolean
    {
      
        false =>0,
        true =>1,

    };

    println!("{}  ->  {}", boolean,binary);
    
}