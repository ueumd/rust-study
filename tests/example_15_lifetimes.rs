#[cfg(test)]
mod example {

    // 没有生命周期
    #[test]
    fn test() {
        fn print_refs(x: &i32, y: &i32) {
            println!("x is {} and y is {}", x, y);
        }

        let (four, nine) = (4, 9);

        // 两个变量的借用（`&`）都传进函数。
        print_refs(&four, &nine); // x is 4 and y is 9
    }

    #[test]
    fn test2() {

        // https://blog.csdn.net/quicmous/article/details/113946091

        // 函数 longer 的返回结果的生命周期，可能与 s1 相同，也可能与 s2 相同。因此，longer 结果的生命周期在编译阶段是未知的。
        // fn _longer(s1: &str, s2: &str) -> &str {
        //     if s2.len() > s1.len() {
        //         s2
        //     } else {
        //         s1
        //     }
        // }

        // 生命周期
        // 使用生命周期需要泛型
        fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
            if s2.len() > s1.len() {
                s2
            } else {
                s1
            }
        }

        let r;

        let s1 = String::from("rust");
        let s2 = String::from("ecmascript");

        //编译器在分析到 r = longer(&s1, &s2); 这一行时，无法确定 r 得到的数据，其生命周期是否足够长
        // r = _longer(&s1, &s2);

        r = longer(&s1, &s2);
        println!("{} is longer", r); // ecmascript is longer
    }
}