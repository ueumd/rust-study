#[cfg(test)]
mod str_study {

    /*
    疯狂字符串
    https://course.rs/difficulties/string.html
    https://rust-book.junmajinlong.com/ch3/04_str_string.html
    https://juejin.cn/post/7140287262553538591
    */
    #[test]
    fn test() {

        /*
        str：这是一个不可变的固定长度的字符串类型，通常以切片 &str 的形式出现。常用于固定的、不需要修改的字符串。
        */

        //  let str1 = "banana";
        //  字符串字面量 自动推导如下
        let str1: &str = "banana";
        /*
        错误用法
        let mut str1 = "sss"
        str1.push("111)
        */
        println!("str1: {}", str1); // str: banana


        /*
        String：这是一个动态的，可增长的字符串类型。它被分配在堆上并且可以修改。常用于需要创建或修改字符串的场合。
        */
        let mut s = String::from("hello");
        s.push('.');        // push()可追加单个char字符类型
        s.push_str(", world!");
        println!("{}", s); // hello, world!


        /*
        &String：这是一个指向 String 的引用，常用于函数参数，以便可以接受 String 类型也可以接受 &str 类型。
        */
        let s2 = String::from("Hello, world!");
        let t: &String = &s2;
        println!("{}", t); //  Hello, world!


        /*
        Box：这是一个堆分配的固定长度字符串。它的使用比较罕见，只有在一些特殊的场合才会用到。
        */

        let s3: Box<str> = Box::from("Hello, world!");
        println!("{}", s3); // This will print: Hello, world!

        /*
        Box<&str>：这是一个罕见的类型，通常不会在 Rust 代码中出现。它是一个指向 str 的堆分配引用。因此，以下例子是比较不常见的。
        */
        let s4: &str = "Hello, world!";
        let t: Box<&str> = Box::new(s4);
        println!("{}", t); //  Hello, world!
    }

    #[test]
    fn test2() {
        let s = String::from("Hello, Rust!");

        let sliced = &s[7..12];

        println!("{}", sliced); // Rust!

    }
}