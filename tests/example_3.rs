#[cfg(test)]

/**
结构体（structure，缩写成 struct）有 3 种类型，使用 struct 关键字来创建
    1. 元组结构体（tuple struct），事实上就是具名元组而已。
    2. 经典的 C 语言风格结构体（C struct）。
    3. 单元结构体（unit struct），不带字段，在泛型中很有用。
 */

mod example_3 {

    #[test]
    fn test() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        // 单元结构体
        struct Unit;

        // 元组结构体
        struct Pair(i32, f32);


        // 带有两个字段的结构体
        struct Point {
            x: f32,
            y: f32,
        }

        // 结构体可以作为另一个结构体的字段
        #[allow(dead_code)]
        struct Rectangle {
            top_left: Point,
            bottom_right: Point,
        }

        // 使用简单的写法初始化字段，并创建结构体
        let name = String::from("Peter");
        let age = 27;
        let peter = Person { name, age };
        println!("{}, {}", peter.name, peter.age);

        // 实例化一个单元结构体
        let _unit = Unit;

        //  实例化一个元组结构体
        let _pari = Pair(1, 0.1);

        // 解构元组体
        let Pair(integer, decimal) = _pari;
        println!("pair contains {} and {}", integer, decimal);

        //
        // 实例化结构体 `Point`
        let point: Point = Point{x: 10.3, y: 0.4};
        // 访问 point 的字段
        println!("point coordinates: ({}, {})", point.x, point.y);


        // 使用 `let` 绑定来解构 point
        let Point { x: left_edge, y: top_edge } = point;
        let _rectangle = Rectangle {
            top_left: Point { x: left_edge, y: top_edge },
            bottom_right: point,
        };
    }

    #[test]
    fn test2() {
        // 枚举

                                                                                                                                             // 该属性用于隐藏对未使用代码的警告。
       //  #![allow(dead_code)]

        enum WebEvent {
            PageLoad,
            PageUnLoad,

            // 一个元组结构体
            KeyPress(char),
            Paste(String),

            // 一个普通的结构体
            Click{x:i64, y: i64}
        }

        // 此函数将一个 `WebEvent` enum 作为参数，无返回值。
        fn inspect(event: WebEvent) {
            match event {
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnLoad => println!("page unloaded"),
                WebEvent::KeyPress(c) => println!("pressed '{}'. ", c),
                WebEvent::Paste(s) => println!("pasted \"{}\".", s),
                WebEvent::Click {x, y} => {
                    println!("clicked at x = {}, y = {}", x, y)
                }

            }
        }

        let pressed = WebEvent::KeyPress('x');

        // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
        let pasted  = WebEvent::Paste("my text".to_owned());
        let click   = WebEvent::Click { x: 20, y: 80 };

        let load    = WebEvent::PageLoad;
        let unload  = WebEvent::PageUnLoad;

        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);
    }


    #[test]
    fn test3() {
        // 该属性用于隐藏对未使用代码的警告。
        #![allow(dead_code)]

        enum Status {
            Rich,
            Poor
        }

        enum Work {
            Civilian,
            Soldier,
        }

        // 使用 use 声明的话，就可以不写出名称的完整路径了
        // WebEvent::PageLoad

        use Status::*;
        use Work::*;

        let status = Poor;
        let work = Civilian;

        match status {
            Rich => println!("the rich have lots of money!"),
            Poor => println!("The poor have no money..."),
        }

        match work {
            // 再次注意到没有用完整路径。
            Civilian => println!("Civilians work!"),
            Soldier  => println!("Soldiers fight!"),
        }
    }


    /**
    常量
    const：不可改变的值（通常使用这种）。
    static：具有 'static 生命周期的，可以是可变的变量（译注：须使用 static mut 关键字）。
    */
    #[test]
    fn test4() {
        static LANGUAGE: &'static str = "Rust";
        const THRESHOLD:i32 = 10;

        fn is_big(num :i32) -> bool {
            num > THRESHOLD
        }

        let num = 16;

        println!("This is {}", LANGUAGE);
        println!("The threshold is {}", THRESHOLD);
        println!("{} is {}", num , if is_big(num) {"big"} else {"small"});

    }
}