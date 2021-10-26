use std::io;//Standard library
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guessing Number Game!");
    //Rust variable usually can't change but adding mut before the variable name
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);
    loop{
        println!("Guess a number!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Can't read");
        let guess : u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("THe number you guess is : {}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal=>{
                println!("You win!");
                break;
            }
        }
    }
}
