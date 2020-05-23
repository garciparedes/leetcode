impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut ages = ages.clone();
        ages.sort();
        
        let mut request_count: i32 = 0;
        for (i, &age_a) in ages.iter().enumerate() {
            
            let mut r = i;
            while r < ages.len() - 1 && ages[r + 1] == age_a {
                r += 1;
            }

            let mut l = 0;
            while l < r && ages[l] as f32 <= 0.5 * age_a as f32 + 7.0 {
                l += 1;
            }
            if age_a < 100 {
               while l < r && ages[l] >= 100 {
                   l += 1;
                }
            }
            request_count += (r - l) as i32;
        }
        return request_count;
    }
}
