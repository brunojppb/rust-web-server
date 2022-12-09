pub struct ThreadPool;

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
        Self
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
