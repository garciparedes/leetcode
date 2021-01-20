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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(&root, 0).unwrap().1
    }
    
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize) -> Option<(usize, i32)> {
        match root {
            Some(node) => {
                let left = Self::helper(&node.borrow().left, depth + 1);
                let right = Self::helper(&node.borrow().right, depth + 1);
                
                let ans = match (left, right) {
                    (None, None) => (depth, node.borrow().val),
                    (Some((dl, vl)), None) => (dl, vl),
                    (None, Some((dr, vr))) => (dr, vr),
                    (Some((dl, vl)), Some((dr, vr))) => {
                        if dl >= dr {
                            (dl, vl)
                        } else {
                            (dr, vr)
                        }
                    }
                };
                return Some(ans);
            
            },
            None => None
        }
    }
}
