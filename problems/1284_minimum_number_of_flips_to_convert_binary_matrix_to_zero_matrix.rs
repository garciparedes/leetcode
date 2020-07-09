use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let mut memory: HashMap<Vec<Vec<i32>>, i32> = HashMap::new();
        let (n, m) = (mat.len(), mat[0].len());
        
        
        let mut count = i32::max_value();
        for i in 0..n {
            for j in 0..m {
                let mut current = Self::flip(mat.clone(), (i, j), &mut memory);
                count = cmp::min(count, current);  
            }
        }
        if count == i32::max_value() {
            return -1;
        }
        return count;
    }
    
    
    fn flip(mut mat: Vec<Vec<i32>>, position: (usize, usize), memory: &mut HashMap<Vec<Vec<i32>>, i32>) -> i32 {
        if Self::valid(&mat) {
            return 0;
        }
        
        let (n, m) = (mat.len(), mat[0].len());
        
        mat[position.0][position.1] = 1 - mat[position.0][position.1];
        if position.0 > 0 {
            mat[position.0 - 1][position.1] = 1 - mat[position.0 - 1][position.1];
        }
        if position.0  < n - 1 {
            mat[position.0 + 1][position.1] = 1 - mat[position.0 + 1][position.1];
        }
        if position.1 > 0 {
            mat[position.0][position.1 - 1] = 1 - mat[position.0][position.1 - 1];
        }
        if position.1 < m - 1 {
            mat[position.0][position.1 + 1] = 1 - mat[position.0][position.1 + 1];
        }

        let mut count = i32::max_value();
        if memory.contains_key(&mat) {
            return cmp::min(count, *memory.get(&mat).unwrap());
        } 
        memory.insert(mat.clone(), count);
        for i in 0..n {
            for j in 0..m {
                let mut current = Self::flip(mat.clone(), (i, j), memory);
                if current == i32::max_value() {
                    continue;
                } 
                count = cmp::min(count, current + 1);  
                count = cmp::min(count, *memory.get(&mat).unwrap());
            }
        }
        memory.insert(mat.clone(), count);
        return count;
    }
    
    fn valid(mat: &Vec<Vec<i32>>) -> bool {
        let (n, m) = (mat.len(), mat[0].len());
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 1 {
                    return false;
                }
            }
        }
        return true;
    }
}

