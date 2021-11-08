// use crate::List::{Cons, Nil};

// fn main() {
//     // let b = Box::new(5);
//     // println!("b = {}", b);



//     let list =Cons(1, Cons(2, Cons(3,Nil)));
// }
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
#![allow(unused)]
fn main() {
    // let list = Cons(1,
    //     Box::new(Cons(2,
    //         Box::new(Cons(3,
    //             Box::new(Nil))))));

    // let x = 5;
    // let y = &x;
    
    // assert_eq!(5,x);
    // assert_eq!(5,*y);
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T>{
        fn new(x:T)->MyBox<T>{
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

//     enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1,
//         Box::new(Cons(2,
//             Box::new(Cons(3,
//                 Box::new(Nil))))));
// }
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    //&m &MyBox<String>
    //deref &String
    //deref &str
    hello(&m);
    hello(&(*m)[..]);
    hello("Rust");

}

fn hello(name:&str){
    println!("Hello, {}", name);
}
