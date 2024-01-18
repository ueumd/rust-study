#[cfg(test)]
mod thread_study {
    use std::sync::{Arc, Condvar, mpsc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        let handle = thread::spawn(|| {
            println!("Hello from the new thread!");
        });

        handle.join().unwrap();

        println!("main thread");
    }

    #[test]
    fn test_move() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            // v 的所有权已经转移到这里
            println!("Here is a vector: {:?}", v);
        });

        // 报错！！！ 已失效，不再拥有所有限
        // println!("主線程 v: {:?}", v);

        handle.join().unwrap();
    }

    #[test]
    fn test1() {
        let v = vec![1, 2, 3];

        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            for i in 1..10 {
                println!("子线程 i：{}, v: {:?}", i, v);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        //主线程执行
        for i in 1..5 {
            println!("--------------------------主线程输出：{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    /*
    Channel
    */
    #[test]
    fn test_channel() {
        enum Fruit {
            Apple(u8),
            Orange(String),
        }

        // 创建通道，返回发送者tx 和接收者rx
        let (tx, rx) = mpsc::channel();


        // 發送
        tx.send(Fruit::Orange("sweet".to_string())).unwrap();
        tx.send(Fruit::Apple(2)).unwrap();


        for _ in 0..2 {
            // 接收
            match rx.recv().unwrap() {
                Fruit::Apple(count) => println!("received {} apples", count),
                Fruit::Orange(flavor) => println!("received {} oranges", flavor),
            };
        };
    }

    #[test]
    fn test2() {
        // 创建通道，返回发送者tx 和接收者rx
        let (tx, rx) = mpsc::channel();

        // 创建一个新线程发送数据到通道
        thread::spawn(move || {
            let message = "Hello from the sender!";
            tx.send(message).unwrap();
        });

        // 在主线程接收数据
        let received = rx.recv().unwrap();
        println!("Received: {}", received);
    }

    #[test]
    fn channel_test() {
        let (tx, rx) = mpsc::channel();

        //通过clone创建多个发送者
        let tx_clone = mpsc::Sender::clone(&tx);

        //第一个发送者 tx
        thread::spawn(move || {
            let v = vec![
                String::from("tx: hello"),
                String::from("tx: every one"),
                String::from("tx: bulabula"),
                String::from("tx: !"),
            ];

            for s in v {
                tx.send(s).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        //第一个发送者 tx_clone
        thread::spawn(move || {
            let v = vec![
                String::from("tx_clone: hello"),
                String::from("tx_clone: every one"),
                String::from("tx_clone: bulabula"),
                String::from("tx_clone: !"),
            ];

            for s in v {
                tx_clone.send(s).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for r in rx {
            println!("received: {}", r)
        }
    }

    /*
    单线程
    */
    #[test]
    fn mutex_test1() {
        // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
        let count = Mutex::new(1);

        {
            // 获取锁，然后deref为`m`的引用
            // lock返回的是Result
            let mut num = count.lock().unwrap();
            *num += 100;
            // 锁自动被drop
        }

        println!("count = {:?}", count); // count = Mutex { data: 101, poisoned: false, .. }
        println!("count = {:?}", count.lock().unwrap()); // count = 101
    }

    /*
     多线程
     Arc<Mutex<T>>
    */
    #[test]
    fn mutex_test() {
        let counter = Arc::new(Mutex::new(0));

        let mut handles = vec![];

        for _ in 1..=10 {
            // 一定要先clone，使其引用计数+1
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    /*
    并发控制
    */
    #[test]
    fn condvar_test() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = Arc::clone(&pair);

        thread::spawn(move || {
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;

            // We notify the condvar that the value has changed.
            cvar.notify_one();
        });


        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    }
}