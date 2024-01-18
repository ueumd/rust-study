// use std::collections::HashMap;
// use std::io;
// use std::io::Write;
// use std::cmp::Ordering;

// 通配符
// use std::collections::*;

// 一行引用
// use std::{collections::HashMap, io::{self, Write}, cmp::Ordering};
mod base;

// mod struct_test;

fn fibnacci_dp2(n: u32) -> u32 {
    let mut dp = vec![];
    // dp[0] = 0;
    // dp[1] = 1;
    // dp[2] = 1;
    dp.push(0);
    dp.push(1);
    dp.push(1);


    for i in 3..=n {
        let idx1 = i as usize;
        let idx2 = (i-1) as usize;
        let idx3 = (i - 2) as usize;
        println!("{i} - {idx1} - {idx2} - {idx3}");
        dp.insert(idx1,  dp[idx2] + dp[idx3])
    }

    dp[n as usize]
}

fn main() {
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

    let s =  fibnacci_dp2(10);

    println!("fib 10: {}",s);
}
