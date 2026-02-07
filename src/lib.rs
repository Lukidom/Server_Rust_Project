// Thread pool for handling multiple tasks at once
use std::{thread, sync::{Arc, Mutex, mpsc}};

// A pool of worker threads
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// A unit of work for workers to execute
type Job = Box<dyn FnOnce() + Send + 'static>;

// Messages sent to workers
enum Message {
    NewJob(Job),  // Execute this job
    Terminate,    // Shut down
}

    impl ThreadPool {   
        // Create a new thread pool with a given number of workers
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            // Create a channel for sending jobs
            let (sender, receiver) = mpsc::channel();

            // Allow workers to share the receiver
            let receiver = Arc::new(Mutex::new(receiver));

            // Create worker threads
            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            ThreadPool {
                workers, sender
            }
        }

        // Run a function on a worker thread
        pub fn execute<F>(&self, f: F) 
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(Message::NewJob(job)).unwrap();
        }
    }


// Clean up when the thread pool shuts down
impl Drop for ThreadPool {
    fn drop(&mut self) {
        print!("Sending terminate message to all workers.\n");

        // Tell all workers to stop
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // Wait for each worker to finish
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
        }
    }
}
}

// A single worker thread
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    // Start a new worker thread
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Wait for a message
            let message = receiver
            .lock()
            .unwrap()
            .recv()
            .unwrap();

            // Do what the message says
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.\n", id);
                    job();
                }
                Message::Terminate => {
                    print!("Worker {} was told to terminate.\n", id);
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}
