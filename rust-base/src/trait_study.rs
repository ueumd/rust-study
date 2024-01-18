#[cfg(test)]

#[allow(dead_code)]
mod vec_study {

    // trait 关键字来声明一个特征，Summary 是特征名
// 类似Java接口
    pub trait Summary {
        fn summarize(&self) -> String;

        // 也可以默认实现方法体
        // fn summarize(&self) -> String {
        //     String::from("(Read more...)")
        // }
    }

    struct Post  {
        title: String,
        author: String,
        content: String
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是: {}, 内容： {}", self.title, self.author, self.content)
        }
    }

    struct Weibo {
        username: String,
        content: String
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }


    // 实现了Summary特征 的 item 参数
    fn _notify(item: &impl Summary) {
        println!("Breaking news !{}", item.summarize());
    }

    //  T: Summary 被称为特征约束
    fn _notify2<T:Summary>(item :&T) {
        println!("Breaking news !{}", item.summarize());
    }
    #[test]
    fn test() {
        let post = Post{
            title:"Rust语言".to_string(),
            author: "Sunface".to_string(),
            content: "Rust棒极了".to_string()
        };
        let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

        println!("{}",post.summarize());
        println!("{}",weibo.summarize());
    }

    #[test]
    fn test2() {

    }
}