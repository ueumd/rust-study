// use std::collections::HashMap;
// use std::io;
// use std::io::Write;
// use std::cmp::Ordering;

// 通配符
// use std::collections::*;

// 一行引用
// use std::{collections::HashMap, io::{self, Write}, cmp::Ordering};


// mod struct_test;


fn _main1() {
    // study::study::hello();
    // println!("add_tow: {}", study::study::add(100));

    // func::func::init();
    // study::vector::init();
    // study::array::init();
    // study::hashmap::init();
    // study::error::init();
    // study::option_result::init();
    // study::lifetimes::init();
    // study::pointer::init();
    // study::trait_study::init();
    // study::oop::init();
    // study::async_await::init();
    // study::thread_test::init();
    // study::channel::init();

    // base::base::init()
    // base::trait_study::init()
}
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from the new thread!");
    });

    //主线程执行
    for i in 1..5 {
        println!("主线程输出：{}", i);
        thread::sleep(Duration::from_millis(1));
    }


    handle.join().unwrap();
}