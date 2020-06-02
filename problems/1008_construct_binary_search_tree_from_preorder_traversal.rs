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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        
        let val = preorder[0];
        
        let mut node = Rc::new(RefCell::new(TreeNode::new(val)));
        
        let left_preorder: Vec<i32> = preorder.iter().cloned().filter(|&x| x < val).collect();
        node.borrow_mut().left = Solution::bst_from_preorder(left_preorder);

        let right_preorder: Vec<i32> = preorder.iter().cloned().filter(|&x| x > val).collect();
        node.borrow_mut().right = Solution::bst_from_preorder(right_preorder);
        
        return Some(node);
    }
}
