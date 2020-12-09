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

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut new = Self { stack: Vec::new() };
        new.push_left(root);
        return new;
    }
    
    fn next(&mut self) -> i32 {
        match self.stack.pop() {
            Some(mut node) => {
                self.push_left(node.borrow_mut().right.take());
                return node.borrow_mut().val;
            },
            None => panic!(),
        }
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
    
    fn push_left(&mut self, mut root: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(mut node) = root {
            root = node.borrow_mut().left.take();
            self.stack.push(node);
        }
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
