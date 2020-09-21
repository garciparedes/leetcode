use std::cmp;

impl Solution {
    pub fn car_pooling(mut trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        trips.sort_unstable_by_key(|k| (k[1], k[2]));
        
        let mut unloads: Vec<Vec<i32>> = Vec::new();
        let mut current_capacity = 0;
        let mut current_position = 0;
        for trip in trips {
            while let Some(unload) = unloads.get(unloads.len() - 1) {
                if !(unload[2] <= trip[1]) {
                    break;
                }
                current_capacity -= unload[0];
                unloads.pop();
            }
            if !(current_position <= trip[1]) && !(current_capacity + trip[0] <= capacity) {
                return false;
            }
            current_capacity += trip[0];
            current_position = cmp::max(current_position, trip[2]);
            
            let mut i = 0;
            while i < unloads.len() && unloads[i][2] >= trip[2] {
                i += 1;
            }
            
            unloads.insert(i, trip);
        }
        
        return true;
    }
}
