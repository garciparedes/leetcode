use std::cmp;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {        
        let (mut odd, mut even) = (0, 0);
        for index in position {
            if index % 2 == 0 {
                odd += 1;
            } else {
                even += 1;
            }
        }
        return cmp::min(odd, even);
    }
}
