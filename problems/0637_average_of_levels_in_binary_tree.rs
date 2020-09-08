// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;


impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        if root.is_none() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        queue.push_back(None);
        let mut tmp = Vec::new();
        while let Some(item) = queue.pop_front() {
            if let Some(node) = item {
                tmp.push(node.borrow().val as f64);
                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow().right.clone());
                }
            } else {
                if !queue.is_empty() {
                    queue.push_back(None);
                }
                let n = tmp.len() as f64;
                ans.push(tmp.iter().sum::<f64>() / n);
                tmp.clear();
            }
        }
        return ans;
    }
}
