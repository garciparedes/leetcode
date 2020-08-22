impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let (mut i, mut n) = (0, a.len());
        while i < n {
            if a[i] % 2 == 0 {
                i += 1;
                continue;
            } else {
                a.swap(i, n - 1);
                n -= 1;   
            }
        }     
        return a;
    }
}
