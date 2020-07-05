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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::rec(root, i32::min_value());
    }
    
    fn rec(root: Option<Rc<RefCell<TreeNode>>>, mut maximum: i32) -> i32 {
        match root {
            Some(node) => {
                maximum = cmp::max(maximum, node.borrow().val);
                return (
                    (node.borrow().val == maximum) as i32 + 
                    Self::rec(node.borrow().left.clone(), maximum) +  
                    Self::rec(node.borrow().right.clone(), maximum)
                )
            },
            None => 0,
        }
    }
}
