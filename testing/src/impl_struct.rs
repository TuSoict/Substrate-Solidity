pub struct Human {
  pub name: String,
  pub height: u32,
  pub weight: u32,
  pub is_male: bool
}

impl Human {
  pub fn detail(&self) {
    println!("Tên là {}", self.name);
    println!("Chiều cao {}", self.height);
    println!("Cân nặng {}", self.weight);
  }

  pub fn sleep(&self) {
    println!("{} đang ngủ zzzzzZ", self.name)
  }

  pub fn hobby(&self) {
    if self.is_male {
      println!("{} thích con gái", self.name);
    }
    else {
      println!("{} thích con trai", self.name);
    }
  }
}