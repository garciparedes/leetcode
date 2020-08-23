impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        
        let mut result = 0;
        let mut i = 0;
        let mut j = piles.len();
        while i < j {
            i += 1;
            j -= 2;
            result += piles[j];
        }
        
        return result;
    }
}
