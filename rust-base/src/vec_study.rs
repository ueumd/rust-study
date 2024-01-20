mod vec_study {

    /*
    https://rustwiki.org/zh-CN/std/vec/struct.Vec.html
    https://rust-book.junmajinlong.com/ch7/01_vec_basic.html

    Vec::new()创建空的vec
    Vec::with_capacity()创建空的vec，并将其容量设置为指定的数量
    vec![]宏创建并初始化vec(中括号可以换为小括号或大括号)
    vec![v;n]创建并初始化vec，共n个元素，每个元素都初始化为v
    */
    #[test]
    fn test() {
        let mut list = Vec::new();

        // push 添加尾部
        list.push('A');
        list.push('B');
        list.push('C');

        println!(" {:?}", list); //  ['A', 'B', 'C']

        list.push('D');

        println!(" {:?}", list); //  ['A', 'B', 'C', 'D']

        // 添加首部
        list.insert(0, 'a');
        println!(" {:?}", list); //  ['a', 'A', 'B', 'C', 'D']

        // 指定位置插入
        list.insert(2, 'b');
        println!(" {:?}", list); //  ['a', 'A', 'b', 'B', 'C', 'D']

        // 循环
        for it in list.iter() {
            print!("{it} ") // a A b B C D
        }
        println!("\n");

        // 下标访问
        println!("list[1]: {}", list[1]); // list[1]: A

        println!("remove list[1] {}", list.remove(1)); // remove list[1] A

        println!(" {:?}", list); //  ['a', 'b', 'B', 'C', 'D']

        // 尾部删除
        while let Some (top) = list.pop(){
            print!("{top} "); // D C B b a
        }
        println!("\n");

        println!(" {:?}", list); //[]

    }



    /*
    Rust中的迭代器的使用：map转换、filter过滤、fold聚合、chain链接
    https://juejin.cn/post/7221159922905333797
    */
    #[test]
    fn test2() {
        let v = vec![1, 2, 3];

        /*
        map：转换数据。接受一个闭包并为迭代器中的每个元素调用该闭包，然后返回一个新的迭代器，其中包含闭包返回的值。
        */

        let v_squared = v.iter().map(|x| x* x);
        let v_squared2: Vec<i32> = v.iter().map(|x| x * x).collect();

        println!("v: {:?}", v); // v: [1, 2, 3]
        println!("v_squared: {:?}", v_squared); // v_squared: Map { iter: Iter([1, 2, 3]) }

        println!("v_squared2: {:?}", v_squared2); // v_squared2: [1, 4, 9]


        // filter：过滤数据。接受一个闭包并为迭代器中的每个元素调用该闭包。如果闭包返回true，则元素将包含在新的迭代器中。
        let v_even: Vec<&i32> = v.iter().filter(|x| {
                println!("x: {x}");
                *x % 2== 0
        }).collect();
        println!("v_even: {:?}", v_even); // v_squared2: [1, 4, 9]

    }

    /*
        rust中String，＆str，Vec <u8>和＆[u8]的惯用转换
        https://zhuanlan.zhihu.com/p/372082802
        https://zhuanlan.zhihu.com/p/606455898
    */
    #[test]
    fn test3() {

    }

    /*
    删除重复的元素
    比如一个vec中是原始数据是 vec![1,2,3,3,4,5] ，然后我要在遍历中把等于c的元素删除掉,目的是得到vec![1,2,4,5]

      ["a", "b", "c", "c", "d", "e"]  ===========》 ["a", "b", "d", "e"]

     ['c', 'a', 'z', 'a', 'x', 'a','c']===========> ['c', 'a', 'z', 'x']
    */
    #[test]
    fn test4() {

        let mut list:Vec<&str> = vec!["a", "b", "c", "c", "d", "e"];
        println!("before list is {:?}", list);
        // before list is ["a", "b", "c", "c", "d", "e"

        let mut indexs: Vec<usize> = Vec::new();

        /*
        iter()是不可变的引用，但是调用remove的时候删除一个元素得时候，对items是可变的引用了，所以一个变量不能既是可变引用又是不可变引用
        items.remove(index)
        */

        for (i, v) in list.iter().enumerate() {
            if *v == "c" {
                indexs.push(i);
            }
        }

        println!("index_list is {:?}", indexs);
        for i in indexs {
            list.remove(i);
        }
        println!("then index_list is {:?}", list);
        // then index_list is ["a", "b", "c", "e"]
    }

    #[test]
    fn test5() {
        let mut items:Vec<&str> = vec!["a", "b", "c", "c", "d", "e"];
        println!("before items is {:?}", items);
        // before items is ["a", "b", "c", "c", "d", "e"]

        // retain的意思是保留，所以这个方法的意思就是接收一个回调函数，然后回调函数里面返回true进行保留，返回false的就移除。
        items.retain(|item| if *item == "c" {false } else { true });
        println!("then items is {:?}", items);
        // then items is ["a", "b", "d", "e"]
    }
}