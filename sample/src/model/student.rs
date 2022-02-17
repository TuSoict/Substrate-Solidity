use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug,Eq, Ord, PartialEq, PartialOrd,Clone)]
#[allow(dead_code)]
pub struct Student {
   pub name: String,
   pub email: String,
   pub score: Vec<u32>,
   pub mssv:u32,
   pub class_name: String,
   pub avr_score:u32
} 

#[allow(dead_code)]
#[derive(Serialize, Deserialize,Clone)]
pub struct User {
   pub username: String,
   pub password: String
}


impl Student {
   pub fn caculate_avr_score (&mut self) {
        let mut total:u32 = 0;
        let count  = self.score.len() as u32;
        
        if self.score.len() > 0 {
            for score in self.score.iter() {
                total = total + score;
            }

          self.avr_score = total/count;
        }

    }

   pub fn sort_field(&self) -> u32 {
        self.mssv
    }

    pub fn sort_avg_score(&self) -> u32 {
        self.avr_score
    }

}
