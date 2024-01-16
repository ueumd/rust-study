#[cfg(test)]
mod example_2 {
    use std::mem;

    #[test]
    fn test() {


        let long_tuple = (1u8, 2u16, 3u32, 4u64,
                          -1i8, -2i16, -3i32, -4i64,
                          0.1f32, 0.2f64,
                          'a', true);

        println!("long the first value: {}", long_tuple.0);


        let pair = (1, true);
        println!("pair is : {:?}", pair);
    }

    #[test]
    fn test2() {

        // 定长数组 [type:len] , 注意中间是冒号
        let xs: [i32; 5] = [1, 2, 3, 4, 5];

        // 下标 [] 式访问，与元组不同.
        println!("xs the first value: {}", xs[0]);
        println!("len: {}", xs.len());

        // 数组是在栈中分配的
        println!("array occupies {} bytes", mem::size_of_val(&xs));

        // 越界的下标会引发致命错误（panic）
        // println!("{}", xs[5]);

    }
}