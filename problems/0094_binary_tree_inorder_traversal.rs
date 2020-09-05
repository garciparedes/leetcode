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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        
        let mut stack = Vec::new();
        let mut current = root;
        while current.is_some() || !stack.is_empty() {
            while current.is_some() {
                stack.push(current.clone());
                current = current.unwrap().borrow().left.clone();
            }
            current = stack.pop().unwrap();
            let val = current.clone().unwrap().borrow().val;
            ans.push(val);
            current = current.unwrap().borrow().right.clone();
        }
        
        return ans;
    }
}
