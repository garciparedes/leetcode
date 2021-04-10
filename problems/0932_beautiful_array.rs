impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans: Vec<usize> = Vec::new();
        ans.push(1);
        
        while ans.len() < n {
            let mut tmp = Vec::new();
            for i in &ans {
                if i * 2 - 1 <= n {
                    tmp.push(i * 2 - 1);
                }
            }
            for i in &ans {
                if i * 2 <= n {
                    tmp.push(i * 2);
                }
            }
            ans = tmp;
        }
        
        return ans
            .into_iter()
            .map(|v| v as i32)
            .collect();
     }
}
