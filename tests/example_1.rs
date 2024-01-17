#[cfg(test)]
mod example_1 {
    #[test]
    fn test1() {
        // 可以使用位置参数
        println!("{0}, this is {1}, this is {0}", "Alice", "Bob");

        // 使用命名参数
        println!("{subject}, {verb}, {object}",
            subject ="the quick brown fox",
            verb = "jumps over",
            object = "the lay dog",
        );

        // 你可以在数字左边补 0。下面语句输出 "000001"
        println!("{number:0width$}", number = 1, width= 6); // 000001
    }

    #[test]
    fn test2() {

        // `Structure` 是一个包含单个 `i32` 的结构体。
        #[derive(Debug)]
        struct Structure(i32);

        #[derive(Debug)]
        struct Deep(Structure);

        println!("Now {:?} will print!", Structure(3));
        // Now Structure(3) will print!



        println!("Now {:?} will print!", Deep(Structure(3)));
        // Now Deep(Structure(3)) will print!


        // 使用 `derive` 的一个问题是不能控制输出的形式。
        // 假如我只想展示一个 `3` 怎么办？
    }

    #[test]
    fn test3() {

        #[derive(Debug)]
       struct Person<'a> {
            name: &'a str,
            age: u8
        }

        let name = "Peter";
        let age = 27;

        let peter = Person{name, age};

        // {:#?} 美化打印 (打印出结构)
        println!("{:#?}", peter);
        println!("{}, {}",peter.name, peter.age)

       /* Person {
            name: "Peter",
            age: 27,
        }*/
    }
}
