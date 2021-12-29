enum Direction {
    Up,
    Down,
    Left,
    Right
}

enum Order {
    Pending,
    Processing,
    Shupment,
    Complete
}

// Khai bao enum trong enum

enum Female {
    a(String, i32),
    
}

// #[derive(Debug)]

enum Gioitinh {
    Male(String), 
    Female(Female),               
    Neutral(i32, i32),
}

fn main() {
   let player_direction:Direction = Direction::Left;

   match player_direction {
       Direction::Up => println!("We are heading up"),
       Direction::Down => println!("We are going all the way down"),
       Direction::Left => println!("Lerft is right!"),
       Direction::Right => println!("Moving toward the right"),
   }

   let order_status:Order = Order::Processing;

   match order_status {
       Order::Pending => println!("Pending to checkout"),
       Order::Processing => println!("Processing "),
       Order::Shupment => println!("Shupment"),
       Order::Complete => println!("Complete"),
   }

  // Kiem tra kieu enum trong enum

  let nam = Gioitinh::Male("thể dục thể thao".to_string());
  let huong = Gioitinh::Female(Female::a(String::from("đánh son"), 2));
  let hung = Gioitinh::Neutral(2,5);

  kiemtra(nam);
  kiemtra(huong);
  kiemtra(hung);
}

fn kiemtra(sex: Gioitinh) {
    match sex {
        Gioitinh::Male(yeuthich1) => println!("Yêu thích là {}", yeuthich1),
      //  Gioitinh::Female(girl) => println!("Sở thích là {:?} và chải tóc mỗi ngày", girl::Female::a),
      Gioitinh::Female(Female::a(sothich, b)) => println!("Sở thích là {} và chải tóc {} lan mỗi ngày", sothich, b), //Gioitinh::Female(Female::a(sothich, b)) Cách gọi dữ liệu Enum trong Enum
        Gioitinh::Neutral(a, b) => println!("Tớ thích tắm {} lần/ngày", a + b),
    }
}
