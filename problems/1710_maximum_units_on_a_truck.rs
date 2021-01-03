use std::cmp;

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_by_key(|entry| cmp::Reverse(entry[1]));
        
        let mut ans = 0;
        for entry in box_types {
            let tmp = cmp::min(truck_size, entry[0]);
            ans += tmp * entry[1];
            truck_size -= tmp;
            if truck_size == 0 {
                break;
            }
        }
        
        return ans;
    }
}
