use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


impl ThreadPool {
    pub fn new(num_workers: usize) -> ThreadPool {
        assert!(num_workers > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(num_workers);
        for worker_id in 0..num_workers {
            workers.push(Worker::new(worker_id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    worker_id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(worker_id: usize, receiver: JobReceiver) -> Worker {
        println!("Spawning worker {}", worker_id);
        let thread = thread::spawn(move || {
            loop {
                let job = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();
                println!("Worker {} got a job; executing.", worker_id);
                job();
            }
        });
        Worker { worker_id, thread }
    }
}

type JobReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;
