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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Solution::deepest_leaves_sum_with_depth(root, 0).1
    }
    
    fn deepest_leaves_sum_with_depth(root: Option<Rc<RefCell<TreeNode>>>, depth: usize) -> (usize, i32) {
        match root {
            Some(node) => {
                let right = node.borrow().right.clone();
                let left =  node.borrow().left.clone();
                
                let right_result = Solution::deepest_leaves_sum_with_depth(right, depth + 1);
                let left_result = Solution::deepest_leaves_sum_with_depth(left, depth + 1);
                
                if right_result.0 == 0 && left_result.0 == 0 {
                    return (depth, node.borrow().val);
                }
                if right_result.0 == left_result.0 {
                    return (right_result.0, right_result.1 + left_result.1);
                } else if right_result.0 < left_result.0 {
                    return left_result;
                } else {
                    return right_result;
                }
                
            },
            None => (0, 0),
        }
    }
}
