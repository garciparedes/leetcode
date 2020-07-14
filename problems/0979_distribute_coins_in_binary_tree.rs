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
impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root, 0).1
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cost: i32) -> (i32, i32) {
        match root {
            Some(node) => {
                let (left, cost) = Self::dfs(node.borrow().left.clone(), cost);
                let (right, cost) = Self::dfs(node.borrow().right.clone(), cost);
                return (node.borrow().val + left + right - 1, cost + left.abs() + right.abs());
            },
            None => (0, cost),
        }
    }
}
