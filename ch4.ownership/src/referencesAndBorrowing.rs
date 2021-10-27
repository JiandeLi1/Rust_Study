fn main(){
    let s1=String::from("Hello");
    let (s2, l) = cal_Str_len(&s1);
    println!("{} {} {}", s1, s2, l);

    let mut s = String::from("hello");
    // {
    //     let r1=&mut s;
    //     println!("{}", r1);
    //     //r1 will goes out of scope here, wso we can make a new reference with no problems.
    // }

    // let r2=&mut s;
    // println!("{}", r2);

    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s;//mutable reference shile we have an immutable one.
    println!("{} {}", r1, r2);
    //variables r1 and r2 will not be used after this point

    let r3= &mut s;// no problem
    // println!("{} {} {}", r1, r2, r3);
    println!("{}", r3);
    let r4=dangle();
    println!("{}", r4);
}

fn cal_Str_len(s: &String)->(&String ,usize){
    (s, s.len())
}
fn dangle()->String{
    let s = String::from("A");
    //&s //we return a reference to the String, s
    //refercence doesn't has ownership, can not transfer it! So, we need to remove the &
    s
}//Here, if s is not tranfer the ownership,  s  goes out of scope, and is dropped. Its memory goes away.

