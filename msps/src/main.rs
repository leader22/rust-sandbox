use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
    });

    let tx2 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = String::from("bye");
        tx2.send(val).unwrap();
    });

    // blocking
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // non-blocking
    let received2 = rx.try_recv().unwrap();
    println!("Got: {}", received2);

    // non-blocking but panic w/ Empty
    let received3 = rx.try_recv().unwrap();
    println!("Got: {}", received3);
}
