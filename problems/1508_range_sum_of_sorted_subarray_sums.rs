impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let left = left as usize;
        let right = right as usize;
        
        let mut sums = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in i+1..n + 1 {
                let val: i32 = nums[i..j].iter().sum();
                sums.push(val);
            }
        }
        sums.sort();
        let mut count: u128 = 0;
        for i in left - 1..right {
            count += sums[i] as u128;
        }
        count = count % (u128::pow(10, 9) + 7);
        return count as i32;
    }
}
