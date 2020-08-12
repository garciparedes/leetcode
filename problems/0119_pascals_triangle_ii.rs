use std::mem;


impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut storage = vec![1; row_index + 1]; 
        let mut new = vec![1; row_index + 1];
        for i in 1..row_index {
            for j in 1..(i  + 1) {
                new[j] = storage[j - 1] + storage[j];
            }
            mem::swap(&mut storage, &mut new);
        }
        return storage.into_iter().collect();
    }
}
