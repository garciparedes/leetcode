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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        
        let nodes = Self::extract(root);        
        for w in nodes.as_slice().windows(2) {
            if let Some(node) = &w[0] {
                node.borrow_mut().left = None;
                node.borrow_mut().right = w[1].clone();
            }
        }
        
        return nodes[0].clone();
    }
    
    fn extract(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        match root {
            Some(node) => {
                let mut base = Self::extract(node.borrow().left.clone());
                base.push(Some(node.clone()));
                base.extend(Self::extract(node.borrow().right.clone()).into_iter());
                return base;
            },
            None => Vec::new(),
        }
    }
}
