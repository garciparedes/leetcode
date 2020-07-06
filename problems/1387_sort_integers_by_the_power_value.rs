impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut values: Vec<_> = (lo..hi + 1).map(|x| (Self::power(x), x)).collect();
        values.sort();
        return values[k as usize - 1].1;
    }
    
    fn power(mut x: i32) -> i32 {
        let mut count = 0;
        while x != 1 {
            if x % 2 == 0 {
                x = x / 2;
            } else {
                x = 3 * x + 1;
            }
            count += 1;
        }
        return count;
    }
}
