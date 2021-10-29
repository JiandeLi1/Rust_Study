#![allow(unused)]
// use std::fs::File;
// use std::io::ErrorKind;
// use std::io;
// use std::io::Read;

fn main() {
    //throw the panic!
    // panic!("crash and burn");

    // let v = vec![1,2,3];
    // v[99];//panic!

    let f = File::open("hello.txt");

    //smaple one
    // let f = match f {
    //     Ok(file)=>file,
    //     Err(error)=>{
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };

    //Specified one
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc)=>fc,
    //             Err(e)=>panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error=>panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // //Better one, will know more at ch13
    // fn main() {
    //     let f = File::open("hello.txt").unwrap_or_else(|error|{
    //         File::create("hello.txt").unwrap_or_else(|error|{
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic("Problem opening the file: {:?}", other_error);
    //     }
    // });

    //let f = File::open("hello.txt").unwrap();
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // fn read_username_from_file()->Result<String, io::Error> {
    //     let f = File::open("hello.txt");
    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };
    
    //     let mut s = String::new();
    
    //     match f.read_to_string(&mut s){
    //         Ok(_)=>Ok(s),
    //         Err(e)=>Err(e),
    //     };
    
    //     //println!("{:?}", s);
    // }

    use std::fs::File;
    //use std::io::ErrorKind;
    use std::io;
    use std::io::Read;
    use std::fs;
    // fn read_username_from_file()-> Result<String, io::Error>{
    //     let mut f = File::open("hello.txt")?;
    //     let mut s = String::new();
    //     f.read_to_string(&mut s)?;
    //     Ok(s)
    // }
    //===
    // fn read_username_from_file() -> Result<String, io::Error>{
    //     let mut s = String::new();
    //     File::open("hello.txt")?.read_to_string(&mut s)?;
    //     Ok(s)
    // }
    //===
    fn read_username_from_file()-> Result<String, io::Error> {
        fs::read_to_string("hello.txt")?;
    }

    //For the ? , we only can use is when return is a Result or option or another type that implements std::ops::Try
    //The main function is special, and there are restrictions on what its return type must be. One vaild return
    //type for main is (), the other one is Return <T,E>
}


