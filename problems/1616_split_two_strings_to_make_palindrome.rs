use std::collections::VecDeque;

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let mut a: Vec<_> = a.chars().collect();
        let mut b: Vec<_> = b.chars().collect();
        let n = a.len();
        let (mut la, mut lb) = (0, 0);
        for i in (0..n) {
            if Self::is_palindrome(&a, &mut la) || Self::is_palindrome(&b, &mut lb) {
                return true;
            }
            let tmp = a[i];
            a[i] = b[i];
            b[i] = tmp;
        }    
        return Self::is_palindrome(&a, &mut la) || Self::is_palindrome(&b, &mut lb);
    }
    
    fn is_palindrome(a: &Vec<char>, j: &mut usize) -> bool {
        let n = a.len();
        for i in *j..(n / 2) {
            if a[i] != a[n - i - 1] {
                return false;
            } else {
                *j = i;
            }
        } 
        return true;
    }
    
}
