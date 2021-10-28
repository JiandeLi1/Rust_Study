//#[derive(Debug)]
fn main() {
    let mut v = vec![1,2,3,4,5];
    //let not_work=&v[100]; Out of the range
    let doesnt_work = &v.get(100);// return a none
    match &doesnt_work {
        None=>println!("This is None!"),// There is a none
        Some(&doesnt_work)=>println!("work, {}",doesnt_work),
    } 
    v.push(6); //work
    let first = &v[0];
    //v.push(6); //because of no enough space, to add new element, the first element will go aways, and apply new space to store the first element
    println!("the first element: {}", first);

    for i in &mut v{
        *i+=50;// change the v
    }

    for i in &v {
        println!("{}", i);//iterating over the values in vector
    }

    //If we need a vector to save many differece type data, we can use enum to help us

    enum Multiple {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Multiple::Int(100),
        Multiple::Float(1.1),
        Multiple::Text(String::from("HELLO")),
    ];

    for i in &row {
        match &i{
            Multiple::Int(element)=>println!("Int:{}",element),
            Multiple::Float(element)=>println!("Float:{}",element),
            Multiple::Text(element)=>println!("Text:{}",element),
            _=>println!("Nothing!"),
        }
    }
}
