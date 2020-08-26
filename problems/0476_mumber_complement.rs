impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let binary = format!("{:b}", num);
        let complement = binary.chars().map(|x| if x == '0' { '1' }  else { '0' }).collect::<String>();
        return i32::from_str_radix(complement.as_str(), 2).unwrap_or(0);
    }
}
