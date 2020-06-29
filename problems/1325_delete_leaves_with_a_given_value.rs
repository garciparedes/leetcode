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
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root?;
        
        let left = Self::remove_leaf_nodes(node.borrow().left.clone(), target);
        let right = Self::remove_leaf_nodes(node.borrow().right.clone(), target);
        
        if node.borrow().val == target && left.is_none() && right.is_none() {
            return None;
        }
        
        node.borrow_mut().left = left;
        node.borrow_mut().right = right;
        
        return Some(node);
    }
}
