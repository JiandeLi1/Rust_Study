#![allow(unused)]
fn main(){
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Red"),50);

    //The way to make the hashmap by my self
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let t1 = String::from("Blue"); 
    let t2 = String::from("Red");
    let s1=scores.get(&t1);
    let s2 =scores2.get(&t1);
    let s3=scores.get(&t2);
    let s4=scores2.get(&t2);
    //hashMap returns Option
    println!("{:?}",s1);
    println!("{:?}",s2);
    println!("{:?}",s3);
    println!("{:?}",s4);

    //iterating the hashmap (The order is random)
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //Updating a Hash map
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    //Only inserting a Value if the key has no value
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(60);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    //Updating a Value Based on the Old Value
    let text = "hello world wonderful world!";

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);// or_insert method actually returns a mutable reference (&mut v) to the value for this key
        *count+=1;
        //Here we store that mutable refernce in the count variable, 
        //so in order to assign to that value,
        //we must first dereference count using * goes out of scope at the end of the for loop
    }

}