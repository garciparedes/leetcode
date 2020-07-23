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
use std::cmp::Ordering;
impl Solution {
    pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::dfs(root, 0).0;
    }
    
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: usize) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        match root {
            Some(node) => {
                let (left, depth_left) = Self::dfs(node.borrow().left.clone(), depth + 1);
                let (right, depth_right) = Self::dfs(node.borrow().right.clone(), depth + 1);
                
                return match depth_left.cmp(&depth_right) {
                    Ordering::Less => (right, depth_right),
                    Ordering::Equal => (Some(node), depth_right),
                    Ordering::Greater => (left, depth_left),   
                };
                
            },
            None => (None, depth),
        }
    }
}
