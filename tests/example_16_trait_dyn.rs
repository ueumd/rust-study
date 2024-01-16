#[cfg(test)]

mod example_16 {
    /*
   trait 面向对象
   */
    #[test]
    fn test() {
        struct Sheep {
            naked: bool,
            name: &'static str
        }

        trait Animal {
            // 静态方法-
            fn new(name: &'static str) -> Self;

            // 实例方法
            fn name(&self) -> &'static str;
            fn noise(&self) -> &'static str;

            // trait 可以提供默认的方法定义。
            fn talk(&self) {
                println!("{} says {}", self.name(), self.noise())
            }
        }

        impl Sheep {
            fn is_naked(&self) -> bool {
                self.naked
            }

            fn shear(&mut self) {
                if self.is_naked() {
                    // 实现者可以使用它的 trait 方法。
                    println!("{} is already naked...", self.name());
                } else {
                    println!("{} gets a haircut!", self.name);

                    self.naked = true;
                }
            }

        }

        impl Animal for Sheep {
            fn new(name: &'static str) -> Self {
                Sheep {name: name, naked: false}
            }

            fn name(&self) -> &'static str {
                self.name
            }

            fn noise(&self) -> &'static str {
                if self.is_naked() {
                    "baaaaah?"
                } else {
                    "baaaaah!"
                }
            }

            fn talk(&self) {
                // 例如我们可以增加一些安静的沉思。
                println!("{} pauses briefly... {}", self.name, self.noise());
            }
        }

        // 这种情况需要类型标注。
        let mut dolly: Sheep = Animal::new("Dolly");
        dolly.talk();
        dolly.shear();
        dolly.talk();


        let mut dolly2: Sheep = Sheep::new("Dolly2");
        dolly2.talk();
        dolly2.shear();
        dolly2.talk();
    }


    // dyn
// 动态分发
    /*
    https://rustwiki.org/zh-CN/rust-by-example/trait/dyn.html

    Rust 编译器需要知道 每个函数 的返回类型 需要多少空间
    这意味着所有函数都必须返回一个具体类型。
    与其他语言不同，如果你有个像 Animal 那样的的 trait，则不能编写返回 Animal 的函数，因为其不同的实现将需要不同的内存量。
    */
    #[test]
    fn test2() {
        struct Sheep {}
        struct Cow {}

        trait Animal {
            // 实例方法签名
            fn noise(&self) -> &'static str;
        }

        // 实现 `Sheep` 的 `Animal` trait。
        impl Animal for Sheep {
            fn noise(&self) -> &'static str {
                "baaaaah!"
            }
        }
        // 实现 `Cow` 的 `Animal` trait。
        impl Animal for Cow {
            fn noise(&self) -> &'static str {
                "moooooo!"
            }
        }

        // 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体。
        // 动态分发
        fn random_animal(random_number: f64) -> Box<dyn Animal> {
            if random_number < 0.5 {
                Box::new(Sheep{})
            } else {
                Box::new(Cow{})
            }
        }

        // 返回单个
        fn choose_cow() -> impl Animal {
            Cow{}
        }

        fn choose_animal2(animal: Box<dyn Animal>)  -> &'static str{
            // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
            animal.noise()
        }

        // 静态分发 使用泛型实现
        fn choose_animal<T: Animal>(animal: T) -> &'static str {
            animal.noise()
        }

        let animal = random_animal(0.234);
        println!("You are randomly chosen an animal, and it says {}", animal.noise());


        let animal2 = choose_cow();
        println!("animal it says {}", animal2.noise()); // animal it says moooooo!


        let noise = choose_animal(Cow{});
        println!("animal it says {}",noise); // animal it says moooooo!

        let noise2 = choose_animal2(Box::new(Sheep{}));
        println!("animal it says {}",noise2); //animal it says baaaaah!


    }

    // https://blog.csdn.net/cacique111/article/details/126323718
    #[test]
    fn test3() {
        #[derive(Debug)]
        struct Button {
            width: u32,
            height: u32,
            label: String,
        }
        #[derive(Debug)]
        struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        }

        trait Draw {
            fn draw(&self);
        }

        impl Draw for Button {
            fn draw(&self) {
                println!("button: {:?}", *self);
            }
        }

        impl Draw for SelectBox {
            fn draw(&self) {
                println!("SelectBox:  {:?}", *self);
            }
        }

        struct Screen {
            // Button 和 SelectBox 都实现了 Draw 特征
            // 我们填什么？
            // components: Vec<?>,

            // 动态数组components  里面元素的类型是 Draw 特征对象
            // 动态分发 dynamic dispatch
            components: Vec<Box<dyn Draw>>,
        }

        impl Screen {
            fn run(&self) {
                for component in self.components.iter() {
                    component.draw()
                }
            }
        }

        // 改用泛型 （静态分发）
        struct Screen2<T: Draw> {
            // 这种写法限制了 Screen 实例的 Vec<T> 中的每个元素必须是 Button 类型或者全是 SelectBox 类型
            // 如果只需要同质（相同类型）集合，更倾向于这种写法：使用泛型和 特征约束，因为实现更清晰，且性能更好(特征对象，需要在运行时从 vtable 动态查找需要调用的方法)。
            components: Vec<T>
        }

        impl<T> Screen2<T> where T: Draw{
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw()
                }
            }
        }

        let screen = Screen {
            components: vec![
                Box::new(Button{
                    width: 50,
                    height: 10,
                    label: "OK".to_string(),
                }),
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No")
                    ],
                })
            ],
        };
        screen.run();


        let screen2 = Screen2 {
            components: vec![
                Button{
                    width: 50,
                    height: 10,
                    label: "OK".to_string(),
                }
            ],
        };
        screen2.run();

    }
}