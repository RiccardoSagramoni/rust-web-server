pub mod errors;

use std::sync::{mpsc, Arc, Mutex};
use std::thread::{Builder, JoinHandle};

use errors::{PoolCreationError, PoolExecuteError, WorkerCreationError};


#[derive(Debug)]
pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
        
        Ok(ThreadPool { _workers: workers, sender })
    }
    
    
    pub fn execute<F>(&self, f: F) -> Result<(), PoolExecuteError>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .send(Box::new(f))
            .map_err(|e| PoolExecuteError::JobCreationError(e.to_string()))
    }
}


type Job = Box<dyn FnOnce() + Send + 'static>;


#[derive(Debug)]
struct Worker {
    _id: usize,
    _thread: JoinHandle<()>,
}

impl Worker {
    fn build(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Result<Self, WorkerCreationError> {
        let worker = Worker {
            _id: id,
            _thread: Builder::new().spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing.");
                job();
                println!("Worker {id} finished job.");
            })?,
        };
        
        Ok(worker)
    }
}
