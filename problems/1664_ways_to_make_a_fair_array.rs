impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let (mut even_cum, mut odd_cum) = (0, 0);
        let (mut evens, mut odds) = (Vec::new(), Vec::new());
        for i in 0..nums.len() {
            if i % 2 == 0 {
                even_cum += nums[i];
            } else {
                odd_cum += nums[i];
            }
            evens.push(even_cum);
            odds.push(odd_cum);
        }
        
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut odd = evens[evens.len() - 1] - evens[i];
            let mut even = odds[odds.len() - 1] - odds[i];
            if i > 0 {
                odd += odds[i - 1];
                even += evens[i - 1];
            }
            if odd == even {
                ans += 1;
            }
            
            
        }
        return ans;
    }
}
