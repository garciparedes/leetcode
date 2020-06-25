use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut memory: HashSet<i32> = HashSet::new();
        
        let mut current = n;
        loop {
            let mut total: i32 = 0;
            while current != 0 {
                let digit = current % 10;
                current /= 10;
                total += digit.pow(2);
            }
            if total == 1 {
                return true;
            }
            current = total;
            if memory.contains(&current) {
                return false;
            }
            memory.insert(current);
        }
    }
}
