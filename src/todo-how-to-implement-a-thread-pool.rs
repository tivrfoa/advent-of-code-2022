use std::thread::JoinHandle;
use std::thread;

struct ThreadPool {
    threads: Vec<thread::JoinHandle<(usize, Vec<(usize, usize)>)>>,
    running_threads: usize,
}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        let num_threads = thread::available_parallelism().unwrap().get();
        let mut threads = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            // create some threads and store them in the vector
        }

        ThreadPool {
            threads,
            running_threads: 0,
        }
    }

    pub fn execute<F, T>(&mut self, f: F) -> JoinHandle<T>
        where
            F: FnOnce() -> T,
            F: Send + 'static,
            T: Send + 'static,
    {
        self.running_threads += 1;
        thread::spawn(|| f())
    }
}


fn main() {
    let thread_join_handle: JoinHandle<i32> = thread::spawn(move || {
        3
    });
    // some work here
    let res = thread_join_handle.join();
    
    println!("{:?}", res);
    
    
    println!("{:?}", ThreadPool::new().execute(|| 5));
    
    let mut pool = ThreadPool::new();
    println!("{:?}", pool.execute(|| 6).join());
    println!("{:?}", pool.running_threads);
    println!("{:?}", pool.execute(|| 9).join());
    println!("{:?}", pool.running_threads);

}