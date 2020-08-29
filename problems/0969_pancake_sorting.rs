impl Solution {
    pub fn pancake_sort(mut a: Vec<i32>) -> Vec<i32> {
        let mut ops = Vec::new();
        for i in (0..a.len()).rev() {
            let mut j = i;
            for k in 0..i {
                if a[k] > a[j] {
                    j = k;
                }
            }
            if j == i {
                continue;
            }
            if j > 0 {
                Self::swap(&mut a, j);
                ops.push(j as i32 + 1);            
            }
            Self::swap(&mut a, i);
            ops.push(i as i32 + 1);            
        }
        return ops;
    }
    
    fn swap(a: &mut Vec<i32>, k: usize) {
        for i in 0..(k + 1) / 2 {
            a.swap(i, k - i);
        }
    }
}
