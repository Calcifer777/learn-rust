use std::{thread, time::Duration, sync::{Mutex, Arc, mpsc::{self, Sender}}};
use rand::prelude::*;


pub fn thread_example() {
    let t = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    t.join().unwrap();
}

pub fn thread_ownership(v: Vec<i32>) {
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

pub fn mutex_example() {
    let m = Arc::new(Mutex::new(5));
    let mut t_handles = Vec::new();
    for i in 1..10 {
        let counter = Arc::clone(&m);
        let t_handle = thread::spawn(move || {
            println!("Thread {}: spawned", i);
            let t = random::<u64>() % 10000;
            let mut num = counter.lock().unwrap();
            println!("Thread {} got the lock", i);
            *num = *num + i;
            println!("Thread {}: sleeping for {} ms", i, t);
            thread::sleep(Duration::from_millis(t));
        });
        t_handles.push(t_handle);
    }
    for t_handle in t_handles {
        t_handle.join().unwrap();
    }
}


pub fn channel_example() {
    let (tx, rx) = mpsc::channel::<i32>();
    let mut txs: Vec<&Sender<i32>> = Vec::new();
    for prime in vec![3, 5, 7, 11] {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for i in 1..3 {
                let v = prime * i;
                println!("Sending {}", v);
                tx_clone.send(v).unwrap();
            } 
            println!("Sending done for channel {}!", prime);
        });
    }

    let rx_handle = thread::spawn(move || {
        for i in rx.iter() {
            println!("Received {}", i);
        }
        println!("Reading done!");
    });
    rx_handle.join().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_thread_example() {
        thread_example();
    }

    #[test]
    fn check_thread_ownership() {
        thread_ownership(vec![1]);
    }
}
