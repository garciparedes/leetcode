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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() != q.is_none() {
            return false;
        }
        if p.is_none() {
            return true;
        }

        let (p_node, q_node) = (p.unwrap(), q.unwrap());
        if p_node.borrow().val != q_node.borrow().val {
            return false;
        }
        if !Self::is_same_tree(p_node.borrow().left.clone(), q_node.borrow().left.clone()) {
            return false;
        }
        
        if !Self::is_same_tree(p_node.borrow().right.clone(), q_node.borrow().right.clone()) {
            return false;
        }
        return true;
    }
}
