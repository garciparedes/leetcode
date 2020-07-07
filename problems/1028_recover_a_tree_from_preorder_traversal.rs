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
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None;
        
        for (val, depth) in Self::extract(&s) {
            root = Self::add(root, val, depth);
        }
        
        return root;
    }
    
    fn extract(s: &str) -> Vec<(i32, usize)> {
        let mut result = Vec::new();
        let mut depth = 0;
        let mut number = String::new();
        for item in s.chars() {
            if item == '-' {
                if number.len() > 0 {
                    result.push((number.parse::<i32>().unwrap(), depth));
                    depth = 0;
                    number.clear();
                }
                depth += 1;
            } else {
                number.push(item);
            }
        }
        if number.len() > 0 {
            result.push((number.parse::<i32>().unwrap(), depth));
        }
        return result;
        
    }
    
    fn add(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_some() && depth == 0 {
            return None;
        }
        if depth == 0 {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        
        let node = root?;
        
        if !node.borrow().right.is_some() {
            let left = Self::add(node.borrow_mut().left.clone(), val, depth - 1);
            if left.is_some() {
                node.borrow_mut().left = left;
                return Some(node);
            } 
        }
        
        let right = Self::add(node.borrow_mut().right.clone(), val, depth - 1);
        if right.is_some() {
            node.borrow_mut().right = right;
            return Some(node);
        }
        return None;
    }
}
