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
use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        let mut queue = VecDeque::new();
        queue.push_back(root);
        queue.push_back(None);
        
        let mut result = Vec::new();
        let mut tmp = Vec::new();
        while let Some(item) = queue.pop_front() {
            match item {
                Some(node) => {
                    tmp.push(node.borrow().val);
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow().left.clone())
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow().right.clone())
                    }
                }
                None => {
                    if result.len() % 2 == 1 {
                        tmp = tmp.into_iter().rev().collect();
                    }
                    result.push(tmp);
                    tmp = Vec::new();
                    if queue.back().is_some() {
                        queue.push_back(None);   
                    }
                }
            }
        }
        return result;    
    }
}
