use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(mut size: usize) -> ThreadPool {
        if size < 1 { size = 1; }

        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

//    pub fn spawn<F, T>(_f: F) -> thread::JoinHandle<T>
//        where
//            F: FnOnce() -> T + Send + 'static,
//            T: Send + 'static
//    {
//
//    }

    pub fn execute<F>(&self, _f: F)
        where
            F: FnOnce() + Send + 'static
    {   }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker{
            id,
            thread,
        }
    }
}
