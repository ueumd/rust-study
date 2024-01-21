#[cfg(test)]
mod macro_study {

    // match like arm for macro
    macro_rules! add {

        // 这里可以有多个分支

         // first arm match add!(1,2), add!(2,3) etc
        ($a:expr, $b:expr)=>{
           $a + $b
        };

        // Second arm match add!(1), add!(2) etc
        ($a:expr)=>{
            {
                $a
            }
        }
    }
    
    #[test]
    fn test() {
        let result = add!(1,2);
        println!("result: {}", result); //result: 3

        println!("add: {}", add!(100)); //10
    }

}