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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root)
    }
    
    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let mut result = Self::dfs(&node.borrow().right);
                if (
                    node.borrow().left.as_ref().is_some() 
                    && node.borrow().left.as_ref().unwrap().borrow().left.is_none() 
                    && node.borrow().left.as_ref().unwrap().borrow().right.is_none()
                ) {
                    result += node.borrow().left.as_ref().unwrap().borrow().val;
                } else {
                    result += Self::dfs(&node.borrow().left);
                }
                return result;
            },
            None => 0,
        }
    }
}
