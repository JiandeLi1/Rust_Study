use std::io;
fn main(){
    println!("++++++++++++++++++++++++++++++++++++++++++++++++++");
    println!("Enter 1, Conversing Celsius/Fahrenheit");
    println!("Enter 2, nth Fibonacci number");
    println!("Enter 3, 'The Twelve Days of Christmas's lyrics!");
    println!("Other number or char will end the progarming!");
    println!("++++++++++++++++++++++++++++++++++++++++++++++++++");
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Reading option fail!!!");
    let option : i32 = option.trim().parse().expect("Enter option not a number!");
    match option{
        1=> converterDeg(),
        2=>{
            println!("You enter 2");
        },
        3=>{
            println!("You enter 3");
        },
        _=>{
            println!("Doesn't has option for you enter number!");
            break;
        }
    }
}

fn converterDeg(){
    println!("Hi!");
}