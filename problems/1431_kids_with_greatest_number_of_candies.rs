impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        return match candies.iter().max() {
            Some(&maximum) => candies.iter().map(|c| c + extra_candies >= maximum).collect::<Vec<bool>>(),
            None => Vec::new(),
        };       
    }
}
