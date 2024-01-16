use std::fs::File;
use std::io::ErrorKind;

fn option1() {
    // let val:Option<u32> = Some(100);
    let val: Option<u32> = None;

    match val {
        Some(num) => println!("val is: {}", num), // val is: 100
        None => println!("vla is none"),
    }

    if let Some(num) = val {
        println!("val is: {}", num)
    } else {
        println!("vla is none")
    }
}

//  java 代码一样，如何做呢
/**
User user = model.getUser();

if (user == null) {
    throw new Exception("用户不存在");
}

System.out.println("你好：" + user.getName());
*/

fn option2() {
    let val: Option<u32> = None;
    // 如果值是 None 则直接 panic!，程序就挂了。
    let num: u32 = val.unwrap();

    println!("num is: {}", num)
}

fn division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn display_division(dividend: i32, divisor: i32) {
    match division(dividend, divisor) {
        None => println!("Oops, divisor == 0"),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        }
    }
}

fn option3() {
    display_division(4, 2); // 4 / 2 = 2
    display_division(1, 0); // Oops, divisor == 0
}

fn option4() {
    // let x = Some(3);  下面会出错
    // let y = Some(4);

    let x = Some(3).unwrap();
    let y = Some(4).unwrap();

    // 可见unwrape() 就是把 Some 里的值取出来。
    println!("{:?}", x + y); //7

    let y: Option<i32> = None;
    // println!("y is: {:?}", x+y.unwrap()); 程序会挂

    println!("y is: {:?}", x + y.unwrap_or(4)); //7
}

fn result1() {
    // let sr: Result<u32, &str> = Ok(123); //or Err("You are wrong");
    let sr: Result<u32, &str> = Err("You are wrong");

    match sr {
        Ok(v) => println!("sr value: {}", v),
        Err(e) => println!("sr is error {}", e),
    }
}

fn first(arr: &[i32]) -> Option<&i32> {
    let v = arr.get(0)?;
    Some(v)
}
// ?号的作用
fn test() {
    let arr = [1, 3, 3, 4];
    let val = first(&arr);
    match val {
        Some(num) => println!("val is: {}", num), // val is: 1
        None => println!("vla is none"),
    }
}

fn result_test() {
    // let f = File::open("hello.txt").unwrap();
    // let f2  = File::open("hello.txt").expect("Faild to open file");
    let f3 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

pub fn init() {
    // option1();
    // option2();
    //
    // result1();
    // test();

    option3();

    option4();

    result_test();
}
