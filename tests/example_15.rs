#[cfg(test)]
mod example {

    // RAII（Resource Acquisition Is Initialization，资源获取即初始化
    #[test]
    fn test() {
        // 在堆上分配一个整型数据
        let _box2 =Box::new(3i32);
        {
            // 在堆上分配一个整型数据
            let _box3 = Box::new(4i32);
            // `_box3` 在这里被销毁，内存得到释放
        }

        fn create_box() {
            // 在堆上分配一个整型数据
            let _box1 = Box::new(3i32);
            // `_box1` 在这里被销毁，内存得到释放
        }

        for _ in 0u32..1_000 {
            create_box()
        }
    }

    // 所有权和移动 (栈)
    #[test]
    fn test2() {

        // 栈分配的整型
        let x = 5u32;

        println!("x: {}", x); // x: 5

        // 将 `x` *复制*到 `y`——不存在资源移动
        let y = x;
        println!("x is {}, and y is {}", x, y); // x is 5, and y is
    }

    // 所有权和移动 (堆)
    #[test]
    fn test3() {
        // `a` 是一个指向堆分配的整数的指针
        let a = Box::new(5i32);
        println!("a contains: {}", a); //a contains: 5

        // *移动* `a` 到 `b`
        // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向 同一个堆分配的数据
        // 但是现在是 `b` 拥有它。
        let b = a;

        // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
        // println!("a contains: {}", a);

        println!("b contains: {}", b); // b contains: 5

        fn destroy_box(c: Box<i32>){
            println!("Destroying a box that contains {}", c);
            // `c` 被销毁且内存得到释放
        }

        // 此函数从 `b` 中取得堆分配的内存的所有权
        destroy_box(b);

        // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的。
        // 报错！和前面出错的原因一样。
        // println!("b contains: {}", b); // b contains: 5
    }

    // 可变性 当所有权转移时，数据的可变性可能发生改变。
    #[test]
    fn test4() {
        let immutable_box = Box::new(5u32);
        println!("immutable_box contains {}", immutable_box);

        // *移动* box，改变所有权（和可变性） mut关键字
        let mut mutable_box = immutable_box;
        println!("mutable_box contains {}", mutable_box);

        // 修改 box 的内容
        *mutable_box = 4;
        println!("mutable_box now contains {}", mutable_box);
    }

    // 部分移动
    #[test]
    fn test5() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        let person = Person {
            name: String::from("Alice"),
            age: 20,
        };

        // `name` 从 person 中移走，但 `age` 只是引用
        let Person {name, ref age} = person;

        println!("The person's age is {}", age);

        println!("The person's name is {}", name);

        // 报错！部分移动值的借用：`person` 部分借用产生
        //println!("The person struct is {:?}", person);
        // println!("The person's name from person struct is {}", person.name);

        // `person` 不能使用，但 `person.age` 因为没有被移动而可以继续使用
        println!("The person's age from person struct is {}", person.age);
    }

    // 借用
    #[test]
    fn test6() {
        // 此函数取得一个 box 的所有权并销毁它
        fn _eat_box_i32(boxed_i32: Box<i32>) {
            println!("Destroying box that contains {}", boxed_i32);
        }

        // 此函数借用了一个 i32 类型 指针地址
        fn borrow_i32(borrowed_i32: &i32) {
            println!("This int is: {}", borrowed_i32);
        }

        // 创建一个装箱的 i32 类型，以及一个存在栈中的 i32 类型。
        let boxed_i32 = Box::new(5_i32);

        // eat_box_i32(boxed_i32);

        // 引用
        borrow_i32(&boxed_i32);
        println!("boxed_i32 is {}", boxed_i32)
    }

    #[test]
    fn p1() {
        let arr = [1, 2, 3, 4];
        let addr = &arr;
        println!("addr:  {:p}", addr); // addr:  0xa9faf0

        let mut vec = vec![1, 2, 3]; // 要获取可变引用，必须先声明可变绑定
        let new_vec = &mut vec; // 通过 &mut 得到可变引用
        new_vec.push(4);
        println!("new_vec: {:?}", new_vec); // // [1, 2, 3, 4]

        // 不可变引用&mut
        let mut str1 = String::from("hello");

        let m1 = &mut str1;
        // let m2 = &mut str1; //  编译错误 只能有一个可变引用

        // println!("m1: {}, m2: {}", m1, m2);
        println!("m1: {}", m1);

        // 共享引用&
        //  let str2 = String::from("world");
        //let r3 = &mut str2; // 不能同时拥有不可变引用和可变引用

        let str2 = String::from("world");
        let r1 = &str2;
        let r2 = &str2; // 没问题

        println!("r1: {}, r2: {}", r1, r2);
    }

    #[test]
    fn box1() {
        // 创建一个在栈上的 num 对象
        let num = 123;
        let num_size = std::mem::size_of_val(&num);
        if num_size <= std::mem::size_of::<usize>() {
            println!("num is likely stored on the stack."); // statck
        } else {
            println!("num is likely stored on the heap.");
        }

        // 将 num 对象移动到堆上，并用 Box<T> 进行包装
        let boxed_num = Box::new("123");
        let boxed_num_size = std::mem::size_of_val(&*boxed_num);

        if boxed_num_size <= std::mem::size_of::<usize>() {
            println!("box num is likely stored on the stack.");
        } else {
            println!("box num is likely stored on the heap."); // heap
        }
    }


    // ref 模式
    #[test]
    fn test_ref() {
        let c = 'Q';
        // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号。
        let ref ref_c1 = c;
        let ref_c2 = &c;

        println!("ref_c1  equals ref_c2: {}", *ref_c1 == *ref_c2);


        #[derive(Clone, Copy)]
        struct Point { x: i32, y: i32 }

        let point = Point { x: 0, y: 0 };


        // `point` 的可变拷贝
        let mut mutable_point = point;
        println!("point is ({}, {})", point.x, point.y); // point is (0, 0)

        mutable_point.x = 100;
        mutable_point.y = 100;
        println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y); // mutable_point is (100, 100)
    }

}