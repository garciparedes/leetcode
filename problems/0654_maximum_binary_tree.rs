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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {        
        let max_index = nums.iter().enumerate().max_by(|(i, x), (j, y)| x.cmp(y))?.0;
        
        let mut tree = TreeNode::new(nums[max_index]);
        
        let head = nums[0..max_index].to_vec();
        let tail = nums[max_index + 1..nums.len()].to_vec();
        
        tree.left = Solution::construct_maximum_binary_tree(head);
        tree.right = Solution::construct_maximum_binary_tree(tail);
        
        return Some(Rc::new(RefCell::new(tree)));
    }
}
