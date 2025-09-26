impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut v: Vec<_> = s.chars().collect();
        let n = v.len();
        for i in 0..n / 2 {
            let j = n - (i + 1);
            if v[i] > v[j] {
                v[i] = v[j]
            } else {
                v[j] = v[i]
            }
        }
        v.into_iter().collect()
    }
}
