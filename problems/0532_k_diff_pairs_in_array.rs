use std::collections::HashMap;

impl Solution {
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = HashMap::new();
        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }
        
        let mut ans = 0;
        for key in counter.keys() {
            if (k == 0 && counter[key] > 1) || (k != 0 && counter.contains_key(&(key + k)))  {
                ans += 1;
            }
        }
        return ans;
    }
}
