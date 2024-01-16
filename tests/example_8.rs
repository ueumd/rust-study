#[cfg(test)]
mod example_8 {

    /**
    if/else
     */
    #[test]
    fn test() {
        let n = 5;
        let big_n = if n < 10 && n >-10 {
            10*n
        } else {
            n /2
        };

        println!("{} -> {}", n, big_n); // 5 -> 50

        let mut count = 0u32;

        // loop 无限循环
        loop {
            count += 1;
            if count == 3 {
                println!("three");
                // 跳过这次迭代的剩下内容
                continue;
            }
            println!("{}", count);

            if count == 5 {
                println!("OK, that's enough");

                // 退出循环
                break;
            }
        }

    }


    /*
    for
    */
    #[test]
    fn test2() {
        let names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"), // There is a rustacean among us!
                _ => println!("Hello {}", name),
            }
        }
    }

    /*
    match 匹配
    Rust 通过 match 关键字来提供模式匹配，和 C 语言的 switch 用法类似。第一个匹配分支会被比对，并且所有可能的值都必须被覆盖。
    */
    #[test]
    fn test3() {
        let number = 13;
        println!("Tell me about {}", number);

        // switch 用法类似
        match number {
            1 => println!("One!"),

            // 匹配多个值
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            13..=19 => println!("A teen"),

            // 处理其他情况
            _ => println!("Ain't special"),
        }

        let boolean = true;
        // match 也是一个表达式
        let binary = match boolean {
            false => 0,
            _ => 1
        };


        println!("{} -> {}", boolean, binary); // true -> 1
    }

    #[test]
    fn test4() {
        let triple = (0, -2, 3);
        match triple {
            (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z), // First is `0`, `y` is -2, and `z` is 3
            (1, ..)  => println!("First is `1` and the rest doesn't matter"),
            _      => println!("It doesn't matter what they are"),
        }


        let pair = (2, -2);

        println!("Tell me about {:?}", pair);

        // 上 match 卫语句（guard） 来过滤分支。
        match pair {
            (x, y) if x == y => println!("These are twins"),
            // ^ `if` 条件部分是一个卫语句
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            _ => println!("No correlation..."),
        }


        // 绑定 match 提供了 @ 符号来绑定变量到名称：

        fn get_age() -> u8 {
            15
        }

        match get_age() {
            0             => println!("I haven't celebrated my first birthday yet"),
            n @ 1 ..=12 => println!("I'm a child of age {:?}", n),
            n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
            // 不符合上面的范围。返回结果。
            n             => println!("I'm an old person of age {:?}", n),
        }

        // “解构” enum 变体，例如 Option:
        fn some_number() -> Option<u32> {
            Some(42)
        }

        match some_number() {
            Some(n @ 42) => println!("The Answer: {}", n),
            Some(n) => println!("Not interesting... {}", n),
            None => ()
        }
    }

    /*
    if let Some(i) = xx {}
    */

    #[test]
    fn test5() {
        // 将 `optional` 定为 `Option<i32>` 类型
        let optional = Some(7);
        match optional {
            Some(i) => {
                println!("This is a really long string and `{:?}`", i); // This is a really long string and `7`
            },
            _ => {}
        }

        // 注意这里中有一个 =
        if let Some(i) = optional {
            println!("This is a really long string and `{:?}`", i); // This is a really long string and `7`
        } else {
            println!("Didn't match a number. Let's go with a letter!");
        }

        // enum Foo {Bar}
        //
        // let a = Foo::Bar;
        // if let Foo::Bar = a {
        //     println!("a is foobar");
        // }
    }

    #[test]
    fn test6() {
        let mut optional = Some(0);
        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 2);
            }
        }
    }
}