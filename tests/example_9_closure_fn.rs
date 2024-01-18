#[cfg(test)]
mod example_9 {

    struct Point {
        x: f64,
        y: f64
    }

    impl Point {
        // 静态方法
        fn origin() -> Point {
            Point{x:  0.0, y: 0.0}
        }

        // 静态方法
        fn new(x: f64, y: f64) -> Point {
            Point {x: x, y: y}
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {

        // 实例方法
        // `&self` 是 `self: &Self` 的语法糖（sugar），其中 `Self` 是方法调用者的
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1} = self.p1;
            let Point { x: x2, y: y2} = self.p2;

            ((x1-x2) * (y1-y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }


    /*
    方法
    */
    #[test]
    fn test() {
        let rectangle = Rectangle {
            p1: Point::origin(), // 静态方法，使用::
            p2: Point::new(3.0, 1.0),
        };

        println!("Rectangle perimeter: {}", rectangle.perimeter());
        println!("Rectangle area: {}", rectangle.area()); // 实例方法， 使用.

        // rectangle 不是可变对象，不可调用
        // rectangle.translate(1.0, 2.0);


        let mut square = Rectangle {
            p1: Point::origin(),
            p2: Point::new(1.0, 1.0),
        };

        // 正常运行！可变对象可以调用可变方法
        square.translate(1.0, 2.0);
    }


    /*
    闭包 |val| val + x
    其他的特点包括：

    1. 声明时使用 || 替代 () 将输入参数括起来。
    2. 函数体定界符（{}）对于单个表达式是可选的，其他情况必须加上。
    3. 有能力捕获外部环境的变量。
    */
    #[test]
    fn test2() {
        let closure_annotated = |i: i32| -> i32 {i + 1};

        let i = 1;
        println!("closure_annotated: {}", closure_annotated(i));

        // 没参数的闭包
        let one = || 1;
        println!("closure returning one: {}", one());
    }

    /*
    二、捕获变量
        闭包可以捕获其环境中的变量，并在闭包的主体中使用。有三种方式可以捕获变量：
        Fn 闭包：通过引用捕获变量，不可变借用。
        FnMut 闭包：通过可变引用捕获变量，可变借用。
        FnOnce 闭包：通过值捕获变量，所有权转移。
    */

    #[test]
    fn test_closure() {

        let x = 5;
        let y = 10;

        // Fn 闭包：通过引用捕获变量
        let add = |a| a +x;


        // FnMut 闭包：通过可变引用捕获变量
        let mut multiply = |a| {
            x*y*a
        };

        // FnOnce 闭包：通过值捕获变量
        let divide = move |a| {
            a / y
        };

        let result1 = add(3);
        let result2 = multiply(2);
        let result3 = divide(10);

        println!("The results are: {}, {}, {}", result1, result2, result3);

        let result3 = divide(10);
        println!("result3 {}", result3);
    }
}