mod vec_study {

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

        for it in list.iter() {
            println!("it: {it}")
        }
        /*
        it: a
        it: A
        it: b
        it: B
        it: C
        it: D
        */
    }
}