use std::mem;

/**
 引用（Reference），共享引用&，不可变引用&mut
原生指针（Raw Pointer），*const和*mut
智能指针（Smart Pointer），Box<T>，Rc<T>等
 */
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
    let mut str2 = String::from("world");
    let r1 = &str2;
    let r2 = &str2; // 没问题
                    // let r3 = &mut str2; // 不能同时拥有不可变引用和可变引用
    println!("r1: {}, r2: {}", r1, r2);
}

fn box1() {
    // 创建一个在栈上的 num 对象
    let num = 123;
    let num_size = mem::size_of_val(&num);
    if num_size <= mem::size_of::<usize>() {
        println!("num is likely stored on the stack."); // statck
    } else {
        println!("num is likely stored on the heap.");
    }

    // 将 num 对象移动到堆上，并用 Box<T> 进行包装
    let boxed_num = Box::new("123");
    let boxed_num_size = mem::size_of_val(&*boxed_num);

    if boxed_num_size <= mem::size_of::<usize>() {
        println!("box num is likely stored on the stack.");
    } else {
        println!("box num is likely stored on the heap."); // heap
    }
}

// 管理大对象
// https://zhuanlan.zhihu.com/p/622242447
fn box2() {
    let arr: [i32; 1000000] = [1; 1000000];
    let arr1 = arr;
    let arr2 = arr1;

    // 上述代码中 从arr到arr1再到arr2的拷贝就是这样，甚至会产生内存溢出crash，
    // 而在使用box的情况下，因为只是进行了指针的转移，消耗极少；
    let box_a = Box::new(arr);
    let box_b = box_a;
    let box_c = box_b;
}

fn box3() {}

pub fn init() {
    p1();

    box1();
}
