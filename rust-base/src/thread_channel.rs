#[cfg(test)]
mod thread_channel {
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let s = String::from("我，飞走咯!");
            tx.send(s).unwrap();
            // println!("val is {}", s);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received); // Got: 我，飞走咯!
    }

    #[test]
    fn test2() {

    }
}