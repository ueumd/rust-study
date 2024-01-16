/**

&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn init() {
    let str1 = String::from("abcd");
    let str2 = "xyz";

    let result = longest(str1.as_str(), str2);
    println!("The longest string is {}", result);
}
