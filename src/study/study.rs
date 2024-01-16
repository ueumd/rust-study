pub fn add(base: u32) -> u32 {
    base + 1
}

pub(crate) fn hello() {
    let food = "彪悍的人生无需解释";
    let mut price = 66.66;

    let price3: i32 = -300;

    let checked = false;

    const PI: f64 = std::f64::consts::PI;
    println!("{}", PI); //输出 3.1415926

    println!("food is:{}", food);
    println!("price is:{}", price);

    println!("price3 is:{}", price3);

    println!("checked is:{}", checked);

    println!("Hello, world!");

    price = 88.88;
    println!("price is:{}", price);

    //不包含5
    for num in 1..5 {
        println!("num is {}", num)
    }

    //包含5
    for num in 1..=5 {
        println!("num is {}", num)
    }
}
