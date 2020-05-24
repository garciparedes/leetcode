impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        return index
            .iter()
            .ziºp(nums)
            .fold(Vec::new(), |vec, (&i, num)| {
                let mut vec: Vec<i32> = vec.iter().cloned().collect();
                vec.insert(i as usize, num); 
                return vec;
            })
        ;
    }
}

