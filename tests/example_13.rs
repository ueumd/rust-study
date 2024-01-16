#[cfg(test)]
mod example {

    // 条件编译

    // 只有linux秒统才会编译
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!");
    }

    // And this function only gets compiled if the target OS is *not* linux
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!");
    }

    #[test]
    fn test() {
        are_you_on_linux();

        if cfg!(target_os = "linux") {
            println!("Yes. It's definitely linux!");
        } else {
            println!("Yes. It's definitely *not* linux!");
        }
    }

    #[test]
    fn test2() {
     // 自定义条件  rustc --cfg some_condition custom.rs && ./custom

        #[cfg(some_condition)]
        fn conditional_function() {
            println!("condition met!")
        }


    }
}