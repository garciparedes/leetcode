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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                if let Some(left_node) = node.borrow().left.clone() {
                    if node.borrow().val != left_node.borrow().val{
                        return false;
                    }
                }
                if let Some(right_node) = node.borrow().right.clone() {
                    if node.borrow().val != right_node.borrow().val{
                        return false;
                    }
                }
                if !Self::is_unival_tree(node.borrow().left.clone()) {
                    return false;
                }
                if !Self::is_unival_tree(node.borrow().right.clone()) {
                    return false;
                }
                
                return true;
            },
            None => true,
        }
    }
}
