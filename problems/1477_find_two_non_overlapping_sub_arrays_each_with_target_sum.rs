use std::cmp::Ordering;


impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut candidates = Vec::new();
        let mut i = 0;
        let mut cumulate = 0;
        for (j, &elem) in arr.iter().enumerate() {
            cumulate += elem;
            match cumulate.cmp(&target) {
                Ordering::Less => {
                    continue;
                },
                Ordering::Equal => {
                    candidates.push((i, j));
                    cumulate -= arr[i];
                    i += 1;
                },
                Ordering::Greater => {
                    while cumulate > target && i < j {
                        cumulate -= arr[i];
                        i += 1;
                    }
                    if cumulate == target {
                        candidates.push((i, j));
                        cumulate -= arr[i];
                        i += 1;
                    }
                }
            }            
        }
        
        if candidates.len() < 2 {
            return -1;
        }
        
        candidates.sort();        
        let mut index = 0;
        for (i, &elem) in candidates.iter().enumerate() {
            if candidates[index].1 - candidates[index].0  > elem.1 - elem.0 {
                index = i;
            }
        }
        let mut first = candidates[index];
                
        let iterable = candidates
            .iter()
            .filter(|&elem| !((first.0 <= elem.0) && (elem.0 <= first.1) || (first.0 <= elem.1) && (elem.1 <= first.1)))
            .enumerate();
        
        let mut tmp = 0;
        for (i, &elem) in iterable {
            if tmp == 0 || tmp  > elem.1 - elem.0 {
                tmp =  elem.1 - elem.0 + 1;
            }
        }
        if tmp == 0 {
            return -1
        }            
        
        return (first.1 - first.0 + 1 + tmp) as i32;
    }
}
