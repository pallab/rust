use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    let handle1 = thread::spawn(move || {
       for i in 1..6 {
           tx.send(format!("thread-1-{i}")).unwrap();
           thread::sleep(Duration::from_millis(2));
       }
    });
    let handle2 = thread::spawn(move || {
        for i in 1..6 {
            tx2.send(format!("thread-2-{i}")).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    for received in rx {
        println!("Received : {received}")
    }

}