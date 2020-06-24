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
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];    
        } 
        
        let mut result = Vec::new();
        for i in (1..n-1).step_by(2) {
            let lefts = Solution::all_possible_fbt(i);
            let rights = Solution::all_possible_fbt(n - 1 - i);
            for left in lefts {
                for right in rights.iter() {
                    let node = Rc::new(RefCell::new(TreeNode::new(0)));
                    node.borrow_mut().left = left.clone();
                    node.borrow_mut().right = right.clone();
                    result.push(Some(node));
                }
            }
        }
        return result;
    }
}
