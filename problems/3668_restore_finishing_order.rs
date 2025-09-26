use std::collections::HashSet;

impl Solution {
    pub fn recover_order(mut order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let friends_set: HashSet<_> = HashSet::from_iter(friends);
        order.retain(|friend| friends_set.contains(friend));
        order
    }
}
