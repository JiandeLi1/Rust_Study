enum List {
    Cons(i32, Rc<List>),//Cons(i32, Box<List>), can't not share
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main(){
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // //a.clone()//Deep clone
    // let b = Cons(3, Rc::clone(&a));//Not deep clone
    // let c = Cons(4, Rc::clone(&a));

    let a = Rc::new(Cons(5, Rc::new(Cons(19, Rc::new(Nil)))));
    println!("Count after creating a ={}", Rc::strong_count(&a));//1

    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("Count after creating b ={}", Rc::strong_count(&a));//2

    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c ={}", Rc::strong_count(&a));//3
    }

    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));//2
}

//Rc::clone will add refer times, not deep clone
//clone will deep clone 
//Rc<T> inmutable refer, let you only read the share data in differece part.