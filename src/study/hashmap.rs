use std::collections::HashMap;

fn hm() {
    let mut scores = HashMap::new();

    // 所有的key同一种类型，所有的v同一种类型
    scores.insert(String::from("red"), 10);
    scores.insert(String::from("yellow"), 20);
    scores.insert(String::from("blue"), 30);
    println!("{:?}", scores);

    let i = scores.get("blue");

    println!("{:?}", i); // Some(30)
    println!("{}", i.unwrap()); // 30

    let key = String::from("yellow2");
    let i2 = scores.get(&key);

    match i2 {
        Some(s) => println!("{}", s),
        None => println!("no value"),
    }
}

// 更新数据
fn hm2() {
    let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 0);

    // 使用 entry 方法只在键没有对应一个值时插入
    // let sc =     scores.entry(String::from("yellow"));
    // println!("{:?}", sc);
    // sc.or_insert(50);

    let str = String::from("blue");

    let mut count = scores.entry(String::from("blue")).or_insert(0);
    *count += 1;

    count = scores.entry(String::from("blue")).or_insert(0);
    println!("count ------- {}", count);
    println!("scores ------- {:?}", scores); // scores ------- {"blue": 1}
}

// 根据旧值更新一个值

fn hm3() {
    let text = "hello";
    let mut map = HashMap::new();

    let list = text.split_whitespace();

    println!("---------list {:?}", list);

    // 将字符串切害成数组
    for word in list {
        println!("cur word: {}", word);
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    // let count = map.entry(word).or_insert(0);
    // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）
    // 其中count是map中对应的元素的引用， 所以*count 解开之后即可变变量， 当执行+=1的时候，对应map中值执行了+1操作。
    println!("{:?}", map); // {"hello": 1, "world": 2, "wonderful": 1}
}

fn hm4() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

pub fn init() {
    hm();
    hm2();
    hm3();
    hm4();
}
