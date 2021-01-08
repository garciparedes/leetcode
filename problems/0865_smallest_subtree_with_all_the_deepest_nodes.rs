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

use std::collections::HashMap;

use std::cmp;
use std::cmp::Ordering;

impl Solution {
    pub fn subtree_with_all_deepest(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {        
        let mut memo = HashMap::new();
        Self::helper(&root, &mut memo);
        while let Some(node) = root {
            let l = if let Some(left) = &node.borrow().left {
                *memo.get(&left.borrow().val).unwrap()
            } else {
                0
            };
            let r = if let Some(right) = &node.borrow().right {
                *memo.get(&right.borrow().val).unwrap()
            } else {
                0
            };
            match l.cmp(&r) {
                Ordering::Less => {
                    root = node.borrow_mut().right.take();
                },
                Ordering::Equal => {
                    root = Some(node);
                    break;
                },
                Ordering::Greater => {
                    root = node.borrow_mut().left.take();
                }
            }
        }
        return root;
        
    }
    
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, memo: &mut HashMap<i32, usize>) -> usize {
        match root {
            Some(node) => {
                let l = Self::helper(&node.borrow().left, memo);
                let r = Self::helper(&node.borrow().right, memo);
                let deepest = 1 + cmp::max(l, r);
                memo.insert(node.borrow().val, deepest);
                return deepest;
            },
            None => 0,
        } 
    }
}
