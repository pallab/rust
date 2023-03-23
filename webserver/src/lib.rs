use std::sync::{Arc, mpsc, Mutex};
use std::thread::JoinHandle;

type  Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {

    /// Creates a new worker with thread on standby
    ///
    /// `id` is the id of worker
    ///
    /// # Panics
    ///
    /// The `new` functions panics if there aren't enough system
    /// resources to create a new thread
    fn new(id: usize, receiver : Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            match  receiver.lock().unwrap().recv() {
               Ok(job) => {
                   println!("Worker {id} received a job");
                   job()
               }
                Err(e) => {
                    println!("Worker {id} received error. exiting");
                    break;
                }
            }


        });
        Worker { id, thread: Some(thread) }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender : Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    /// Creates a new thread pool with the given number of threads
    ///
    /// `size` is number of threads
    ///
    /// # Panics
    ///
    /// The `new` function panics if size is less than 1
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // a thread safe receiver
        // Arc will let threads share the receiver while keeping thrack of ownership
        // mutex will ensure only one thread can mutate the receiver at any one point in time
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}