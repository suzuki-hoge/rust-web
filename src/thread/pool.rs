use crate::LOGGER;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::logger::Logger;

pub struct Pool {
    _workers: Vec<Worker>,
    sender: Sender<Job>,
}

impl Pool {
    pub fn new(limit: u8) -> Self {
        assert!(0 < limit);

        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut _workers = Vec::with_capacity(limit as usize);
        for _ in 0..limit {
            _workers.push(Worker::new(receiver.clone()));
        }

        LOGGER.info(format!("init {} threads", limit));

        Self { _workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    _thread: JoinHandle<()>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let _thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            job.call_box();
        });

        LOGGER.info("create thread");
        Self { _thread }
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}
