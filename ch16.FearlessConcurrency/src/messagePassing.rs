use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //println!("val is {}", val); //not work because val's ownership was sent to rx
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);


    let (tx1, rx1) = mpsc::channel();
    let tx2=tx1.clone();//Can be more transmitter, but only one reciver
    thread::spawn(move || {
        let vals= vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals= vec![
            String::from("more"),
            String::from("information"),
            String::from("for"),
            String::from("you")
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for r in rx1 {
        println!("Got: {}", r);
    }
}