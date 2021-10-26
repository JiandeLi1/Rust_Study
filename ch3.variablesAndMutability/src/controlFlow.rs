fn main(){
    let condition = true;
    let condition2=3;
    if condition {
        println!("consition is true!");
    }else{
        println!("condition is false!");
    }
    //Condition must be bool, if not bool, throw error
    // if condition2 { //error[E0308]: mismatched types. expected `bool`, found integer
    //     println!("condition 2 is true!");
    // }else{
    //     println!("condition 2 is false!");
    // }
    //Using if in let statement
    let number = if condition { 5 } else { 6 };
    println!("The number is {}", number); 

    let n=if condition {
        "five"// if here is integer, inside the else also need to be integer. because n variables need to have olny one type
    }else{
        "six"
    };

    println!("n = {}", n);

    loop {
        println!("HI");
        //continue;// ueing continue and nothing here, will print forever!
        break; //break it!
    }

    //Two layer loop
    let mut count = 0;
    'counting_up: loop {
        println!("count={}", count);
        let mut remaining = 10;
        loop {
            println!("remaining={}", remaining);
            if remaining == 9 {
                break;
            }
            if count==2 {
                break 'counting_up;//outer loop break, inside also break
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
    //There is multiple loop
    //each inside loop subtract 1 from remaining
    //each outer loop will adding 1 from count
    //output:
    //count=0
    //remaining=10
    //remaining=9
    //count=1
    //remaining=10
    //remaining=9
    //count=2
    //remaining=10
    //End count = 2

let mut c = 0;
let result = loop{
    c+=1;
    if c==10 {
        break c*2;
    }
};

println!("The result is {}", result);

let mut num = 3;
while num != 0 {
    println!("{}!", num);
    num=num-1;
}
println!("{}!", num);

let arr=[1,2,3,4,5,6,7,8,9];
for n in arr.iter() {
    println!("{}", n);
}


}