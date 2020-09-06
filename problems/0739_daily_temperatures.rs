impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; t.len()];
        let mut stack = Vec::new();
        for (i, &case) in t.iter().enumerate() {
            while let Some(&j) = stack.last() {
                if t[j] >= case {
                    break;
                }
                ans[j] = (i - j) as i32;                
                stack.pop();
            }
            stack.push(i);
        }
        return ans;
    }
}
