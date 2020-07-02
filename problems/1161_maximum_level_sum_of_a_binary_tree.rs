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
    collections::VecDeque,
};

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        
        let (mut best_depth, mut best_cumulated) = (1, 0);
        let (mut depth, mut cumulated) = (1, 0);
        
        queue.push_back(root);
        queue.push_back(None);
        while let Some(item) = queue.pop_front() {
            if let Some(node) = item {
                cumulated += node.borrow().val;
                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow().right.clone());
                }
                continue;
            } 
            
            if cumulated > best_cumulated {
                best_cumulated = cumulated;
                best_depth = depth;
            }

            if queue.is_empty() {
                break;
            }

            queue.push_back(None);
            depth += 1;
            cumulated = 0;
        }
        return best_depth;
    }
}
