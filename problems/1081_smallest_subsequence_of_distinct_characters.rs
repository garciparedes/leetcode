impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut stack = Vec::new();
        let mut last = vec![0; 26];
        let mut seen = vec![false; 26];
        
        for (i, c) in s.chars().enumerate() {
            last[(c as u8 - 'a' as u8) as usize] = i;
        }
        for (i, c) in s.chars().enumerate() {
            let c = (c as u8 - 'a' as u8) as usize;
            
            if seen[c] {
                continue;
            }
            seen[c] = true;
            
            while let Some(&peek) = stack.last() {
                if peek <= c || i >= last[peek] {
                    break;
                }
                seen[peek] = false;
                stack.pop();
            }
            stack.push(c);
        }
        let mut ans = String::new();
        for c in stack {
            ans.push((c as u8 + 'a' as u8) as char);
        }
        return ans;
    }
}
