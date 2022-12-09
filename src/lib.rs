use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

pub struct Worker {
    id: String,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: String) -> Self {
        let thread_id = id.clone();
        // if we want that to really be production-grade
        // we should consider using std::thread::Builder instead.
        let handle = thread::spawn(move || {
            println!("Worker with id '{}' starting...", thread_id);
        });

        Self { id, thread: handle }
    }
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

        let mut workers = Vec::with_capacity(size);

        for index in 0..size {
            // create some threads and add to the pool
            let id = format!("rust-web-server-worker-{}", index);
            let w = Worker::new(id);
            workers.push(w);
        }

        ThreadPool { workers }
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
    }
}
