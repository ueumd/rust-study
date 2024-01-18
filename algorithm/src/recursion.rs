#[cfg(test)]

#[allow(dead_code)]
mod recursion {
    /*
    递归三定律
    1 递 归 算 法 必 须 具 有 基 本 情 况
    2 递 归 算 法 必 须 向 基 本 情 况 靠 近
    3 递 归 算 法 必 须 以 递 归 方 式 调 用 自 身
    */

    /*
    问题： 举个简单的例子。假设你想计算整数 [2,1,7,4,5] 的总和，最直观的就是用一个加法累加器，逐个将值与之相加，具体代码如下。
    */
    fn nums_sum(nums: &[i32]) -> i32 {
        let mut sum = 0;
        for num in nums {
            sum += num
        }
        sum
    }

    fn nums_sum1(nums: &[i32]) -> i32 {
        if 1 == nums.len() {
            nums[0]
        } else {
            let first = nums[0];
            first + nums_sum1(&nums[1..])
        }
    }

    #[test]
    fn test() {
        let nums = [2, 1, 7, 4, 5];
        println!("nums_sum: {}", nums_sum(&nums));
        println!("nums_sum1: {}", nums_sum1(&nums));
    }


    // 斐波那契数列问题
    fn fibnacci(n: u32) -> u32 {
        if n <= 2 {
            return 1;
        } else {
            fibnacci(n - 1) + fibnacci(n - 2)
        }
    }

    /*
   动态规划的三个特征
   1. 适用于最优解问题
   2. 有大量的重复子问题
   3. 子问题之间有依赖（不独立）

   动态规划实现
   ● 自顶向下 的备忘录方式，用递归实现
   ● 自底向上 的方式，用迭代实现

   与分治 / 递归关系
   分治法中，有大量的重复子问题，且它们之间 无依赖


   与递归的关系
   这些重复的子问题，DP算法将其结果用一维或二维数组（邻接矩阵）保存下来，等下一次又要计算该子问题时，直接用已计算好的；
   而递归却不是这样，它会一遍又一遍地计算这些重复的子问题，从而效率狂降。
   子问题重复率较高的递归算法可改写成动态规划，但不是所有递归算法都适合改成动态规划。
   */

    fn fibnacci_dp(n: u32) -> u32 {
        let mut dp = [1, 1];

        for i in 2..=n {
            let idx1 = (i % 2) as usize;
            let idx2 = ((i - 1) % 2) as usize;
            let idx3 = ((i - 2) % 2) as usize;
            println!("{i} - {idx1} - {idx2} - {idx3}");

            // 数组大小是固定的，改变的始终是 0 和 1下标
            dp[idx1] = dp[idx2] + dp[idx3];
        }

        dp[((n - 1) % 2) as usize]

        /*
        2 - 0 - 1 - 0
        3 - 1 - 0 - 1
        4 - 0 - 1 - 0
        5 - 1 - 0 - 1
        6 - 0 - 1 - 0
        7 - 1 - 0 - 1
        8 - 0 - 1 - 0
        9 - 1 - 0 - 1
        10 - 0 - 1 - 0
        */
    }

    fn fibnacci_dp2(n: u32) -> u32 {
        let mut dp = vec![];
        // 错误用法，不可以下标添加
        // dp[0] = 0;
        // dp[1] = 1;
        // dp[2] = 1;

        //push()方法
        dp.push(0);
        dp.push(1);
        dp.push(1);


        for i in 3..=n {
            let idx1 = i as usize;
            let idx2 = (i-1) as usize;
            let idx3 = (i - 2) as usize;
            println!("{i} - {idx1} - {idx2} - {idx3}");
            dp.insert(idx1,  dp[idx2] + dp[idx3])
        }

        println!("dp {:?}", dp); //dp [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]

        dp[n as usize]
    }

    /*
    错误版本
    这个版本想改成JS一样的，后来发现rust中数组大小是固定的

    数组（array）与元组不同，数组中的每个元素的类型必须相同。数组的特点是：
        数组大小固定
        元素均为相同类型
        默认不可变

        可以通过 let mut 关键字定义可变绑定的 mut_arr，但是也只能通过下标修改数组元素的值。
    */
    fn fibnacci_dp3(n: u32) -> u32 {
        // 声明后就是固定了
        let mut dp = [0,1,1];
        for i in 3..=n {
            let idx1 = i as usize;
            let idx2 = (i-1) as usize;
            let idx3 = (i - 2) as usize;
            println!("{i} - {idx1} - {idx2} - {idx3}");

            // 所以这里会报错的
            dp[idx1] = dp[idx2] + dp[idx3];
        }

        println!("dp {:?}", dp);

        dp[n as usize]
    }

    #[test]
    fn test2() {
        println!("fib 10: {}", fibnacci(10));
        println!("fib 10: {}", fibnacci_dp(10));
        println!("fib 10: {}", fibnacci_dp2(10));
    }
}