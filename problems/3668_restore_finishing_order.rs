use std::collections::HashSet;

impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let friends_set: HashSet<_> = HashSet::from_iter(friends);
        order.into_iter()
            .filter(|friend| friends_set.contains(friend))
            .collect()
    }
}
