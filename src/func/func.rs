fn get_name(str:String){
    println!("str: {}", str);
}

fn tuple_test(){
    let t:(&str, &str) = ("Vue", "react");
    println!("t : {:?}", t);

    println!("{}", t.0);
    println!("{}", t.1);
}

fn array_tst() {
    let mut arr2=["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];

    arr2[1] = "Vue";

    for item in arr2 {
        println!("{}", item)
    }
}

// 值
fn double_price(mut price:i32) {
    price = price *2;
    println!("内部的 price {}", price) // 20
}

fn double_price2(price:&mut i32) {
    *price = *price*2;
    println!("内部的 price {}", price) // 20
}

fn slice_test() {
    let mut v = Vec::new();
    v.push("vue");
    v.push("react");
    v.push("angular");
    println!("len: {:?}", v.len());

    let s1 = &v[0..3]; // len: 3
    println!("s1:{:?}",s1); // s1:["vue", "react", "angular"]

    show_slice(s1);
    modify_slice(&mut v);
}

// 切片当参数
fn show_slice(s:&[&str]) {
    println!("show slice: {:?}", s); // show slice: ["vue", "react", "angular"]
}

// 可变切片
fn modify_slice(s: &mut[&str]) {
    s[0] ="已精通";
    println!("modify_slice:{:?}", s); // modify_slice:["已精通", "react", "angular"]
}


fn if_let() {
    let v = Some(3);
    if let Some(3) = v {
        println!("three")
    } else {
        println!("others")
    }
}


pub(crate) fn init() {

    let name:String = String::from("从0到Go语言微服务架构师");
    get_name(name);

    tuple_test();

    array_tst();

    let price = 10;
    double_price(price);
    println!("外部的price {}", price) ;// 10

    let mut price2 = 10;
    double_price2(&mut price2);
    println!("外部的price {}", price2); // 10

    slice_test();

    if_let();
}