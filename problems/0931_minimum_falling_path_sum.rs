use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn min_falling_path_sum(a: Vec<Vec<i32>>) -> i32 {
        let mut memory = HashMap::new();
        
        return (0..a[0].len())
            .map(|i| Self::dfs(&a, 0, i as isize, &mut memory))
            .min()
            .unwrap()
    }
    
    fn dfs(a: &Vec<Vec<i32>>, depth: usize, index: isize, memory: &mut HashMap<(usize, isize), i32>) -> i32 {
        if index < 0 || index as usize >= a[depth].len() {
            return i32::max_value();
        }
        if let Some(ans) = memory.get(&(depth, index)) {
            return *ans;
        }
        
        if depth >= a.len() - 1 {
            return a[depth][index as usize];
        }

        let ans = a[depth][index as usize] + cmp::min(
            Self::dfs(a, depth + 1, index - 1, memory),
            cmp::min(
                Self::dfs(a, depth + 1, index, memory),
                Self::dfs(a, depth + 1, index + 1, memory),
            )
        );  
        memory.insert((depth, index), ans);
        return ans;
    }
}
