// https://zhuanlan.zhihu.com/p/127365605
trait Hello {
    // 默认实现子方法体
    fn say_hi(&self) {
        println!("hi")
    }
}

trait Work {
    // 无默认实现
    fn working(&self);
}

struct Student {}

// 默认实现 + 无Override
impl Hello for Student {}

struct Teacher {}

// Override
impl Hello for Teacher {
    fn say_hi(&self) {
        println!("hi, I'm teacher Lee.");
    }
}

impl Work for Teacher {
    fn working(&self) {
        println!("hi, I'm  working...");
    }
}

//--------Trait与泛型--------------

// trait Animal<T> {
//     fn run<T>(&self, i: T) -> String;
// }

fn test1() {
    let s = Student {};
    s.say_hi();

    let t = Teacher {};
    t.say_hi();
    t.working();
}

pub fn init() {
    test1();
}
