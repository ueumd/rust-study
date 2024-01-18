#[cfg(test)]
mod algorithm {
    fn sum_of_n(n: i64) -> i64 {
        let mut sum = 0;
        for i in 1..=n {
            sum += i;
        }
        sum
    }

    #[test]
    fn test() {
        for _i in 0..5 {
            let now = std::time::SystemTime::now();
            let _sum = sum_of_n(1000000);
            let duration = now.elapsed().unwrap();

            let time = duration.as_millis();
            println!("func used {time} ms");
        }
    }

    #[test]
    fn test2() {

    }
}