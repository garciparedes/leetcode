impl Solution {
    pub fn sort_array_by_parity_ii(mut a: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut i = 0;
        while i < n {
            if i % 2 == a[i] as usize % 2 {
                i += 1;
            } else {
                let item = a.remove(i);
                a.push(item);
            }
        }
        return a;
    }
}
