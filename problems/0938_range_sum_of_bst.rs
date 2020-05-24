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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        return match root {
            Some(node) => {
                let mut val = node.borrow().val;
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                
                if val < l || r < val {
                    val = 0;
                }

                return val + Solution::range_sum_bst(left, l, r) + Solution::range_sum_bst(right, l, r);     
            },
            None => 0,
        };
        
    }
}
