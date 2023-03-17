use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    let counter = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            (*num).push(i);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("counter {:?}", &counter.lock().unwrap());
}