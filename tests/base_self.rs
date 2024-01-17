
/*
https://cloud.tencent.com/developer/article/2338755

Self  => self: Self             表示向函数传递的是一个对象，会发生所有权的转移，对象的所有权会传递到函数中。
&self => self: &Self            表示向函数传递的是一个引用，不会发生对象所有权的转移；
&mut self => self: &mut Self

self 当self用作函数的第一个参数时
self: Self。
&self 参数等价于    self: &Self
&mut self参数等价于  self: &mut Self
*/


trait Shape {
    fn area(&self) -> u32;

    fn clone(&self) -> Self;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    // 静态 返回自己
    fn new () -> Self {
        Rectangle {
            width: 10,
            height: 20
        }
    }

    fn width(self) -> u32{
        self.width
    }

    // 实例调用 可变和不可变都可以调用
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可变量 实例调用
    fn translate(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn clone(&self) -> Self {
       Self{
           width: self.width,
           height: self.height
       }
    }
}


#[test]
fn test() {
    // 不可变
    let rectangle = Rectangle::new();
    println!("rectangle {:?}", rectangle); // rectangle Rectangle { width: 10, height: 20 }
    println!("rectangle area is {}", rectangle.area()); // rectangle area is 200

    // 错误 无法借用不可变局部变量 `rectangle` 作为可变变量
    // translate 需要可变变量调用
    // rectangle.translate(1, 2);


    println!("rectangle width: {}", rectangle.width); // rectangle width: 10

    // 表示向函数传递的是一个对象，会发生所有权的转移，对象的所有权会传递到函数中。
    println!("rectangle width(): {}", rectangle.width()); // rectangle width(): 10

    // 再次调用 报错，没有所有权
    // println!("rectangle width: {}", rectangle.width);

    // 可变
    let mut rectangle2 = Rectangle::new();
    println!("rectangle2 {:?}", rectangle2); // rectangle rectangle2 { width: 10, height: 20 }

    // 错误 不调可用
    // println!("rectangle width: {}", rectangle2.width()); // rectangle width: 10

    // 改变 width 和 height
    rectangle2.translate(10, 10);
    println!("rectangle2 {:?}", rectangle2); // rectangle2 Rectangle { width: 10, height: 10 }


    println!("rectangle area is {}", rectangle2.area()); // rectangle area is 100
}