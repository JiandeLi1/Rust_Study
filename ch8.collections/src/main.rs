
use std::collections::HashMap;
fn main(){
    let v =vec![1,1,2,3,4,5,6];
    println!("Mean {}", mean(&v));
    println!("Median {}", median(&v));
    println!("Mode {}", mode(&v));
}

fn mean(v: &Vec<i32>)->i32{
    let mut ave=0;
    let len= v.len();
    for i in v{
        ave+=i;
    }
    ave/len as i32
}

fn median(v: &Vec<i32>)->f64{
    let len= v.len();
    let mut m=0.0;
    match len%2{
        0=> m=(v[len/2-1] as f64+(v[len/2]) as f64)/2.0,
        _=> m=v[len/2] as f64,
    }
    m
}

fn mode(v: &Vec<i32>)->i32{
    let mut m : HashMap<i32, i32> = HashMap::new();
    for i in v{
        let count = m.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut mode_position=0;
    let mut max = 0; 
    for (value, key) in &m{
        if value > &max {
            mode_position=v[*key as usize];
        }
    }
    
    mode_position
}