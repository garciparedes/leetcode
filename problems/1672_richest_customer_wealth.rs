impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.into_iter().map(|account| account.into_iter().sum()).max().unwrap_or(0)
    }
}
