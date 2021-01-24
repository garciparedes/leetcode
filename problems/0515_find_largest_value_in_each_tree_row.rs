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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        
        Self::helper(root, 0, &mut ans);
        
        return ans;
    }
    
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, ans: &mut Vec<i32>) {
        match root {
            Some(node) => {
                while depth >= ans.len() {
                    ans.push(i32::min_value());
                }
                
                if ans[depth] < node.borrow().val {
                    ans[depth] = node.borrow().val;
                }
                
                Self::helper(node.borrow_mut().left.take(), depth + 1, ans);
                Self::helper(node.borrow_mut().right.take(), depth + 1, ans);
            },
            None => (),
        }
    }
}
