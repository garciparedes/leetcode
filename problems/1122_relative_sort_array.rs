use std::collections::HashMap;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mapper:HashMap<i32, usize> = arr2
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        
        arr1.sort_by_key(|&v| {
            (mapper.get(&v).unwrap_or(&usize::max_value()), v)
        });
        
        return arr1;
    }
}
