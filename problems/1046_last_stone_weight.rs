impl Solution {
    pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
        stones.sort();
        
        while stones.len() > 1 {
            if let (Some(first), Some(second)) = (stones.pop(), stones.pop()) {
                let new = first - second;
                if new == 0 {
                    continue;
                }
                let mut i = 0;
                while i < stones.len() && stones[i] < new {
                    i += 1;
                }
                stones.insert(i, new);
            } else {
                panic!();
            }
        }
        
        return *stones.get(0).unwrap_or(&0);
    }
}
