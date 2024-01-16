#[cfg(test)]
mod example_10 {

    /*
    可以在路径中使用 super （父级）和 self（自身）关键字，从而在访问项时消除歧义，以及防止不必要的路径硬编码。
    */
    fn function() {
        println!("called `function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `cool::function()`");
        }
    }

    mod my {

        // 私有
        fn _function() {
            println!("called `my::function()`");
        }

        // 内部私有模块
        mod _cool {
            pub fn _function() {
                println!("called `my::cool::_function()`");
            }
        }

        pub mod cool2 {
            pub fn function() {
                println!("called `my::cool2::function()`");
            }
        }
    }

    #[test]
    fn test() {
        self::function(); // called `function()`

        self::cool::function(); // called `cool::function()`

        self::my::cool2::function(); // called `my::cool2::function()`

        // 错误，内部私有，不给访问
        // self::my::_cool::_function()
    }

    #[test]
    fn test2() {

    }
}