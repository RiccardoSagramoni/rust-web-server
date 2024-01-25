use std::io;

use thiserror::Error;


#[derive(Error, Debug)]
pub enum PoolCreationError {
    #[error("can't create a ThreaPool with size = 0")]
    ZeroSize,
    #[error("failed to create a new worker")]
    WorkerCreationError(#[from] WorkerCreationError),
}

#[derive(Error, Debug)]
pub enum WorkerCreationError {
    #[error("failed to spawn a new thread")]
    ThreadSpawnError(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum PoolExecuteError {
    #[error("failed to allocate a new job: {0}")]
    JobCreationError(String)
}
