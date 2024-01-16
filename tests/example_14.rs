#[cfg(test)]
mod example {
    use std::fmt::Debug;

    // 泛型
    // impl 实现
    #[test]
    fn test() {
        struct Val {
            val: f64
        }
        impl Val {
            fn value(&self) -> &f64 {&self.val}
        }

        struct GenVal<T> {
            gen_val: T
        }

        // GenVal 的 `impl`，指定 `T` 是泛型类型
        impl <T> GenVal<T> {
            fn value(&self) -> &T {&self.gen_val}
        }
        let x = Val { val: 3.0 };
        let y = GenVal { gen_val: 3i32 };
        println!("{}, {}", x.value(), y.value());

    }


    // 泛型约束
    #[test]
    fn test2() {
        #[derive(Debug)]
        struct Rectangle { width: f64, height: f64 }

        #[allow(dead_code)]
        struct Triangle  { width: f64, height: f64 }

        trait HasArea {
            fn area(&self) -> f64;
        }

        // Rectangle 必须实现 HasArea的方法
        impl HasArea for Rectangle {
            fn area(&self) -> f64 {
                self.width * self.height
            }
        }

        //T 必须实现 HasArea， 任意符合该约束的泛型的实例
        fn area<T: HasArea>(t: &T) -> f64 {t.area()}

        let rectangle = Rectangle { width: 3.0, height: 4.0 };

        let _triangle = Triangle  { width: 3.0, height: 4.0 };

        println!("The Rectangle area: {}", area(&rectangle));

        // 报错 Triangle 并未实现 HasArea
        // println!("The Rectangle area: {}", area(&_triangle));
    }

    // 泛型约束2  理解 #[derive(Debug)]
    #[test]
    fn test3() {
        /*
        当使用 #[derive()] 属性时，Rust 编译器会自动生成实现特定 traits 所需的代码
        从而简化了手动编写这些代码的过程。让我们更详细地了解一下如何使用和定制 #[derive()]。
        */
        #[derive(Debug)]
        struct Rectangle { _width: f64, _height: f64 }

        // 泛型 `T` 必须实现 `Debug` 。只要满足这点，无论什么类型
        // 都可以让下面函数正常工。
        fn print_debug<T: std::fmt::Debug>(t: &T) {
            println!("{:?}", t);
        }


        let rectangle = Rectangle { _width: 3.0, _height: 4.0 };
        print_debug(&rectangle); // Rectangle { _width: 3.0, _height: 4.0 }
    }

    // 多重约束（multiple bounds）可以用 + 连接。和平常一样，类型之间使用 , 隔开。
    #[test]
    fn test4() {
        use std::fmt::{Debug, Display};

        fn compare_prints<T: Debug + Display>(t: &T) {
            println!("Debug: `{:?}`", t);
            println!("Display: `{}`", t);
        }

        fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
            println!("t: `{:?}`", t);
            println!("u: `{:?}`", u);
        }

        let string = "words";
        let array = [1, 2, 3];
        let vec = vec![1, 2, 3];

        compare_prints(&string);
        /*
        Debug: `"words"`
        Display: `words`
        */

        // compare_prints(&array);

        compare_types(&array, &vec);
    }


    // where 分句
    // 约束也可以使用 where 分句来表达
    #[test]
    fn test5() {

        // impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
        // impl <A, D> MyTrait<A, D> for YourType where  A: TraitB + TraitC, D: TraitE + TraitF {}
        trait PrintInOption {
            fn print_in_option(self);
        }

        // 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
        // 或者改用另一种间接的方法。
        impl <T> PrintInOption for T where Option<T>: Debug {
            fn print_in_option(self) {
                println!("{:?}", Some(self));
            }
        }

        let vec = vec![1, 2, 3];

        vec.print_in_option();
    }

    #[test]
    fn test6() {
        struct Container(i32, i32);

        trait Contains<A, B> {
            // 显式地要求 `A` 和 `B`
            fn contains(&self, _: &A, _: &B) -> bool;

            // 未显式地要求 `A` 或 `B`
            fn first(&self) -> i32;

            // 未显式地要求 `A` 或 `B`
            fn last(&self) -> i32;
        }

        impl Contains<i32, i32> for Container {
            fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
                (&self.0 == number_1) && (&self.1 == number_2)
            }

            fn first(&self) -> i32 {
               self.0
            }

            fn last(&self) -> i32 {
                self.1
            }
        }

        // // 容器 `C` 就包含了 `A` 和 `B` 类型。
        // 鉴于此，必须指出 `A` 和 `B` 显得很麻烦。

        // 不使用关联类型
        fn difference<A, B, C>(container: &C) -> i32 where C: Contains<A, B> {
            container.last() - container.first()
        }

        let number_1 = 3;
        let number_2 = 10;

        let container = Container(number_1, number_2);

        println!("Does container contain {} and {}: {}",
                 &number_1, &number_2,
                 container.contains(&number_1, &number_2));

        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        println!("The difference is: {}", difference(&container));

    }
}