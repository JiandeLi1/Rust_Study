use std::thread;
use std::time::Duration;
fn main() {
    let handle=thread::spawn(||{
        for i in 1..10{
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{//When main thread is end, all is end, but we can use handle.join
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
