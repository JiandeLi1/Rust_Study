fn main(){
    println!("Hello!");
    another_function();
    print_labled(12,'z');
    scope();
    println!("the function five return {}", five());
    println!("The function push_one test: {}", plus_one(5));
}

//The other function call in main can be anywhere, before or after the main()
fn another_function(){
    println!("Another function!");
}

//
fn print_labled(value:i32, uint_label:char){
    println!("the value is {}, and the label is {}.", value, uint_label);
}

//fn body using spoce
fn scope(){
    let x=5;
    let y={
        //let x=3// if do it, return x=4
        x+1//if ; here, will not return
    };
    println!("y is: {}", y);
}

//Rust arrow function
fn five()->i32{
    5
}

//arrow function with parameter
fn plus_one(x:i32)->i32{//->i32 means need to return a variable with type i32, if add ; in x+1 tail, means not thing return, there is a error 
    x+1
}