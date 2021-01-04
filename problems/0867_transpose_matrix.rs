impl Solution {
    pub fn transpose(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut b = vec![vec![0; a.len()]; a[0].len()];
        for i in 0..a.len() {
            for j in 0..a[0].len() {
                b[j][i] = a[i][j];
            }
        }
        return b;
    }
}
