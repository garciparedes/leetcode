impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut counter = 0;
        for i in 0..rating.len() {
            for j in (i + 1)..rating.len() {
                for k in (j + 1)..rating.len() {
                    counter += (
                        (rating[i] < rating[j] && rating[j] < rating[k]) 
                        || (rating[k] < rating[j] && rating[j] < rating[i])
                    ) as i32;
                }
            }
        }
        return counter;
    }
}
