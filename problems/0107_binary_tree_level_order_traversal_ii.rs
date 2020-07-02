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

use std::{
    rc::Rc,
    cell::RefCell,
    collections::VecDeque
};

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        let mut queue = VecDeque::new();
        queue.push_back(root);
        queue.push_back(None);
        
        let mut result = Vec::new();
        let mut current = Vec::new();
        while let Some(item) = queue.pop_front() {
            if let Some(node) = item {
                current.push(node.borrow().val);
                
                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow().left.clone());
                }
                
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow().right.clone());
                }
                continue;
            } 
            
            if queue.is_empty() {
                break;
            }
            
            result.push(current);
            current = Vec::new();
            queue.push_back(None);
        }
        result.push(current);
        
        return result
            .into_iter()
            .rev()
            .collect();
    }
}
