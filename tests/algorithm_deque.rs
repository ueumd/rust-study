#[cfg(test)]
mod deque {
    /*
    双端队列: 栈 和 队列
    LIFO 和 FIFO 排序

    入队 -->                         <--  入队
               4 2 1 7 8 3 5 0 1               队首（front）
    出队 <--                         -->  出队

    */
    #[derive(Debug)]
    struct Deque<T> {
        cap: usize,// 容量
        data: Vec<T> // 数据容器
    }

    #[allow(dead_code)]
    impl <T> Deque<T> {
        fn new(size: usize) -> Self {
            Deque {
                cap: size,
                data: Vec::with_capacity(size)
            }
        }

        // Vec 末尾 为 队首
        fn add_front(&mut self, val: T) -> Result<(), String> {
            if Self::size(&self) == self.cap {
                return Err("No space available".to_string());
            }
            self.data.push(val);
            Ok(())
        }

        // Vec 首部 为 队尾
        fn add_rear(&mut self, val: T)  -> Result<(), String> {
            if Self::size(&self) == self.cap {
                return Err("No space available".to_string());
            }
            self.data.insert(0, val);
            Ok(())
        }

        // 从 队 首 移 除 数 据
        fn remove_front(&mut self) -> Option<T> {
            if Self::size(&self) > 0 {
                // 尾部是队首
                self.data.pop()
            } else { None }
        }

        // 从 队 尾 移 除 数 据
        fn remove_rear(&mut self) -> Option<T> {
            if Self::size(&self) > 0 {
                // 头部是队尾
               Some(self.data.remove(0))
            } else { None }
        }


        fn size(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            0 == Self::size(&self)
        }
    }


    #[test]
    fn test() {
        let mut queue = Deque::new(3);
        let _r1 = queue.add_front(1);
        let _r2 = queue.add_front(2);
        let _r3 = queue.add_front(3);

        println!("size: {}, empty: {}", queue.size(), queue.is_empty());
        println!("queue content : {:?}", queue); // queue content : Deque { cap: 3, data: [1, 2, 3] }

        if let Err(error) = queue.add_front(4) {
            println!("Enqueue error: {error}");
        }

        if let Some(data) = queue.remove_rear() {
            println!("data: {data}");  // data: 1
        } else {
            println!("empty queue");
        }

        println!("queue content : {:?}", queue); // queue content : Deque { cap: 3, data: [2, 3]
    }


}