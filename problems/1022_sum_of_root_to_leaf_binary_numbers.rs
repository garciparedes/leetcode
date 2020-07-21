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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, Vec::new())
    }
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut bits: Vec<i32>) -> i32 {
        match root {
            Some(node) => {
                bits.push(node.borrow().val);
                
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return Self::to_number(&bits);
                }
                                
                return (
                    Self::dfs(&node.borrow().left, bits.clone()) 
                    + Self::dfs(&node.borrow().right, bits)
                );
            }
            None => 0,
        }
    }
    
    fn to_number(bits: &Vec<i32>) -> i32 {
        bits
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, v)| acc + v * i32::pow(2, i as u32))
    }
        
}
