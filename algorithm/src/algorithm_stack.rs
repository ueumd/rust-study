#[cfg(test)]
mod algorithm_stack {

    /*
    栈: 先进后出
    */
    #[derive(Debug)]
    struct Stack<T> {
        top: usize,// 栈顶
        data: Vec<T>
    }

    #[allow(dead_code)]
    impl <T> Stack<T> {
        fn new() -> Self {
            Stack {
                top: 0,
                data: Vec::new()
            }
        }

        // 链式调用
        fn push(&mut self, val: T) -> &mut Self {
            self.data.push(val);
            self.top += 1;
            self
        }

        fn pop(&mut self) -> Option<T> {
            if self.top == 0 {
                return None;
            }
            self.top -= 1;
            self.data.pop()
        }

        // 数 据 不 能 移 动 ， 只 能 返 回 引 用
        fn peek(&self) -> Option<&T> {
            if self.top == 0 {
                return None;
            }
            self.data.get(self.top-1)
        }
        fn size(&self) -> usize {
            self.top
        }

        fn is_empty(&self) -> bool {
            self.top == 0
        }
    }

    #[test]
    fn test() {
        let mut s = Stack::new();
        s.push(1).push(2).push(3);

        println!("s size:  {}", s.size());

        println!("pop {:?}, size {}",s.pop().unwrap(), s.size());

        println!("is_empty:{}, stack:{:?}", s.is_empty(), s);
    }

    /*
    考虑下面正确匹配的括号字符串:
    (()()()())
    (((())))
    (()((())()))

    以及这些不匹配的括号:
    ((((((())
    ()))
    (()()(()


    ( ( ) ( ( ) ) ( ) )
    */

    fn _par_checker1(par: &str) -> bool {
        let mut char_list = Vec::new();

        for c in par.chars() {
            char_list.push(c);
        }

        let mut index = 0;
        let mut balance = true; // 括 号 是 否 匹 配 (平衡)标示
        let mut stack = Stack::new();

        while index < char_list.len() && balance {
            let c = char_list[index];

            // 如 果 为 开 符 号 ， 入栈
            if '(' == c {
                stack.push(c);
            } else {
                // 闭符号
                if stack.is_empty() {
                    // 为 空 则 不 平 衡
                    balance = false
                } else {
                    let _r = stack.pop();
                }
            }

            index += 1
        }

        // 平 衡 且 栈 为 空 ， 括 号 表 达 式 才 是 匹 配 的
        balance && stack.is_empty()
    }

    /*
    (){}[]
    */
    fn par_checker2(par: &str) -> bool {
        let mut char_list = Vec::new();

        for c in par.chars() {
            char_list.push(c);
        }

        let mut index = 0;
        let mut balance = true; // 括 号 是 否 匹 配 (平衡)标示
        let mut stack = Stack::new();

        while index < char_list.len() && balance {
            let c = char_list[index];

            // 如 果 为 开 符 号 ， 入栈
            // 同 时 判 断 三 种 开 符 号
            if '(' == c  || '[' == c || '{' == c{
                stack.push(c);
            } else {
                // 闭符号
                if stack.is_empty() {
                    // 为 空 则 不 平 衡
                    balance = false
                } else {
                    // 比 较 当 前 括 号 和 栈 顶 括 号 是 否 匹 配
                    let top = stack.pop().unwrap();
                    if !par_match(top, c) {
                        balance = false;
                    }
                }
            }

            index += 1
        }

        // 平 衡 且 栈 为 空 ， 括 号 表 达 式 才 是 匹 配 的
        balance && stack.is_empty()
    }

    /*
    (a+b)(c*d)func()
    */
    fn par_checker3(par: &str) -> bool {
        let mut char_list = Vec::new();

        for c in par.chars() {
            char_list.push(c);
        }

        let mut index = 0;
        let mut balance = true; // 括 号 是 否 匹 配 (平衡)标示
        let mut stack = Stack::new();

        while index < char_list.len() && balance {
            let c = char_list[index];

            // 如 果 为 开 符 号 ， 入栈
            // 同 时 判 断 三 种 开 符 号
            if '(' == c  || '[' == c || '{' == c{
                stack.push(c);
            }

            if ')' == c || ']' == c || '}' == c{
                // 闭符号
                if stack.is_empty() {
                    // 为 空 则 不 平 衡
                    balance = false
                } else {
                    // 比 较 当 前 括 号 和 栈 顶 括 号 是 否 匹 配
                    let top = stack.pop().unwrap();
                    if !par_match(top, c) {
                        balance = false;
                    }
                }
            }
            // 非 括 号 直 接 跳 过
            index += 1
        }

        // 平 衡 且 栈 为 空 ， 括 号 表 达 式 才 是 匹 配 的
        balance && stack.is_empty()
    }

    fn par_match(open: char, close: char) -> bool {
        let opens = "([{";
        let closers = ")]}";
        opens.find(open) == closers.find(close)
    }

    #[test]
    fn test2() {
        let sa = "()()";
        let res1 = _par_checker1(sa);
        println!("sa balanced:{res1}");

        let sb = "()((()";
        let res2 = _par_checker1(sb);
        println!("sb balanced:{res2}");

    }

    #[test]
    fn test3() {
        let sa ="(){}[]";
        let res1 = par_checker2(sa);
        println!("sa balanced:{res1}");

        let sb = "(){)[}";
        let res2 = par_checker2(sb);
        println!("sb balanced:{res2}");

    }

    #[test]
    fn test4() {
        let sa = "(2+3){func}[abc]";
        let res1 = par_checker3(sa);
        println!("sa balanced:{res1}");

        let sb = "(2+3)*(3-1";
        let res2 = par_checker3(sb);
        println!("sb balanced:{res2}");

    }
}