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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, Vec::new())
    }
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut bits: Vec<bool>) -> i32{
        match root {
            Some(node) => {
                bits.push(node.borrow().val == 1);
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return Self::bin_to_dec(&bits);
                }
                return (
                    Self::dfs(&node.borrow().left, bits.clone()) 
                    + Self::dfs(&node.borrow().right, bits)
                );
            },
            None => 0,
        }
    }
    
    fn bin_to_dec(bits: &Vec<bool>) -> i32 {
        let mut ans = 0;
        for (exponent, digit) in bits.iter().rev().enumerate() {
            if !digit {
                continue;
            }
            ans += i32::pow(2, exponent as u32);
        }
        return ans;
    }
}
