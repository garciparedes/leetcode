use std::cmp;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let n = cmp::max(a.len(), b.len()) + 1;
        
        let mut first = vec![0; n - a.len()];
        first.extend(a.chars().map(|x| x.to_digit(10).unwrap() as u8));
        
        let mut second = vec![0; n - b.len()];
        second.extend(b.chars().map(|x| x.to_digit(10).unwrap() as u8));
        
        let mut result = vec![0; n];
        let mut reminder = 0;
        for i in (0..n).rev() {
            let tmp = first[i] + second[i] + reminder;
            result[i] = tmp % 2;
            reminder = (tmp > 1) as u8;
        }
        return result[((result[0] == 0) as usize)..]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
    }
}
