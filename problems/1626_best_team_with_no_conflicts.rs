use std::cmp;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut team: Vec<_> = ages.into_iter().zip(scores.into_iter()).collect();
        let n = team.len();
        team.sort_unstable_by_key(|&p| cmp::Reverse(p));
        
        let mut ans = 0;
        let mut dp = vec![0; n];
        for i in 0..n {
            let score = team[i].1;
            dp[i] = score;
            for j in 0..i {
                if team[j].1 >= team[i].1 {
                    dp[i] = cmp::max(dp[i], dp[j] + score);
                }
            }
            ans = cmp::max(ans, dp[i]);
        }
        return ans;
    }
}
