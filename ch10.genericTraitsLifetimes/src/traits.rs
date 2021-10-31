#![allow(unused)]
fn main(){
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username:String,
        pub content: String,
        pub reply:bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self)->String{
            format!("{}: {}", self.username, self.content)
        }
    }


    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as probably already know "),
        reply:false,
        retweet:false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let number_list = vec![34, 50,25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list =vec!['a','b','c', 'd'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter(){
            if item > largest {
                largest = item;
            }
        }
        largest
    }


