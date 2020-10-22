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
use std::cmp::Ordering;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match &root {
            Some(node) => {
                let l = Self::min_depth(node.borrow_mut().left.take());
                let r = Self::min_depth(node.borrow_mut().right.take());
                
                1 + match (l.cmp(&0), r.cmp(&0)) {
                    (Ordering::Equal, Ordering::Greater) => r,
                    (Ordering::Greater, Ordering::Equal) => l,
                    (Ordering::Greater, Ordering::Greater) => cmp::min(l, r),
                    _ => 0,
                }
            },
            None => 0,
        }
    }
}
