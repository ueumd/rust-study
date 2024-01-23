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

use feature_demo::bmp::process_bmp;
use feature_demo::ico::process_ico;
use feature_demo::png::process_png;
use feature_demo::webp::process_webp;


fn main() {
    process_bmp();
    process_png();
    process_ico();
    process_webp();
}