#![allow(unused)]
fn main(){
    let mut s1 = String::from("foo");
    let s2 = "abc";
    s1.push_str(s2);

    println!("{}", s1);

    let s3 = String::from("Hello");
    let s4 = String::from(" world!");

    let s5 = s3+&s4;// s3 not work again, this like fn add(self, s: &str) -> String {}, s3 can not &. 
    //So, we can s3 need to a String, s4 need to a &str, we can not do that with two Strings.
    //&s4 was coercion to &str from &String
    //When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].
    //Because of using + just calling fn add(self, s: &str) -> String {}, s3 ownership will transfer to s5
    println!("{}-{}", s4, s5);


    let s6=String::from("a");
    let s7=String::from("b");
    let s8=String::from("c");

    // let s9 =s6 + "-" + &s7 + "-" + &s8; // Because we using +, means using add, s6 ownership will transfer, and s7 and s8 need to  use &.
    // println!("{}", s9);

    //We have the better way and doesn't transfer the ownership
    let s9 = format!("{}-{}-{}", s6,s7,s8);
    println!("{}", s9);
    //The format! macro works in the same way as println!, but instead of printing the outputto the screen, it returns a String with the contents.

    //This is error, we can not do indexing into strings.
    // let s10 = String::from("hello");
    // let s11 = s10[0]

    //A string is a wrapper over a vec<u8>. Example,
    let len = String::from("Hola").len();
    println!("{}", len);

    
    let hello = String::from("Здравствуйте");
    println!("{}", &hello.len());// Answer is 24, in UTF-8, each unicode scalar value in that string takes 2 bytes of strage.
    println!("{}", &hello[0..4]);//Зд
    //println!("{}", &hello[0..1]);//failed, cause each unicode is 2 bytes!

    for c in "नमस्ते".chars() {
        println!("{}", c);
        // न
        // म
        // स
        // ्
        // त
        // े
    }


    for b in "नमस्ते".bytes() {
        println!("{}", b);
        // 224
        // 164
        // 168
        // 224
        // 164
        // 174
        // 224
        // 164
        // 184
        // 224
        // 165
        // 141
        // 224
        // 164
        // 164
        // 224
        // 165
        // 135
    } 


    //Rust make different choices about how to present this complexity to the programmer.
    //Rust has chosen to make the correct handling of String data the default behavior for all Rust programs,
    //which means programmers have to put more thought into handling UTF-8 data upfront.
}