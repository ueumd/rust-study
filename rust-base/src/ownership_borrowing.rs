#[cfg(test)]
/*
    Rust 所有权详解 和 引用
    在 Rust 中，所有权的概念根植于语言自身并且由编译器在编译时检查。
    每个值都有一个决定其生死的所有者，当这个所有者被释放（在 Rust 中，称为 Drop）时，它所拥有的值占用的的内存也会被释放。

    Rust 提供了 Rc 和 Arc 两种引用计数类型。
    Rc 和 Arc(Atomic Reference Count) 的唯一区别是，Arc 是线程安全的，支持多线程，但是性能有影响，
    所以，如果不需要线程共享，应该使用 Rc。
*/
mod ownership_borrowing {
    /*
     1. 移动（Move）
    值的所有权可以通过移动操作进行转移。
    当将一个值赋值给另一个变量或作为函数参数传递时，所有权会从一个变量转移到另一个变量。
    */
    #[test]
    fn test() {
        let s1 = String::from("hello");

        // 字符串"hello"的所有权从 s1 移动到 s2
        // 所有权被移动后，不能再进行任何操作，它所拥有的值占用的的内存也会被释放
        let s2 = s1;

        // s1 不在拥有所有权，无法打印
        // println!("s1: {}", s1);

        println!("s2: {}", s2);

        /*
        2. 克隆（Clone）
        需要创建一个值的完全独立的副本，而不是移动所有权
        */
        let s11 = String::from("hello");

        // s1 不再拥有 "hello" 所有权，所以无法从s1克隆，只能再定义s11
        let s3 = s11.clone();
        println!("{} {}", s11, s3); // 正常打印 "hello hello"

    }

    /*
    3. 借用（Borrowing）
    借用允许我们暂时地借用值的引用，而不获取其所有权。
    */
    fn print_length(s: &str) {
        println!("Length: {}", s.len()); // 5
    }

    fn print_length2(s: String) {
        println!("Length: {}", s.len()); // 5
    }

    #[test]
    fn test2() {
        let s = String::from("hello");
        print_length(&s); // 借用

        println!("{}", s); // 正常打印 "hello"

        print_length2(s); // 移动

        // 不再拥所有权，无法打印
        // println!("{}", s);
    }

    /*
    4. 生命周期（Lifetime）
    在借用值时，需要考虑值的生命周期。
    生命周期是指值在内存中存在的有效范围。Rust使用生命周期注解来确保借用值的引用在有效范围内。
    */

    fn print_length_lifetimes<'a>(s: &'a str) {
        println!("Length: {}", s.len());
    }

    #[test]
    fn test3() {
        let s = String::from("hello");
        print_length_lifetimes(&s); // 借用
        println!("{}", s); // 正常打印 "hello"
    }
}


/*
引用
一、什么是引用？
引用是指向数据的指针，它允许我们以只读或可变的方式访问数据，而不获取数据的所有权。
引用的存在使得在Rust中可以进行借用操作，实现灵活的数据共享和临时访问，同时保证了内存安全。

四、引用的规则
在使用引用时，需要遵守一些规则以确保内存安全：
    同一时间只能存在一个可变引用或多个不可变引用，但不能同时存在可变引用和不可变引用。
    引用必须始终有效，即被引用的数据不能在引用的生命周期内被销毁。
*/
mod ownership_borrowing2{

    // 不可变引用
    #[test]
    fn test1() {
        // 可变变量x 不可变引用y
        let mut x = 5;
        println!("{}", x); // 5

        x = 10;

        let y = &x;   // 不可变引用

        println!("y: {}", y);  // 10
        println!("x: {}", x);  // 10
    }

    // 可变引用
    #[test]
    fn test2() {
        let mut x = 5;
        let y = &mut x; // 可变引用

        // 通过 y 修改 x的值
        // 使用解引用操作符*来获取引用所指向的值，并进行修改。
        *y = 500;

        // 必须先打印y 再打印x。
        println!("y: {}", y);  // 500
        println!("x: {}", x);  // 500
    }

    #[test]
    fn test3() {
        let mut data = vec![1, 2, 3, 4, 5];

        // 使用不可变引用读取数据
        let slice = &data[1..3];
        println!("Slice: {:?}", slice); // Slice: [2, 3]

        // 使用一个新的作用域
        {
            // 使用可变引用修改数据
            let mut_ref = &mut data;
            mut_ref.push(6);

            // 这里会报错，在可变引用的作用域内打印，避免脏数据
            // println!("Slice: {:?}", data);

            println!("Modified Data: {:?}", mut_ref); // Modified Data: [1, 2, 3, 4, 5, 6]
        } // 在这里，mut_ref 的作用域结束


        // 这里会报错，因为存在不可变引用slice和可变引用mut_ref
        // 因为在原作用域内同时存在 slice（不可变引用）和 mut_ref（可变引用）违反了Rust的借用规则。
        // println!("Slice: {:?}", slice);

        // 在不可变引用以及可变引用的作用域外打印
        println!("Slice: {:?}", data); // Slice: [1, 2, 3, 4, 5, 6]

    }

}
