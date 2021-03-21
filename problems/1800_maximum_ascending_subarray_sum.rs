impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let (mut ans, mut prev, mut cum) = (0, 0, 0);
        for num in nums {
            if prev < num {
                cum += num;
                if ans < cum {
                    ans = cum;
                }
            } else {
                cum = num;
            }
            prev = num;
        }
        return ans;
    }
}
