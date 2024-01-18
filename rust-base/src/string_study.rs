mod string_study {

    /*
    去除空格
    */
    fn _trim(str: &str) -> String{
        let mut list = vec![];

        // 移出前面的空格
        for (_i, v) in  str.chars().enumerate() {
            if !v.is_whitespace() {
                // println!("v is: {}", v);
                list.push(v);
            } else {
                if !list.is_empty() && v.is_whitespace() {
                    list.push(v);
                }
            }
        }

        // list.iter().rev().for_each(|v| {
        //     if v.is_whitespace() {
        //         list.remove(list.len() - 1)
        //     }
        // });

        let mut remove_indexs: Vec<usize> = Vec::new();
        for (i, v) in list.iter().rev().enumerate() {
            if (*v).is_whitespace() {
                println!("i: {}, v: {}",i, *v);
                remove_indexs.push(i)
            } else {
                break
            }
        }

        // 移出尾部空格
        for _i in remove_indexs {
            list.pop();
        }


        println!("list: {:?}", list);
        let s: String = list.iter().collect();

        return s;
    }

    #[test]
    fn test() {

        /*
        https://www.cnblogs.com/BuzzWeek/p/16861661.html
        */

        let str = String::from("   he  llo   ");
        // let str = String::from("hello");
        // println!("{:?}", str.chars());
       let s = _trim(&str);
        println!("s is: {}", s)
    }
}