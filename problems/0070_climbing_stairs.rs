use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memory = HashMap::new();
        return Self::rec(n, &mut memory);
    }
    
    fn rec(n: i32, memory: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 {
            return 1;
        }
        
        if let Some(count) = memory.get(&n) {
            return *count;
        }
        
        let mut count = 0;
        if n >= 1 {
            count += Self::rec(n - 1, memory);
        }
        if n >= 2 {
            count += Self::rec(n - 2, memory);
        }
        
        memory.insert(n, count);
        return count;   
    }
}
