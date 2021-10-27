use std::io;
//Slices
fn main(){
    let mut input = String::new();
    println!("The input: {}", input);
    io::stdin().read_line(&mut input).expect("Can't read the input!");
    let word=first_word(&input);
    println!("The first word is {}.", word);

    let a=[1,2,3,4,5];
    let slice = &a[1..3];
    println!("{}",assert_eq!(slice, &[2,3]));

}

fn first_word(s: &String)->&str{
    let bytes=s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}