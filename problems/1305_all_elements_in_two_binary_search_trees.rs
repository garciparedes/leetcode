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
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut base = Solution::extract(root1);
        let mut other = Solution::extract(root2);
        
        let mut i = 0;
        for item in other {
            while i < base.len() && base[i] < item  {
                i +=1;
            }
            base.insert(i, item);
        }
        
        return base;
    }
    
    
    fn extract(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(node) => {
                let mut base = Solution::extract(node.borrow().left.clone());
                let mut other = Solution::extract(node.borrow().right.clone());
                
                base.push(node.borrow().val);
                base.append(&mut other);
                
                return base;
                
            },
            None => Vec::new(),
        }
        
    }
}
