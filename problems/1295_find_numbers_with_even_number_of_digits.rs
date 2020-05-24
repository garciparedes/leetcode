impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        return nums
            .iter()
            .map(|&num| {
                let mut counter = 0;
                let mut current = num;
                while current != 0 {
                    counter += 1;
                    current /= 10;
                }
                return counter;
            })
            .fold(0, |cum, v| cum + (v % 2 == 0) as i32)
        ;
    }
}
