fn v1() {
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    v.push(100);
    v.push(200);
    v.push(300);

    println!("{:?}", v);
}

fn v2() {
    let v = vec![1, 2, 3];

    // 越界,程序panic

    // let third :&i32=&v[200];
    // println!("{}", third); // 3

    // 这种方式不会panic
    match v.get(200) {
        Some(third) => println!("the third element is {}", third),
        None => println!("There is no third element"),
    }
}

// 不能在同一作用域内同时拥有 可变 和 不可变借用
fn v3() {
    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0]; // 不可变借用

    v.push(6); // 对v进行了修改,可变借用

    // println!("The first element is {}", first); // 不可变借用
    println!("{:?}", v)
}

fn v4() {
    // 不可变
    let v = vec![1, 2, 3, 4, 5];

    for it in v {
        print!("{} ", it)
    }

    println!("\n");

    // 可变
    let mut v2 = vec![1, 2, 3, 4, 5];

    for it in &mut v2 {
        *it += 50;
        print!("{} ", it)
    }
    println!("\n");
}

// enum 和 Vector 结合 存放不同数据

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn v5() {
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(66.66),
        SpreadSheetCell::Text(String::from("Hello")),
    ];

    for it in row {
        println!("{:?}", it);
    }
}

pub fn init() {
    v1();
    v2();
    v3();
    v4();
    v5();
}
