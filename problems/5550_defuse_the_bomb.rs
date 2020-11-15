impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        if k == 0 {
            return vec![0; n];
        }
        let mut sign: isize = if k > 0 { 1 } else { - 1};
        let k = k.abs() as usize;
        
        let mut ans = Vec::new();
        for i in 0..n {
            let mut tmp = 0;
            let mut j = i;
            for _ in 0..k {
                j = (((j as isize + sign) % n as isize + n as isize) % n as isize) as usize;
                tmp += code[j];
            }
            ans.push(tmp);
        }
        return ans;
    }
}
