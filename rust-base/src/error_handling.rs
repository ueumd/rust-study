#[cfg(test)]
mod error_handling {
    use std::fs::File;
    use std::io::{self, Read};

    /*
    https://cloud.tencent.com/developer/article/2338768
    Rust 错误处理详解
    */

    // 1、自定义错误类型
    #[allow(dead_code)]
    enum MyError {
        SomeError,
        AnotherError
    }

    fn do_something() -> Result<(), MyError> {
        Err(MyError::SomeError)
    }

    #[test]
    fn test() {
        match do_something() {
            Ok(_) => println!("Ok"),
            Err(error) => match error {
                MyError::SomeError => println!("SomeError"), // SomeError
                MyError::AnotherError => println!("AnotherError")
            }
        }
    }


    // 二、Result 类型
    fn divide(x: i32, y: i32) -> Result<i32, &'static str> {
        if y == 0 {
            Err("除数不能为零")
        } else {
            Ok(x / y)
        }
    }

    #[test]
    fn test2() {
        match divide(10, 2) {
            Ok(result) => println!("结果：{}", result), // 5
            Err(error) => println!("错误：{}", error),
        }
    }

    // 三、错误传播
    // 3. 使用 ? 运算符传播错误
    // 在函数内部，使用 ? 运算符将错误传播给调用方，以便在适当的位置处理错误。

    fn read_file() -> Result<(), io::Error> {
        let mut file = File::open("file.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        println!("文件内容：{}", contents);
        Ok(())
    }

    fn test_read_file() -> Result<(), io::Error> {
        read_file()?;
        Ok(())
    }

    #[test]
    fn test3() {
      let info =   test_read_file();
        println!("{:?}", info)
    }
}