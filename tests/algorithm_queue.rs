#[cfg(test)]
mod queue {
    /*
    队列: 先进先出
    入队  -->  4 2 1 2 7 8 3 5 0 1  --> 出队
    */
    #[derive(Debug)]
    struct Queue<T> {
        cap: usize,// 容量
        data: Vec<T> // 数据容器
    }

    #[allow(dead_code)]
    impl <T> Queue<T> {
        fn new(size: usize) -> Self {
            Queue {
                cap: size,
                data: Vec::with_capacity(size)
            }
        }

        // 判 断 是 否 有 剩 余 空 间 、 有 则 数 据 加 入 队 列
        fn enqueue(&mut self, val: T) -> Result<(), String> {
          if Self::size(&self) == self.cap {
              return Err("No space available".to_string())
          }
          self.data.insert(0, val);
          Ok(())
        }

        // 数据出队
        fn dequeue(&mut self) -> Option<T> {
           if Self::size(&self) > 0 {
               self.data.pop()
           } else {
               None
           }
        }


        fn size(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            0 == Self::size(&self)
        }
    }

    /*
    击鼓传花
    入队----->  Brad Kew Jane Susan David Shieber  -----> 出队
    */
    fn hot_potato(names: Vec<&str>, num: usize) -> &str {
        let mut q = Queue::new(names.len());

        for name in names {
            let _rm = q.enqueue(name);
        }

        println!("queue content : {:?}", q);

        while q.size() > 1 {
            for _i in 0..num {
                let name = q.dequeue().unwrap();
                let _rm = q.enqueue(name);
            }

            // 出入 栈 达到 num 次 ， 删除一人
            let _rm = q.dequeue();
        }

        q.dequeue().unwrap()

    }

    #[test]
    fn test() {
        let mut queue = Queue::new(3);
        let _r1 = queue.enqueue(1);
        let _r2 = queue.enqueue(2);
        let _r3 = queue.enqueue(3);

        println!("size: {}, empty: {}", queue.size(), queue.is_empty());
        println!("queue content : {:?}", queue);

        if let Err(error) = queue.enqueue(4) {
            println!("Enqueue error: {error}");
        }

        if let Some(data) = queue.dequeue() {
            println!("data: {data}");
        } else {
            println!("empty queue");
        }

        println!("queue content : {:?}", queue);
    }

    #[test]
    fn test2() {
        let name = vec!["Shieber","David","Susan","Jane","Kew","Brad"];
        let rem = hot_potato(name, 8);
        println!("The left person is {rem}");
    }

}