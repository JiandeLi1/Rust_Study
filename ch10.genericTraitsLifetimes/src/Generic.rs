#![allow(unused)]
#[derive(Debug)]

    //Performance of code using generics
    //if we have two enum
    //When Rust compiles this code, it performs monomorphization.
    //During that process, the compiler reads the values that have been used in Option<T> instances and identifies
    //two kinds of Option<T>: one is i32 and the other is f64. 
    // let integer = Some(5);
    // let float = Some(5.0);

    //That is like
    enum Option_i32{
        Some(i32),
        None,
    }
    enum Option_f64{
        Some(f64),
        None,
    }


struct Point<T, U>{
    x:T,
    y:U,
}
impl<T, U> Point<T, U> {
    //The two Point struc may have differece type parameter
    //V from point 1, W from point 2
    //The inside function need take differece generic from outer 
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}
fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // println!("The largest for number_list is {}", largest(&number_list));
    
    //Two typies in a struc
    // let both_integer = Point {x:5, y:10};
    // let both_float = Point {x:2.0, y:4.0};
    // let integer_and_float = Point {x:5, y:4.0};
    // println!("{:?}{:?}{:?}", both_integer, both_float, integer_and_float);

    let p1 = Point {x:5, y:10.4};
    let p2 = Point {x: "Hello", y:'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x={}, p3.y={}", p3.x, p3.y);


    let interger = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);

    


}



// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }