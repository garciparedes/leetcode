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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::helper(&root, k as usize, 0).0
    }
    
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, k: usize, mut current: usize) -> (i32, usize) {
        match root {
            Some(node) => {
                let (mut val, mut current) = Self::helper(&node.borrow().left, k, current);
                
                if current == k {
                    return (val, current);   
                }
                
                val = node.borrow().val;
                current += 1;
                
                if current == k {
                    return (val, current);
                }
                
                return Self::helper(&node.borrow().right, k, current);
            },
            None => (0, current),
        }
    }
}
