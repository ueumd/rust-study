use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn init1() {
    let v = vec![1, 2, 3];
    let new_thread = thread::spawn(move || {
        // 在子线程的闭包中捕获了环境中的 v 变量
        // 使用 move 关键字拿走 v 的所有权即可
        println!("I am a new thread. {:?}", v);
    });

    // join()方法会阻止当前线程（主线程）的运行，直到handle对应的线程结束运行
    new_thread.join().unwrap();

    let new_thread2 = thread::Builder::new()
        .name("thread1".to_string())
        .stack_size(4 * 1024 * 1024)
        .spawn(move || {
            println!("I am a new thread2.");
        });

    new_thread2.unwrap().join().unwrap();
}

pub fn init2() {
    let new_thread = thread::spawn(move || {
        thread::spawn(move || loop {
            println!("I am a new thread.");
        })
    });

    new_thread.join().unwrap();

    println!("Child thread is finish!");

    // 睡眠一段时间，看子线程创建的子线程是否还在运行
    thread::sleep(Duration::from_millis(100));
}
pub fn init() {
    let v = vec![1, 2, 3];

    //子线程执行
    //使用move关键字在线程间移交数据所有权
    let handle: thread::JoinHandle<()> = thread::spawn(move || {
        for i in 1..10 {
            println!("子线程输出：{}", i);
            println!("子线程输出：{:?}", v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //主线程执行
    for i in 1..5 {
        println!("主线程输出：{}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //join()方法会阻止当前线程（主线程）的运行，直到handle对应的线程结束运行
    handle.join().unwrap();
}
