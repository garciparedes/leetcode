impl Solution {
    pub fn pancake_sort(mut a: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut indexes = Vec::new();
        for i in (1..n).rev() {
            if Self::is_sorted(&a) {
                break;
            }
            
            let j = a[..i + 1]
                .iter()
                .enumerate()
                .max_by_key(|(_, &item)| item)
                .unwrap()
                .0;
            
            if j == i {
                continue;
            }
            
            if j > 0 {
                Self::flip_head(&mut a, j);    
                indexes.push((j + 1) as i32);
            }
            
            Self::flip_head(&mut a, i);
            indexes.push((i + 1) as i32);
        }

        return indexes;
    }
    
    fn flip_head(a: &mut Vec<i32>, i: usize) {
        for j in 0..(i + 1) / 2 {
            a.swap(j, i - j);
        }
    }
    
    fn is_sorted(a: &Vec<i32>) -> bool {
        for i in 0..a.len() - 1 {
            if a[i + 1] < a[i] {
                return false;
            }
        }
        return true;
    }
}
