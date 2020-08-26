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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                return Self::dfs(&Some(node), val, val);
            },
            None => 0,   
        }
    }
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut minimum: i32, mut maximum: i32) -> i32 {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                minimum = cmp::min(minimum, val);
                maximum = cmp::max(maximum, val);
                
                return cmp::max(
                    Self::dfs(&node.borrow().left, minimum, maximum),
                    Self::dfs(&node.borrow().right, minimum, maximum),
                )
            },
            None => maximum - minimum,
        }
    }
}
