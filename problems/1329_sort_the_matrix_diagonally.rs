impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (n, m) = (mat.len(), mat[0].len());
        
        let mut sorted: Vec<Vec<i32>> = vec![vec![0; m]; n];
        let mut k = 0;
        for j in 0..m {
            let mut tmp = Vec::new();
            for i in 0..n {
                if !(j + i < m) {
                    break;
                }
                tmp.push(mat[i][j + i])
            }
            tmp.sort();
            for i in 0..n {
                if !(j + i < m) {
                    break;
                }
                sorted[i][j + i] = tmp.remove(0);
            }
        }
        for i in 1..n {
            let mut tmp = Vec::new();
            for j in 0..m {
                if !(i + j < n) {
                    break;
                }
                tmp.push(mat[i + j][j])
            }
            tmp.sort();
            for j in 0..m {
                if !(i + j < n) {
                    break;
                }
                sorted[i + j][j] = tmp.remove(0);
            }
        }
        return sorted;
    }
}
