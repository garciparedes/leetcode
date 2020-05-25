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
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut result = 0;
            
            let l = node.borrow().left.clone();
            let r = node.borrow().right.clone();
            
            result += Solution::sum_even_grandparent(l.clone());
            result += Solution::sum_even_grandparent(r.clone());
            
            if node.borrow().val % 2 != 0 {
                return result;
            }
            
            if let Some(left_node) = l {
                let l1 = left_node.borrow().left.clone();
                if let Some(left_node_1) = l1 {
                    result += left_node_1.borrow().val;
                }
                let l2 = left_node.borrow().right.clone();
                if let Some(left_node_2) = l2 {
                    result += left_node_2.borrow().val;
                }
            }

            if let Some(right_node) = r {
                let r1 = right_node.borrow().left.clone();
                if let Some(right_node_1) = r1 {
                    result += right_node_1.borrow().val;
                }
                let r2 = right_node.borrow().right.clone();
                if let Some(right_node_2) = r2 {
                    result += right_node_2.borrow().val;
                }
            }
            return result
        } 
        return 0;   
    }
}
