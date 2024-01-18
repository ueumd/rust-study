#[cfg(test)]
mod data_type {

    /*
    Rust数据类型详解
    https://cloud.tencent.com/developer/article/2338748
    */


    /*
    Option数据类型
    https://rustwiki.org/zh-CN/std/option/enum.Option.html
    */
    #[test]
    fn test() {
        let mut op = Option::Some("hello");
        println!(" {:?}", op); // Some("hello")
        println!(" op.unwrap(): {}", op.unwrap()); //  op.unwrap(): hello

        println!("{:?}", op.take());//  Some("hello")
    }

    #[test]
    fn test2() {

        #[derive(Debug)]
        struct Node<T> {
            elem: T
        }

        let mut op = Some(Node{
            elem: 10
        });

        println!("{:?}", op); // Some(Node { elem: 10 })

        op.take().map(|node| {
            println!("node: {:?}", node) // node: Node { elem: 10 }
        });
    }
}