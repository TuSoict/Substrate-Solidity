
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    

#[derive(Debug, Clone)]
     struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn hidden(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
     trait Summary {
        fn summarize(&self) -> String;

        fn hidden(&self) -> String;
    }


    /**
     * Super trait 
     */

     trait Person {
        fn name(&self) -> String;
    }
    
    // Person is a supertrait of Student.
    // Implementing Student requires you to also impl Person.
    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }
    
     trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    trait MyDebug : std::fmt::Debug {
        fn my_subtrait_function(&self);
    }


    impl MyDebug for Tweet {
        fn my_subtrait_function(&self) {

        }
    }

pub fn run_sample () {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }; 


    println!("print tw {:?}",tweet);

    println!("1 new tweet: {}", tweet.summarize());

    let tw_clone = tweet.clone();
    println!("tweet clone: {}", tw_clone.summarize());

}