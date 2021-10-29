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
        2=> fibon(),
        3=>{
            println!("You enter 3");
        },
        _=>{
            println!("Doesn't has option for you enter number!");
        }
    }
}

fn converterDeg(){
    println!("Enter a degree (only number first):");
    let mut deg = String::new();
    io::stdin().read_line(&mut deg).expect("Can't read the degree you enter!");
    let deg : f64 = deg.trim().parse().expect("Can't read the degree!");
    println!("Enter a temperature unit (only C or F, if you enter C, C to F, else, F to C):");
    let mut unit = String::new();
    io::stdin().read_line(&mut unit).expect("Can't read the degree you enter!");
    let unit = unit.trim();
    match unit{
        "F" | "f"=>{
            let c : f64 = 5.0 / 9.0 * (deg - 32.0);  
            println!("You enter {}F, conver to C is {}C", deg, c);
        },
        "C" | "c"=>{
            let c : f64 = 9.0 / 5.0  * deg + 32.0;  
            println!("You enter {}C, conver to F is {}F", deg, c);
        },
        _=>{
            println!("You enter some thing invaild!");
        }
    }
}

fn fibon(){
    println!("Enter number fo fibon:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Can't read the number!");
    let n : u32 = n.trim().parse().expect("Can't read the number you enter!");
    println!("");
    for i in 1..n+1{
        println!("{}\t", cal_fibon(i));
    }
}

fn cal_fibon(i:u32)->u32{
    if i <=2 {
        1
    }else{
        cal_fibon(i-1)+cal_fibon(i-2)
    }
}

