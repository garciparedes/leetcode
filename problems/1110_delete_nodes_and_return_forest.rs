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
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        
        let mut result = vec![root];
        for val in to_delete {
            let mut head = Vec::new();
            for root in result.iter_mut() {
                head = Self::delete(root, val);
                if !head.is_empty() {
                    break;
                }
            }
            result.extend(head.into_iter());    
        }
        return result.into_iter().filter(|x| x.is_some()).collect();
    }
    
    fn delete(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        match root {
            Some(node) => {
                if node.borrow().val == val {
                    let mut result = Vec::new();
                    let left = node.borrow().left.clone();
                    if left.is_some() {
                        result.push(left);
                    }
                    let right = node.borrow().right.clone();
                    if right.is_some() {
                        result.push(right);
                    }
                    *root = None;
                    return result;
                }
                
                let left = Self::delete(&mut node.borrow_mut().left, val);
                if !left.is_empty() {
                    return left;
                }
                
                let right = Self::delete(&mut node.borrow_mut().right, val);
                if !right.is_empty() {
                    return right;
                }
                
                return Vec::new();
            },
            None => Vec::new(),
        }
    }
}
