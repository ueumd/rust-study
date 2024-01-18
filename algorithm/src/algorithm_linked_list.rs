#[cfg(test)]
mod algorithm {
    #[derive(Debug)]
    struct Node<T> {
        elem: T,
        next: Link<T>
    }

    /*
    Option类型在Rust中用于表示可能为空的值。它有两个可能的取值：Some(value)表示有值，None表示无值
    */

    // 节点 连 接用 Box 指针 （ 大小确定 ）， 因 为 确 定 大 小 才 能 分 配 内 存
    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug)]
    struct List<T> {
        size: usize,  // 链表 节 点数
        head: Link<T> // 头节点
    }

    #[allow(dead_code)]
    impl<T> List<T> {
        fn new() -> Self {
            List {size: 0, head: None}
        }

        fn is_empty(&self) -> bool {
            self.size == 0
        }

        fn size(&self) -> usize {
            self.size
        }

        // 新 节 点 总 是 加 到 头 部
        fn push(&mut self, elem: T) -> &mut Self {
            let node =Box::new( Node {
                elem: elem,
                next: self.head.take()
            });

            self.head = Some(node);
            self.size += 1;
            self
        }

        fn pop(&mut self) -> Option<T> {
            //  删除链表头节点，不需要参数，返回 Node。
            self.head.take().map(|node| {
                self.head = node.next;
                self.size -= 1;
                node.elem
            })
        }

        // peek 不改变值 ， 只 能 是引 用
        fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.elem)
        }

        // peek_mut 可改变值 ， 是 可 变引 用
        fn peek_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|node| &mut node.elem)
        }

        // 以 下 是 实 现 的 三 种 迭 代 功 能
        // into_iter: 链表改变 ， 成 为 迭代 器
        // iter: 链表不变 ， 只 得 到 不 可 变 迭 代 器
        // iter_mut: 链表不变 ， 得 到 可 变 迭 代 器
        fn iter(&self) -> Iter<T> {
            Iter { next: self.head.as_deref() }
        }

    }

   // struct IntoIter<T>(List<T>);
   //
   //  impl <T> Iterator for IntoIter<T> {
   //      type Item = T;
   //      fn next(&mut self) -> Option<Self::Item> {
   //          self.0.pop()
   //      }
   //  }

    struct Iter<'a, T: 'a> { next: Option<&'a Node<T>> }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                &node.elem
            })
        }
    }

    #[test]
    fn test() {
        let mut list = List::new();
        list.push(1).push(2).push(3).push(4);

        println!("list: {:?}", list);

        println!("{}", list.is_empty());
        println!("{}", list.size());
        println!("{:?}", list.pop());

        for v in list.iter() {
            println!("value:{}", v)
        }
    }
}