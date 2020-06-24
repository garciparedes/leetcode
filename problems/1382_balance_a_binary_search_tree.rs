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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let nodes = Self::extract(&root);
        return Self::rebuild(&nodes);
    }
    
    fn extract(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let mut base = Self::extract(&node.borrow().left);                
                let other = Self::extract(&node.borrow().right);
                
                base.push(node.clone());
                base.extend(other.into_iter());
                
                return base;
            }
            None => Vec::new(),
        }
    }
    
    fn rebuild(nodes: &[Rc<RefCell<TreeNode>>]) -> Option<Rc<RefCell<TreeNode>>> {
        if nodes.is_empty() {
            return None;
        }
        
        let mid = nodes.len() / 2;
        
        let node = nodes[mid].clone();
        node.borrow_mut().left = Self::rebuild(&nodes[..mid]);
        node.borrow_mut().right = Self::rebuild(&nodes[mid + 1..]);
        
        return Some(node);
    }

    
}
