use std::thread;
use std::time::Duration;
fn main() {
    let handle=thread::spawn(||{
        for i in 1..10{
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();//handle will go first
    for i in 1..5{//When main thread is end, all is end, but we can use handle.join
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //handle.join().unwrap(); //If we put join here, main thread and hanle will cross compilation



    let v=vec![1,2,3];
    let h = thread::spawn(move||{
        println!("Here's a vector: {:?}", v);
    });
    //drop(v); //main thread will go first, if drop here, h doesn't get h ownership!
    h.join().unwrap();
}
