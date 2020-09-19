impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 1..9 {
            Self::backtrack(low, high, i, &mut ans);
        }
        ans.sort_unstable();
        return ans;
    }
    
    fn backtrack(low: i32, high: i32, current: i32, ans: &mut Vec<i32>) {
        if (low <= current) && (current <= high) {
            ans.push(current);
        }
        if current > high {
            return;
        }
        let last = current % 10;
        if last == 9 {
            return;
        }
        Self::backtrack(low, high, current * 10 + last + 1, ans);
    }
}
