impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        return index
            .iter()
            .zip(nums)
            .fold(Vec::new(), |mut vec, (&i, num)| {
                vec.insert(i as usize, num); 
                return vec;
            })
        ;
    }
}
