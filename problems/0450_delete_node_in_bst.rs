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
use std::cmp::Ordering;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>, 
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                match key.cmp(&val) {
                    Ordering::Less => {
                        let left = Self::delete_node(node.borrow().left.clone(), key);
                        node.borrow_mut().left = left;
                        return Some(node);
                    },
                    Ordering::Equal => {
                        if node.borrow().left.is_none() && node.borrow().right.is_none() {
                            return None;
                        }
                        if node.borrow().right.is_some() {
                            let left = Self::leftmost(node.borrow().right.clone());
                            left.unwrap().borrow_mut().left = node.borrow().left.clone();
                            return node.borrow().right.clone();
                        } else {
                            return node.borrow().left.clone();
                        }
                    },
                    Ordering::Greater => {
                        let right = Self::delete_node(node.borrow().right.clone(), key);
                        node.borrow_mut().right = right;
                        return Some(node);
                    }
                }
            }
            None => None,
        }
    }
    
    fn leftmost(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                if node.borrow().left.is_some() {
                    return Self::leftmost(node.borrow().left.clone());
                }
                return Some(node);
            },
            None => panic!(),
        }
    }
}
