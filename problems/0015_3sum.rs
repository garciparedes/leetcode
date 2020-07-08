use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        nums.sort();
        let mut result = HashSet::new();
        
        for i in 0..n {
            if nums[i] > 0 {
                break;
            }
            let target = nums[i];
            for j in i+1..n {
                let first = nums[j];
                for k in j + 1..n {
                    let second = nums[k];
                    if target + first + second > 0 {
                        break;
                    }
                    if target + first + second == 0 {
                        let mut tmp = vec![target, first, second];
                        tmp.sort();
                        result.insert(tmp);
                        break;
                    }
                }
            }
        }
        return result.into_iter().collect();
    }
}
