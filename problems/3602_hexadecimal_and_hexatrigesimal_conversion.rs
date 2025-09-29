impl Solution {
    pub fn concat_hex36(n: i32) -> String {
       format!(
        "{}{}", 
        Solution::formatHexadecimal(n * n), 
        Solution::formatHexatrigesimal(n * n * n)
       ) 
    }
    
    fn formatHexadecimal(v: i32) -> String {
        format!("{:X}", v)
    }

    fn formatHexatrigesimal(mut v: i32) -> String {
        let mut digits = Vec::new();
        while v != 0 {
            digits.push((v % 36));
            v /= 36;
        }
        digits.into_iter()
            .rev()
            .map(|d| char::from_digit(d as u32, 36).unwrap().to_uppercase().to_string())
            .collect()
    }
}
