use std::thread;
use std::time::Duration;

pub fn run() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread 1 : {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread 2 : {i}");
            thread::sleep(Duration::from_millis(9));
        }
    });

    println!("Hello, world!");

    for i in 1..5 {
        println!("Main Thread : {i}");
        thread::sleep(Duration::from_millis(8));
    }

    handle.join();
    handle2.join();
}