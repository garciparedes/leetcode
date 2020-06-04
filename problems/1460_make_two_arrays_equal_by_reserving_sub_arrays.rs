use std::collections::HashMap;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut target_counter: HashMap<i32, i32> = HashMap::new();
        let mut arr_counter: HashMap<i32, i32> = HashMap::new();
        
        for (&a, &b) in target.iter().zip(arr.iter()) {
            *target_counter.entry(a).or_insert(0) += 1;
            *arr_counter.entry(b).or_insert(0) += 1;
        }
        
        if target_counter.len() != arr_counter.len() {
            return false;
        }
        
        for (key, &expected) in target_counter.iter() {
            match arr_counter.get(key) {
                Some(&observed) => {
                    if observed != expected {
                        return false;
                    }
                },
                None => return false
            }
        }
        return true;
    }
}
