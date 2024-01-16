fn _base1() {

    // 变量 或 函数 前带_ 在未被使用时，可以不被警告
    let _x = 5;
    let _y = 10;

    const _MAX_POINTS: u32 = 100_000;


    // 解构
    let (a, mut b):(bool, bool) = (true, false);
    println!("a = {}, b = {}", a, b);

    b = true;
    assert_eq!(a, b);
    assert_eq!(a, b, "we are testing addition with {} and {}", a, b);

    let x = 5;
    println!("x is {}", x); // 5

    // 在main函数的作用域内对之前的x进行遮蔽

    // 注意let x 重新生成的变量, 两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配
    // 而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好
    let x = x + 1;
    println!("x is {}", x); // 6

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // 12
    }
    println!("The value of x is: {}", x); // 6


    // 由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节：
    let z = '中';
    let c = 'z';
    println!("字符中占用了{}字节的内存大小", std::mem::size_of_val(&z)); // 字符中占用了4字节的内存大小

    println!("字符z占用了{}字节的内存大小", std::mem::size_of_val(&c)); // 字符z占用了4字节的内存大小

    _test_fn(5, 6.1);

    println!("x+y = {}", _add(1, 3))

}

/**
函数要点
    函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
    函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
    每个函数参数都需要标注类型
*/

//   每个函数参数都需要标注类型!!!
// 这里去掉 x 或者 y 的任何一个的类型，都会报错
fn _test_fn(x: i32, y:f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


// 函数返回值
// 语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值
// 因此在上述函数体的三行代码中，前两行是语句，最后一行是表达式。
// 注意：表达式 没有;号结尾
fn _add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

// 基于表达式是函数式语言的重要特征，表达式总要返回值

fn _add(x: u32, y:u32) -> u32 {
    // 没有分号，会返回结果
    x + y // 这是一个表达式
}

fn _string_test() {
    let mut str = String::from("hello");
    str.push_str(", world!");
    println!("str is {}", str); // str is hello, world!


    let _s1 = String::from("hello");
    // let s2 = _s1; // 无效引用，编译报错

    // 可以进行克隆
    let _s2 = _s1.clone();

    // 字面量

    // s3 只是引用了存储在二进制中的字符串 "hello"，并没有持有所有权。
    let s3: &str = "hello";

    // 仅仅是对该引用进行了拷贝
    let mut s4 = s3;
    println!("s3: {}, s4：{}", s3, s4); // s3: hello, s4：hello

    s4 = "world";
    println!("s3: {}, s4：{}", s3, s4); // s3: hello, s4：hello
}


fn _main1() {
    let s = String::from("hello");  // s 进入作用域

    // 并不是拷贝，是移动
    _takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    // println!("s is {}", s);

    let x = 5;                 // x 进入作用域

    // 复制
    _makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
    println!("x is {}", x); // x is 5

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

fn _takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string); // hello

} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn _makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer); // 5
} // 这里，some_integer 移出作用域。不会有特殊操作


// -----------------------引用与解引用
fn _borrowing () {
    let x = 5;

    // 获取变量x的引用, 指针的地址
    let y = &x;

    // *y 解引用
    println!("x is {}, y is {}", x, *y); // x is 5, y is 5

    let s1 = String::from("hello");

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // 5

    // 正确修改方式 mut 关键字
    fn change(str: &mut String) {
        str.push_str(", world!");
    }
    let mut s2 = String::from("hello");
    change(&mut s2);

    println!("s2 is :{}", s2); // s2 is :hello, world!
}

//--------------------- 泛型
fn _add_i8(a: i8, b:i8) -> i8{
    a + b
}

fn _add_i16(a: i32, b:i32) -> i32{
    a + b
}

fn _add_i32(a: f64, b:f64) -> f64{
    a + b
}

// T 可以是任意类型，但不是所有类型都可以相加
// 需要用 std::ops::Add<Output = T> 对 T 进行限制
fn _add2<T: std::ops::Add<Output = T>>(a: T, b:T) -> T {
    a + b
}


fn _largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_f32(list: &[f32]) -> f32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// https://zhuanlan.zhihu.com/p/140066109
fn _largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {

    // let mut largest = list[0];
    let mut largest = &list[0];

   // for &item in list {
    for item in list.iter(){
        if item > largest {
            largest = item
        }
    }
    largest
}

// fn _add_main() {
//     println!("add i8: {}", _add_i8(2i8, 3i8));
//     println!("add i32: {}", _add_i16(20, 30));
//     println!("add f64: {}", _add_i32(1.23, 1.23));
//
//     println!("add f64: {}", _add2(1.23, 1.23));
//
//
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
// }

fn _main2() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = _largest_i32(&number_list);
    println!("The largest i32 is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = _largest_char(&char_list);
    println!("The largest char is {}", result);

    let f_list = vec![34.2, 5.0, 250f32, 10f32, 635.0];
    let result = _largest_f32(&f_list);
    println!("The largest f32 is {}", result);

    let f_list = vec![34.2, 5.0, 250f32, 10f32, 635.0];
    let result = _largest(&f_list);
    println!("The largest f32 is {}", result);

}

struct _Point<T, U>{
    x: T,
    _y: U,
}

impl <T, U> _Point<T, U> {
    fn _x(&self) ->&T {
        &self.x
    }
}

impl <T, U> _Point<T, U> {
    fn _mix_up<V, W>(self, other:_Point<V, W>) -> _Point<T, W> {
        _Point{
            x: self.x,
            _y: other._y
        }
    }
}

fn _main3() {
    let p = _Point{x: 5, _y:10};
    println!("p.x= {}", p._x());

    let p2 = _Point { x: "Hello", _y: 'c'};
    let p3 = p._mix_up(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3._y);
}

fn _init() {
    // _base1();
    // _string_test();
    // _main1();
    // _borrowing();
    _string_test();
    _main2();
    _main3();
}