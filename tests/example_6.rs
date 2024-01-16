#[cfg(test)]
mod example_6 {

    /*
    From str 转换成 String
    */
    #[test]
    fn test() {
        let my_str = "Hello";
        let _my_string = String::from(my_str);

        /*
       也可以为我们自己的类型定义转换机制：
       */


        #[derive(Debug)] // 用于打印  {:?}
        struct Number{
            value: i32
        }

        impl From<i32> for Number {
            fn from(value: i32) -> Self {
                Number {value: value}
            }
        }

        let num = Number::from(30);
        println!("my number is {:?}, {}", num, num.value); // my number is Number { value: 30 }, 30

        // Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。
        let int = 5;
        // 试试删除类型说明
        let num2: Number = int.into();
        println!("My number is {:?}", num2); // My number is Number { value: 5 }
    }


    /*
    ToString 和 FromStr
    */
    #[test]
    fn test2() {
        // 一个实现 ToString 的例子

        struct Circle {
            radius: i32
        }

        impl ToString for  Circle {
            fn to_string(&self) -> String {
                format!("Circle of radius: {:?}", self.radius)
            }
        }

        let circle = Circle { radius: 6 };
        println!("{}", circle.to_string()); // Circle of radius: 6
    }

    #[test]
    fn test3() {
        // 解析字符串
        let parsed = "5".parse::<u8>().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();

        let sum = parsed as i32+ turbo_parsed;
        println!{"Sum: {:?}", sum}; // 15
    }
}