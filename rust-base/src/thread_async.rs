#[cfg(test)]
mod example {

    // `block_on`会阻塞当前线程直到指定的`Future`执行完成，这种阻塞当前线程以等待任务完成的方式较为简单、粗暴，
    // 好在其它运行时的执行器(executor)会提供更加复杂的行为，例如将多个`future`调度到同一个线程上执行。

    use futures::executor::block_on;
    async fn do_something() {
        println!("go go go !");
    }

    async fn hello_world() {
        // 使用.await
        hello_cat().await;
        println!("hello, world!");
    }

    async fn hello_cat() {
        println!("hello, kitty!");
    }

    #[test]
    fn test() {
        let future = do_something();

        // `block_on`会阻塞当前线程直到指定的`Future`执行完成
        block_on(future);

        println!("test test test");

        let future2 = hello_world();
        block_on(future2);
    }

}