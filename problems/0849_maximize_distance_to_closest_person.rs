use std::cmp;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        
        let mut i = 0;
        while seats[i] != 1 {
            i += 1;
        }
        
        let mut j = seats.len() - 1;
        while seats[j] != 1 {
            j -= 1;
        }
        
        let mut ans = cmp::max(i, n - (j + 1)) as i32;
        let mut curr = 0; 
        for k in i..(j + 1) {
            curr += 1;
            
            if seats[k] != 1 {
                continue;
            }
            
            if ans < (curr - 1) / 2  {
                ans = (curr - 1) / 2;
            }
            
            curr = 1;
            
        }
        
        return ans;
    }
}
