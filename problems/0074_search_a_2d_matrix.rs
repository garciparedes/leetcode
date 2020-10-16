use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        }
        let mut i = (0, matrix.len());
        let mut j = (0, matrix[0].len());
        
        while (i.1 - i.0) > 1 {
            let mid = (i.0 + i.1) / 2;
            i = match target.cmp(&matrix[mid][0]) {
                Ordering::Less => (i.0, mid),
                Ordering::Equal => (mid, mid),
                Ordering::Greater => (mid, i.1),
            };
        }
        
        while (j.1 - j.0) > 1 {
            let mid = (j.0 + j.1) / 2;
            j = match target.cmp(&matrix[i.0][mid]) {
                Ordering::Less => (j.0, mid),
                Ordering::Equal => (mid, mid),
                Ordering::Greater => (mid, j.1),
            };
        }
        
        return matrix[i.0][j.0] == target;
        
    }
}
