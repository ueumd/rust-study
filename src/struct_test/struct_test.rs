enum IpAddrKind {
    V4,
    V6
}


#[derive(Debug)]
struct Study {
    name: String,
    target: String,
    spend: i32
}



fn test1(){
    let s = Study {
        name: String::from("从0到Go语言微服务架构师"),
        target: String::from("全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。"),
        spend: 3,
    };
    println!("{:?}", s);

    println!("{}", s.name);
}

fn init() {
    test1();
}