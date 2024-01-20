#[cfg(test)]
mod thread_barrier {
    use std::sync::{Arc, Barrier, Mutex};

    #[test]
    fn test() {
        let num_threads = 10;
        let my_mutex = Arc::new(Mutex::new(0));

        let mut handlers = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            let my_lock = my_mutex.clone();
            handlers.push(std::thread::spawn(move || {
                let mut guard = my_lock.lock().unwrap();
                *guard += 1;
            }))
        }


        for handler in handlers {
            handler.join().unwrap();
        }

        let answer = {
            *my_mutex.lock().unwrap()
        };

        println!("answer: {}", answer);
        println!("num_threads: {}", num_threads);
    }

    #[test]
    fn test2() {
        let num_threads = 10;
        let my_mutex = Arc::new(Mutex::new(0));

        // let mut handlers = Vec::with_capacity(num_threads);
        let barrier = Arc::new(Barrier::new(num_threads + 1));

        for i in 0..num_threads {
            let my_barrier = barrier.clone();
            let my_lock = my_mutex.clone();

            std::thread::spawn(move || {
                let mut guard = my_lock.lock().unwrap();
                *guard += 1;

                // Release the lock to prevent a deadlock
                drop(guard);
                println!("thread {} is ready", i);

                // Blocks the current thread until all threads have rendezvoused here.
                my_barrier.wait();
                println!("thread {} is done", i)
            });
        }


        barrier.wait();

        let answer = {
            *my_mutex.lock().unwrap()
        };

        println!("answer: {}", answer);
        println!("num_threads: {}", num_threads);
    }
}