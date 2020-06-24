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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::rec(&root);
        return root;
    }
    
    fn rec(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let left = Solution::rec(&node.borrow_mut().left);
                let right = Solution::rec(&node.borrow_mut().right);
                
                if !left {
                    node.borrow_mut().left = None;
                }
                if !right {
                    node.borrow_mut().right = None;
                }
                
                return left || right || (node.borrow().val == 1);        
            },
            None => return false,
        };
        
        
    }
}
