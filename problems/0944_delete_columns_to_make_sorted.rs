impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        let (n, m) = (a.len(), a[0].len());
        
        let mut count = 0;
        for j in 0..m {
            let mut current = a[0].chars().nth(j).unwrap();
            for i in 1..n {
                if current > a[i].chars().nth(j).unwrap() {
                    count += 1;
                    break;
                }
                current = a[i].chars().nth(j).unwrap();
            }
        }
        return count;
    }
}
