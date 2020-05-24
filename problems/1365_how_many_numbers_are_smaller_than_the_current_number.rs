
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        return nums
            .iter()
            .map( |&current| {
                let mut counter = 0;
                for &other in nums.iter() {
                    counter += (other < current) as i32;
                }
                return counter;
            })
            .collect::<Vec<i32>>()
        ;
    }
}
