pub mod errors;

use std::sync::{mpsc, Arc, Mutex};
use std::{mem, thread};

use errors::{PoolCreationError, PoolExecuteError, WorkerCreationError};


#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::ZeroSize);
        }
        
        // Create a mpsc channel
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        // Create the workers
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::build(id, receiver.clone())?);
        }
        
        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }
    
    
    pub fn execute<F>(&self, f: F) -> Result<(), PoolExecuteError>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .as_ref()
            .unwrap()
            .send(Box::new(f))
            .map_err(|e| PoolExecuteError::JobCreationError(e.to_string()))
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Drop sender to stop all workers");
        drop(mem::take(&mut self.sender));
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = mem::take(&mut worker.thread) {
                thread.join().unwrap();
            }
        }
    }
}


type Job = Box<dyn FnOnce() + Send + 'static>;


#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn build(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Result<Self, WorkerCreationError> {
        let thread = thread::Builder::new().spawn(move || loop {
            let job = match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    job
                }
                Err(_) => {
                    println!("> Worker {id} disconnected; shutting down.");
                    return;
                }
            };
            
            println!("> Worker {id} got a job; executing.");
            job();
            println!("> Worker {id} finished job.");
        })?;
        
        let worker = Worker {
            id,
            thread: Some(thread),
        };
        
        Ok(worker)
    }
}
