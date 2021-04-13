static MAX_POW_2: i32 = 536870912;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let digits = Self::generate_sorted_digits(n);

        let mut power = 1;
        while power <= MAX_POW_2 {
            if Self::generate_sorted_digits(power) == digits {
                return true;
            }
            power *= 2;
        }
        return false;
    }
    
    fn generate_sorted_digits(mut n: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        while n != 0 {
            ans.push(n % 10);
            n /= 10;
        }
        ans.sort_unstable();
        return ans;
    }
}
