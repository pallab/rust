use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
       for i in 1..6 {
           tx.send(format!("counter-{i}")).unwrap();
           println!("Sent {i}");
           thread::sleep(Duration::from_millis(2));
       }
    });

    for received in rx {
        println!("Received : {received}")
    }

    handle.join().unwrap()
}