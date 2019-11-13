use std::thread;
use std::time::Duration;

const DELAY_MS: u64 = 10;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("move is needed to use {:?}", v);

        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(DELAY_MS));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(DELAY_MS));
    }
}
