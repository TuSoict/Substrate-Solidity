use std::io;
use std::collections::HashMap;

pub struct Account {
    pub username: String,
    pub password: String,
}

pub struct Accounts{
    pub class: HashMap<String, Account>,
}    

impl Accounts {
    pub fn new() -> Self{
        Self {class: HashMap::new()}
    }

    pub fn add(&mut self, account: Account) {
        self.class.insert(account.username.to_string(), account);
    }

    pub fn all_accounts(&self) -> Vec<&Account> {
        self.class.values().collect()
    }
}


pub fn register() -> Account{
    let mut s1 = Account{
        username: String::new(),
        password: String::new(),
    };

    println!("\nHãy nhập tên đăng kí");
    io::stdin()
        .read_line(&mut s1.username)
        .expect("Failed to read line");
    
    println!("Hãy nhập mật khẩu đăng kí.");

    io::stdin()
        .read_line(&mut s1.password)
        .expect("Failed to read line");
    s1
}

fn add_account(accounts: &mut Accounts){
    let mut is_exist = false;
    let s2 = register();
    loop {
        for account in accounts.all_accounts(){
        if s2.username == account.username {
            is_exist = true;    
        }
    }
    if is_exist == false{
        println!("Đăng kí thành công, tài khoản của bạn có user là:{}và pass là: {}", &s2.username, &s2.password);
        break;
    }
    else {
        println!("tài khoản đã tồn tại hãy nhập lại");
        break;
    }
  }
    accounts.add(s2);
}

pub fn check_login(accounts: &mut Accounts){
    let mut _a = false;
    let mut s3 = Account{
        username: String::new(),
        password: String::new(),
    };
    println!("\nHãy nhập tên đăng nhập");
    io::stdin()
        .read_line(&mut s3.username)
        .expect("Failed to read line");
            
    println!("Hãy nhập mật khẩu.");
    io::stdin()
        .read_line(&mut s3.password)
        .expect("Failed to read line");
    loop {
        for account in accounts.all_accounts(){
            if (s3.username == account.username) && (s3.password == account.password) {
                _a = true;
                // println!("Đăng nhập thành công");
                // break;
            }
        }
        if _a == true {
            println!("Đăng nhập thành công");
            break;
        }
        else {
            println!("tài khoản không chính xác hãy thử lại");
            check_login(accounts);
            break;
        }
    }
}

pub fn home() {
    let mut accounts = Accounts::new();
        loop{
            println!("");
            println!("=====Home=====");
            println!("1. Register");
            println!("2. Login");
            println!("==============");
            println!("Please enter your choice");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading");
            let input: u8 = match input.trim().parse() {
                Ok(s) => s,
                Err(_) => continue,
            };
            match input{
                1 => {
                        add_account(&mut accounts);
                        continue;
                    },
                2 => {  if accounts.class.is_empty(){
                        println!("\nVui lòng đăng kí tài khoản trước khi đăng nhập");
                        continue;
                     }
                        else {check_login(&mut accounts);
                        break;}
                    },
                _ => {
                        println!("\n Choice not match");
                        continue;},
            }
        }
}