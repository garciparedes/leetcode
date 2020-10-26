use std::cmp;

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut p: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        tokens.sort_unstable();
        
        let (mut ans, mut points) = (0, 0);
        let (mut i, mut j) = (0, tokens.len() - 1);
        while i <= j {
            if p >= tokens[i] {
                p -= tokens[i];
                i += 1;
                points += 1;
                ans = cmp::max(ans, points);
            } else if points > 0 {
                points -= 1;
                p += tokens[j];
                j -= 1;
            } else {
                break;
            }
        }
        return ans;
    }
}
