use std::fs::File;

fn open_file() {
    // let f = File::open("hello.txt");
    //
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     }
    // };

    // 简写
    let f = File::open("hello.txt").expect("hello.txt 文件不存在");
}

pub fn init() {
    open_file();
}
