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

struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        FindElements {
            root: root,
        }
    }
    
    fn find(&self, target: i32) -> bool {
        return Self::rec_find(&self.root, 0, target);
    }
    
    fn rec_find(root: &Option<Rc<RefCell<TreeNode>>>, val:i32, target:i32) -> bool {
        match root {
            Some(node) => (
                !(val > target) 
                && 
                (
                    val == target 
                    || Self::rec_find(&node.borrow().left.clone(), 2 * val + 1, target) 
                    || Self::rec_find(&node.borrow().right.clone(), 2 * val + 2, target)
                )
            ),
            None => false,
        }
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
