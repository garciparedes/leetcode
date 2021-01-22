use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn min_falling_path_sum(arr: Vec<Vec<i32>>) -> i32 {
        let mut memo = HashMap::new();
        return Self::helper(&arr, 0, arr.len(), &mut memo);
    }
    
    fn helper(arr: &Vec<Vec<i32>>, depth: usize, last: usize,  memo: &mut HashMap<(usize, usize), i32>) -> i32 {
        if depth == arr.len() {
            return 0;
        }
        
        if let Some(&ans) = memo.get(&(depth, last)) {
            return ans;
        }
        
        let first_i = (0..arr.len())
            .filter(|i| *i != last)
            .min_by_key(|i| arr[depth][*i])
            .unwrap();
        let mut ans = arr[depth][first_i] + Self::helper(arr, depth + 1, first_i, memo);
        
        if arr.len() > 1 {    
            let second_i = (0..arr.len())
                .filter(|i| *i != last && *i != first_i)
                .min_by_key(|i| arr[depth][*i])
                .unwrap();
            let second_ans = arr[depth][second_i] + Self::helper(arr, depth + 1, second_i, memo);
            ans = cmp::min(ans, second_ans);
        }
        
        memo.insert((depth, last), ans);
        return ans;
    }
}
