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
    
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::bst_to_gst_recursion(&root, 0);
        return root;
    }
    
    fn bst_to_gst_recursion(root: &Option<Rc<RefCell<TreeNode>>>, cum: i32) -> i32 {
        return match root {
            Some(node) => {
                let right_sum = Solution::bst_to_gst_recursion(&node.borrow().right, cum);
                node.borrow_mut().val += right_sum;
                
                let left_sum = Solution::bst_to_gst_recursion(&node.borrow().left, node.borrow().val);
                return cmp::max(node.borrow().val.clone(), left_sum);
                
            },
            None => cum,
        }
    }
}
