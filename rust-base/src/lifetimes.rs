#[cfg(test)]
mod lifetimes {
    /**

    &i32        // 引用
    &'a i32     // 带有显式生命周期的引用
    &'a mut i32 // 带有显式生命周期的可变引用
     */
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }


    fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }


    #[test]
    fn test() {
        let str1 = String::from("abcd");
        let str2 = "xyz";

        let result = longest(str1.as_str(), str2);
        println!("The longest string is {}", result);
    }

    #[test]
    fn test2() {
        let result;
        {

            // 编译不了 str1 `str1` does not live long enough

            // let str1 = String::from("hello");
            // let str2: &str = "world!";
            // result = longest2(str1.as_str(), str2);


            let str1 ="hello";
            let str2 = "world!";
            result = longest2(str1, str2);
        }

        println!("The longest string is: {}", result);
    }
}