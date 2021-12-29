    // mot trait tuong tu nhu interface

    pub trait Summary {

        // phuong thuc mac dinh

        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Day la method deffault {}...)", self.summarize_author())
        }

       // fn hotnewpress(&self) -> String;  Khi implement bat buoc phai viet lai body cho cac method nay
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

        fn summarize(&self) -> String {
            format!("{}: {} Day co phai la 1 retweet khong: {} ", self.username, self.content, self.retweet)
        }
    }

    // không thể gọi triển khai mặc định từ một triển khai ghi đè của cùng một phương thức  ?

    // ====================

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.headline)
        }

        fn summarize(&self) -> String {
            format!("{}, by {} ({}) - Thu goi: {}", self.headline, self.author, self.location, self.summarize_author())
        }
        
    }

    // Traits được dùng như 1 tham số của hàm
   
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // trait noud syntax

    // neu ca 2 tham so cung 1 kieu trait thi dung cach nay

    pub fn notify_trait_boud<T: Summary>(item1: &T, item2: &T) {
        println!("Hai tham so cung 1 kieu du lieu: {} {}", item1.summarize(), item2.summarize());
    }
    
    // neu 2 tham so cung khac kieu trait thi dung cach nay

    pub fn notify_two_trait(item1: &impl Summary, item2: &impl Summary) {
        println!("Hai tham so khac kieu du lieu:  {} {}", item1.summarize(), item2.summarize());
    }


    fn main() {

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: true,
        };

       //  println!("Implement sumary for Tweet - 1 new tweet: {}", tweet.summarize());


        let articale = NewsArticle {
            headline: String::from("VietNam won championship!"),
            location: String::from("ASIA"),
            author: String::from("Biden"),
            content: String::from("This is title of Article")
        };

        let articale1 = NewsArticle {
            headline: String::from("VietNam won championship!"),
            location: String::from("ASIA"),
            author: String::from("Biden"),
            content: String::from("This is title of Article")
        };

      //   println!("Implement sumary for Articale: {}", articale.summarize());


      //   notify(&articale);

        // // trait bound

         notify_trait_boud(&articale, &articale1);

        // //tham so dung 2 loai du lieu khac nhau vi du 2 struct khac nhau

      //  notify_two_trait(&articale, &tweet);
    }

    