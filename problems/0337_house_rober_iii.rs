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
use std::cmp;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (a, b) = Self::dfs(&root);
        return cmp::max(a, b);
    }    
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            Some(node) => {                
                let left = Self::dfs(&node.borrow().left);
                let right = Self::dfs(&node.borrow().right);
                return (
                    node.borrow().val + left.1 + right.1,  
                    cmp::max(left.0, left.1) + cmp::max(right.0, right.1),
                );
            },
            None => (0, 0),
        }
    }
}
