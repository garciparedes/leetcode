impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();
        
        let mut ans = 0;
        for c in costs {
            if c > coins {
                break;    
            } 
            coins -= c;
            ans += 1;
        }
        return ans;
    
    }
}
