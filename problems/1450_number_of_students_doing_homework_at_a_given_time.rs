impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        return start_time
            .iter()
            .zip(end_time)
            .map(|(&lower, upper)| lower <= query_time && query_time <= upper)
            .fold(0, |cum, v| cum + v as i32)
        ;
        
    }
}
