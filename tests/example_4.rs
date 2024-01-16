#[cfg(test)]
mod example_3 {

    /*
    可变变量
    */
    #[test]
    fn test() {
        // 不可变
        let _immutable_binding = 1;

        // 可变
        let mut mutable_binding = 1;

        println!("Before mutation: {}", mutable_binding);

        mutable_binding += 1;
        println!("After mutation: {}", mutable_binding);

        // 错误代码！
        //  _immutable_binding += 1;
    }

    /*
    作用域和遮蔽
    */
    #[test]
    fn test2() {
        // 此绑定生存于 main 函数中
        let long_lived_binding = 1;

        {
            // 此绑定只存在于本代码块
            let short_lived_binding = 2;

            println!("inner short: {}", short_lived_binding); // 2

            // 此绑定*遮蔽*了外面的绑定
            let long_lived_binding = 5_f32;
            println!("inner long: {}", long_lived_binding); // 5
        }


        // 报错！`short_lived_binding` 在此作用域上不存在
        // println!("outer short: {}", short_lived_binding);

        println!("outer long: {}", long_lived_binding); // 1

        // 此绑定同样*遮蔽*了前面的绑定
        let long_lived_binding = 'a';
        println!("outer long: {}", long_lived_binding); // a
    }

    /*
    变量先声明
    */
    #[test]
    fn test3() {

        // 声明变量!!!
        let a_binding ;

        // 块作用域
        {
            let x = 2;

            // 初始化一个绑定
            a_binding = x * x;
            println!("a binding: {}", a_binding); // 4
            println!("x binding: {}", x); // 2
        }

        println!("a binding: {}", a_binding); // 4
        let another_binding;

        // 报错！！！ 必须要初始化后方可使用
        // println!("another binding: {}", another_binding);

        another_binding = 10;
        println!("another binding: {}", another_binding); // 10
    }

    /*
    冻结
    */
    #[test]
    fn test4() {
        let mut _mutable_integer = 7i32;

        {
            // 被不可变的 `_mutable_integer` 遮蔽
            let _mutable_integer = _mutable_integer;

            // 错误  `_mutable_integer` 在本作用域被冻结 此 _mutable_integer 是内部的 _mutable_integer
            // _mutable_integer = 50;
            println!("_mutable_integer binding: {}", _mutable_integer); // 7
        }


        // 外部作用域
        _mutable_integer = 10;

        println!("mutable_integer binding: {}", _mutable_integer); // 10
    }
}