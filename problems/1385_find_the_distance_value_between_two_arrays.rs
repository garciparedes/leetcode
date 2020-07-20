impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        arr1
            .into_iter()
            .filter(|v1| arr2.iter().all(|&v2| i32::abs(v2 - v1) > d))
            .count() as i32
    }
}
