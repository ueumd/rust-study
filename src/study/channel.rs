use futures::join;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Receiver, Sender};

// pub fn init() {
//     // 创建一个消息通道, 返回一个元组：(发送者，接收者)
//     let (tx, rx) = mpsc::channel();
//
//     // 创建线程，并发送消息
//     thread::spawn(move || {
//         // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
//         tx.send(100).unwrap();
//
//         // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
//         // tx.send(Some(1)).unwrap()
//     });
//
//     // 在主线程中接收子线程发送的消息并输出
//     println!("receive {}", rx.recv().unwrap());
//
// }

// 使用多发送者
// https://zhuanlan.zhihu.com/p/668519578
pub fn init1() {
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let (tx, mut rx) = mpsc::channel::<u32>();

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx1.send(50).unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    thread::spawn(move || {
        tx2.send(100).unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    for received in rx {
        if let i = received {
            println!("Got = {}", received);
            db[4] = received;
            println!("{:?}", db);
        }
    }
}

// 使用 for 进行循环接收
pub fn init2() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}


// 传输具有所有权的数据
pub fn init3() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("我，飞走咯!");
        // String底层的字符串是存储在堆上，并没有实现Copy特征，当它被发送后，会将所有权从发送端的s转移给接收端的received
        // 之后s将无法被使用， 所以这里进行一次clone
        tx.send(s.clone()).unwrap();
        println!("s is {}", s);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 异步通道 同步同道
pub fn init4() {

    // 异步通道
    // let (tx, rx) = mpsc::channel();

    // 同步通道
    let (tx, rx) = mpsc::sync_channel(0);
    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

enum Fruit {
    Apple(u8),
    Orange(String)
}

// 传输多种类型的数据
pub fn init() {
    let (tx, rx):(Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();
    
    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) =>println!("received {} apples", count),
            Fruit::Orange(flavor) =>  println!("received {} oranges", flavor),
        };
    };
}