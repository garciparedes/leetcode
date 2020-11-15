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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Self::dfs(&root, low, high)
    }
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                let mut ans = 0;
                if low <= val && val <= high {
                    ans += val;
                }
                if val > low {
                    ans += Self::dfs(&node.borrow().left, low, high);
                }
                if val < high {
                    ans += Self::dfs(&node.borrow().right, low, high);
                }
                return ans;
            },
            None => 0,
        }
    }
}
