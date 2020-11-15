use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let mut visited = vec![false; 20001]; 
        for i in forbidden {
            visited[i as usize] = true;
        } 
        
        let mut memory = HashMap::new();
        
        let ans = Self::dfs(a as usize, b as usize, x as usize, 0, false, &mut visited, &mut memory);
        if ans > 10_000 {
            return -1;
        }
        return ans;
    }
    
    fn dfs(
        a: usize, 
        b: usize, 
        x: usize, 
        i: usize, 
        backwarded: bool, 
        visited: &mut Vec<bool>,
        memory: &mut HashMap<(usize, bool), i32>,
    ) -> i32 {
        if i == x {
            return 0;
        }        
        if let Some(&ans) = memory.get(&(i, backwarded)) {
            return ans;
        }
        
        if i >= 20001 || visited[i] {
            return 10_000;
        }
        visited[i] = true;
        
        let mut ans = Self::dfs(a, b, x, i + a, false, visited, memory);
        if !backwarded && i > b {
            ans = cmp::min(ans, Self::dfs(a, b, x, i - b, true, visited, memory))
        }
        ans += 1;
        
        visited[i] = false;

        memory.insert((i, backwarded), ans);
  
        return ans;
    }
}
