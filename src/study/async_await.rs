use futures::executor::block_on;
use std::num::ParseIntError;
struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "周杰伦".to_string(),
        name: String::from("《菊花台》"),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "菊花残，满地伤~ ~"
    );
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn learn_and_sing() {
    let song = learn_song().await;

    /// 唱歌必须要在学歌之后
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。
    // 若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f1, f2);
}

pub fn init() {
    // 一次只能做一件事，实际上我们完全可以载歌载舞啊:
    // let song = block_on(learn_song());
    // block_on(sing_song(song));
    // block_on(dance());

    block_on(async_main());

    fn try_to_parse() -> Result<i32, ParseIntError> {
        let x: i32 = "123".parse()?; // x = 123
        let y: i32 = "24a".parse()?; // returns an Err() immediately
        Ok(x + y) // Doesn't run.
    }

    let res = try_to_parse();
    println!("res: {:?}", res);
}
