#![allow(unused)]
fn main() {
    #[derive(Debug)]
    enum UsState{
        NY,
        NJ,
        MA,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin:Coin)-> u8{
        match coin {
            Coin::Penny=>{
                println!("Penny!");
                1
            },
            Coin::Nickel=>{
                println!("Nickel!");
                5
            },
            Coin::Dime=>{
                println!("Dime!");
                10
            },
            Coin::Quarter(state)=>{
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::NY));

    let coin = Coin::Penny; 
    let mut count = 0;
    match &coin {//& to pervent ownership transfer
        Coin::Quarter(state)=>println!("State quarter from {:?}!", state),
        _=>count+=1,
    }
    println!("{}", count);
    
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    }else{
        count+=1
    }
    println!("{}", count);
}
