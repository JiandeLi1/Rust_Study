fn main(){}

// #[cfg(test)]
// mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
    // #[test]//wil fail
    // fn another(){
    //     panic!("Make this test fail!");
    // }

    #[derive(Debug)]
    struct Rectangle {
        width:u32,
        height:u32,
    }

    impl Rectangle {
        fn can_hold(&self, other:& Rectangle)->bool{
            self.width > other.width && self.height >other.height
        }
    }
// }

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {width:8, height:7};
        let smaller = Rectangle {width:5, height:1};
        assert!(larger.can_hold(&smaller));
    }
}

pub fn add_two(a:i32)->i32{
    a+2
}

#[cfg(test)]
mod tests2 {
    use super::*;
    #[test]
    fn it_adds_two(){
        assert_eq!(4, add_two(2));
        //assert_eq! and assert_ne! macros use the operators == and !=, respectively!
        //we can add a more useful failure message in this case would print the value we got from the
        //greeting function.
        //assert!(
        //  result.contains("Carol");
        //  "Greeting did not contain name, value was `{}`",
        //  result
        //)
    }
}

pub struct Guess {
    value:i32,
}

impl Guess {
    pub fn new(value:i32)->Guess{
        // if value < 1 || value > 100{
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }else if value>100{
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess{
            value
        }
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;
    #[test]
    #[should_panic(expected="Guess value must be less than or equal to 100")]//expected the wrong
    fn greater_than_100(){
        Guess::new(200);
    }
}

#[cfg(test)]
mod tests4 {
    #[test]
    fn it_works()->Result<(), String>{
        if 2+2==4{
            Ok(())
        }else{
            Err(String::from("two plus two does not equal four!"))
        }
    }
}

/*
cargo test --help will displays the options you 
can use with cargo test, and runing cargo test -- help 
displays the options you can use with cargo test, and 
running cargo test -- help displays the options you can 
use after the separator --

if you run multiple tests , by default they run in parallel using threads.
We need to make sure that they are not dependent on each other, because 
they might interfere with each other, even if the code is right, but the result
might wrong.

There are two ways to prevent it, first, make sure all test not dependent on each other,
or run test once each time.
cargo test -- --test-threads=1

if test pass, defual will not show the output. Only show pass or failed,
but if you want, you can run cargo test -- --nocapture

If you only want to test one, you can do cargo test name,
we can do test some for prefix, like add_one, add_two, we can only do
cargo test add
*/


#[test]
#[ignore]//do it for ignore this if using cargo test, if we want to test it only, use cargo test -- --ignored 
fn expensive_test(){
    //code
}

/*
#[cfg(test)]
If run cargo test, it work. If run cargo build, will not work.
cfg=configuration
*/

