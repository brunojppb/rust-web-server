use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a ThreadPool.
    ///
    /// Takes the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` fn will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "Can't have an empty thread pool buddy...");

        let (sender, receiver) = mpsc::channel();

        // Rust default implementation is based on
        // Multiple producers (sender), Single Consumer (receiver).
        // We can't share the receiver with multiple threads by default
        // by cloning it. We must use atomics here to proper reference-count
        // and a mutex to control access when mutating it (consuming new messages).
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for index in 0..size {
            // create some threads and add to the pool
            let id = format!("rust-web-server-worker-{}", index);
            let w = Worker::new(id, Arc::clone(&receiver));
            workers.push(w);
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        // We will eventually pass this closure
        // to thread::spawn which takes FnOnce
        // and implements 'Send' so we can transfer
        // context between threads.
        // The 'static lifetime is important because
        // we don't know how long this closure will live.
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Dropping the sender closes the channel.
        // So no more messages can be sent.
        drop(self.sender.take());
        // We can now drop the worker threads by 'join'ing the
        // running threads which will allow them to clean-up
        // properly before the main thread shuts down.
        for worker in &mut self.workers {
            println!("Shutting down worker '{}'", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: String,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: String, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread_id = id.clone();
        // if we want that to really be production-grade
        // we should consider using std::thread::Builder instead.
        let handle = thread::spawn(move || {
            println!("Worker with id '{}' starting...", thread_id);
            loop {
                // The cool thing about Rust here is that
                // any intermediate values held during the `let` expression
                // on the right side are dropped once the let statement ends.
                //
                // So the lock is released immediately released once the `recv`
                // call gets its message.
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        // When the worker is executing the `job` call, other threads
                        // can already consume more messages in the channel.
                        println!("[{}] Got a job. executing...", thread_id);
                        job();
                        println!("[{}] Job done", thread_id);
                    }
                    Err(_) => {
                        println!("[{}] Disconnected. Shutting down...", thread_id);
                        // Stop the loop to allow thread shutdown
                        break;
                    }
                }
            }
        });

        Self {
            id,
            thread: Some(handle),
        }
    }
}
