fn array_tst() {
    // 拥有相同类型 T 的对象的集合，在内存中是连续存储的。
    let mut arr2 = ["Go语言极简一本通", "Go语言微服务架构核心22讲", "100"];

    arr2[1] = "Vue";

    for item in arr2 {
        println!("{}", item)
    }
}

pub fn init() {
    array_tst();
}
