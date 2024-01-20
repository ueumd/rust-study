#[cfg(test)]
mod thread_atomic {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;

    // 初始化原子bool类型
    static FLAG:AtomicBool = AtomicBool::new(false);

    #[test]
    fn test() {

        let handle_a = thread::spawn(||{
            // 原子操作修改
            FLAG.store(true, Ordering::Relaxed);
        });

        let handle_b = thread::spawn(|| {
            // 读取原子操作
            if FLAG.load(Ordering::Relaxed) {
                println!("Relaxed: Flag is set {:?}", FLAG);
            }
        });

        handle_a.join().unwrap();
        handle_b.join().unwrap();
    }
}