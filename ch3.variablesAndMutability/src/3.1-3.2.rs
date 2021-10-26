//use std::num::Wrapping;
use std::io;
fn main() {
    // let mut x = 5;//Needing mut to make the variable change the value
    // println!("The value of x is :{}", x);
    // x=6;
    // println!("The value of x is: {}", x);


    // let x=5;//If not let, will not do shadowing!!!!!!   
    // let x=x+1;
    // {
    //     let x=x*2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    // println!("The value of x in the inner scope is: {}", x);

    //let guess="42".parse().expect("Not a number!"); //for using parse, you need to give guess a type to converse
    // let guess : u32="42".parse().expect("Not a number!");
    // println!("{}", guess);

    // let x : u8=256;
    // println!("{}",x);//overflow, there is a error!

    //For char, each char is 4 bytes, can be chinese, korean, ..., emoji...
    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';
    // println!("{}{}{}",c,z,heart_eyed_cat);

    // let tup : ( i32, f64, u8) = (500, 6.4, 1);
    // //read by index
    // println!("{} {} {}", tup.0,tup.1,tup.2);
    // //destructuring
    // let (x,y,z) = tup;
    // println!("{} {} {}", x,y,z);

    let a = [1,2,3,4,5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin().
        read_line(&mut index).
        expect("Failed to read line");
    
    let index: usize = index.trim().parse().expect("Index entered was not a number!");

    let element = a[index];

    println!("The value of the element at index {} is {}", index, element);

}
