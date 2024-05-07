use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    /// The 'new' function will panic if the size is not between 1 and 10.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        assert!(size < 11);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }

        ThreadPool { workers, sender: Some(sender) }
    }
    pub fn execute<F>(&self, work: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(work);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("Unable to lock receiver")
                .recv();

            match job {
                Ok(job) => {
                    println!("Worker {id} got a job!");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            }

        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
