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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>, 
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root1.is_none() != root2.is_none() {
            return false;
        } 
        if root1.is_none() {
            return true;
        }
        
        let (node1, node2) = (root1.unwrap(), root2.unwrap());
        
        
        if node1.borrow().val != node2.borrow().val {
            return false;
        }
        
        
        return (
            (
                Self::flip_equiv(node1.borrow().left.clone(), node2.borrow().left.clone()) 
                && Self::flip_equiv(node1.borrow().right.clone(), node2.borrow().right.clone())
            )
            ||
            (
                Self::flip_equiv(node1.borrow().left.clone(), node2.borrow().right.clone()) 
                && Self::flip_equiv(node1.borrow().right.clone(), node2.borrow().left.clone())
            )
        );
    }
}
