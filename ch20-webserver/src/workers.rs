use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender
        .as_ref()
        .unwrap()
        .send(job)
        .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.worker_id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    worker_id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(worker_id: usize, receiver: JobReceiver) -> Worker {
        println!("Spawning worker {}", worker_id);
        let thread = thread::spawn(move || {
            loop {
                let job = receiver
                .lock()
                .unwrap()
                .recv();
                match job {
                    Ok(job) => {
                        println!("Worker {} got a job; executing.", worker_id);
                        job()
                    },
                    Err(_) => {
                        println!("Worker {} disconnected; shutting down.", worker_id);
                        break
                    }
                }
            }
        });
        Worker { worker_id: worker_id, thread: Some(thread) }
    }
}

type JobReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;
