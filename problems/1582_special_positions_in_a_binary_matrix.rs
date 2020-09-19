impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (mat.len(), mat[0].len());
        let mut rows = vec![0; mat.len()];
        let mut columns = vec![0; mat[0].len()];
        let mut pairs = Vec::new();
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] != 1 {
                    continue;
                }
                rows[i] += 1;
                columns[j] += 1;
                pairs.push((i, j));
            }
        }
        let mut ans = 0;
        for (i, j) in pairs {
            if (rows[i] != 1) || (columns[j] != 1) {
                continue;
            }
            ans += 1;
        }
        return ans;
    }
}
