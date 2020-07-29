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
    pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let (mut pre_index, mut post_index) = (0, 0);
        
        return Self::rec(&pre, &post, &mut pre_index, &mut post_index)
    }
    
    fn rec(pre: &Vec<i32>, post: &Vec<i32>, pre_index: &mut usize, post_index: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        let mut tree = TreeNode::new(pre[*pre_index]);
        *pre_index += 1;
        
        if tree.val != post[*post_index] {
            tree.left = Self::rec(pre, post, pre_index, post_index);
        }
        if tree.val != post[*post_index] {
            tree.right = Self::rec(pre, post, pre_index, post_index);
        }
        *post_index += 1;
    
        return Some(Rc::new(RefCell::new(tree)));
    }
}
