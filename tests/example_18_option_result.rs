
mod example_18 {
    /*
    使用 ? 解开 Option

    比 match xx {} 更优雅
    */
    #[test]
    fn test() {
        #[derive(Clone, Copy)]
        struct PhoneNumber {
            area_code: Option<u8>,
            _number: u32,
        }

        #[derive(Clone, Copy)]
        struct Job {
            phone_number: Option<PhoneNumber>,
        }


        struct Person {
            job: Option<Job>
        }

        impl Person {
            fn work_phone_area_code(&self) -> Option<u8> {
                // ********** 没有`？`运算符的话，这将需要很多的嵌套的 `match` 语句。
                // 这将需要更多代码——尝试自己编写一下，看看哪个更容易。
                self.job?.phone_number?.area_code
            }
        }

        let p = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(61),
                    _number: 439222222,
                })
            })
        };

        println!("{}", p.work_phone_area_code() == Some(60));
        assert_eq!(p.work_phone_area_code(), Some(61));

    }



    /*
    组合算子：map
    Option 有一个内置方法 map()，这个组合算子可用于 Some -> Some 和 None -> None 这样的简单映射。
    多个不同的 map() 调用可以串起来，这样更加灵活。
    */
    #[test]
    fn test2() {
        #![allow(dead_code)]

        #[derive(Debug)] enum Food { Apple, Carrot, Potato }

        // 削皮
        #[derive(Debug)] struct Peeled(Food);

        // 切食物
        #[derive(Debug)] struct Chopped(Food);

        // 烹饪
        #[derive(Debug)] struct Cooked(Food);

        // 削皮。如果没有食物，就返回 `None`。否则返回削好皮的食物。
        fn peel(food: Option<Food>) -> Option<Peeled> {
            match food {
                Some(food) => Some(Peeled(food)),
                None       => None,
            }
        }

        // 切食物。如果没有食物，就返回 `None`。否则返回切好的食物。
        fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
            match peeled {
                Some(Peeled(food)) => Some(Chopped(food)),
                None               => None,
            }
        }

        // 烹饪食物。这里，我们使用 `map()` 来替代 `match` 以处理各种情况。
        fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
            chopped.map(|Chopped(food)| Cooked(food))
        }


        // 这个函数会完成削皮切块烹饪一条龙。我们把 `map()` 串起来，以简化代码。
        fn process(food: Option<Food>) -> Option<Cooked> {
            food.map(|f| Peeled(f))
                .map(|Peeled(f)| Chopped(f))
                .map(|Chopped(f)| Cooked(f))
        }

        // 在尝试吃食物之前确认食物是否存在是非常重要的！
        fn eat(food: Option<Cooked>) {
            match food {
                Some(food) => println!("Mmm. I love {:?}", food),
                None       => println!("Oh no! It wasn't edible."),
            }
        }

        let apple = Some(Food::Apple);
        let carrot = Some(Food::Carrot);
        let potato = None;

        let cooked_apple = cook(chop(peel(apple)));
        let cooked_carrot = cook(chop(peel(carrot)));

        // 现在让我们试试看起来更简单的 `process()`。
        let cooked_potato = process(potato);

        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }


    #[test]
    fn result1() {
        fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
            let first_number = first_number_str.parse::<i32>().unwrap();
            let second_number = second_number_str.parse::<i32>().unwrap();
            first_number * second_number
        }

        let twenty = multiply("10", "2");
        println!("double is {}", twenty);

        //  产生panic 退出程序
        // let tt = multiply("t", "2");
        // println!("double is {}", tt);


        // 修改 使用Result 匹配
        fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, std::num::ParseIntError> {
            match first_number_str.parse::<i32>() {
                Ok(first_number) => {
                    match second_number_str.parse::<i32>() {
                        Ok(second_number) => {
                            Ok(first_number * second_number)
                        },
                        Err(e) => Err(e),
                    }
                },
                Err(e) => Err(e),
            }
        }
        let tt = multiply2("t", "2");
        println!("double is {:?}", tt); // double is Err(ParseIntError { kind: InvalidDigit })

        println!("{:?}", multiply2("10", "2"));



        // 就像 `Option` 那样，我们可以使用 `map()` 之类的组合算子。
        // 除去写法外，这个函数与上面那个完全一致，它的作用是：
        // 如果值是合法的，计算其乘积，否则返回错误。
        fn multiply3(first_number_str: &str, second_number_str: &str) -> Result<i32, std::num::ParseIntError> {
            first_number_str.parse::<i32>().and_then(|first_number| {
                second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
            })
        }

        println!("multiply3 {:?}", multiply3("10", "2")); // multiply3 Ok(20)
        println!("multiply3 {:?}", multiply3("10", "ttt")); // multiply3 Err(ParseIntError { kind: InvalidDigit })

        // 引入 ? https://rustwiki.org/zh-CN/rust-by-example/error/result/enter_question_mark.html
        fn multiply4(first_number_str: &str, second_number_str: &str) -> Result<i32,  std::num::ParseIntError> {
            let first_number = first_number_str.parse::<i32>()?;
            let second_number = second_number_str.parse::<i32>()?;

            Ok(first_number * second_number)
        }

        println!("multiply4 {:?}", multiply3("10", "2")); // multiply4 Ok(20)
        println!("multiply4 {:?}", multiply3("10", "ttt")); // multiply4 Err(ParseIntError { kind: InvalidDigit })
    }
}