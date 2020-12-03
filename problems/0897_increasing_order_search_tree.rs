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
    pub fn increasing_bst(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(root, None)
    }
    
    fn helper(
        left: Option<Rc<RefCell<TreeNode>>>, 
        parent: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = left {
            
            let right = Self::helper(node.borrow_mut().right.take(), parent);
            node.borrow_mut().right = right;
            
            let left = node.borrow_mut().left.take();
            
            return Self::helper(left, Some(node));
        } else {
            return parent;
        }
    }
}
