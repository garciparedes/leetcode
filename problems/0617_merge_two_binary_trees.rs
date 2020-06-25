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
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if t1.is_some() && t2.is_some() {
            let (t1_node, t2_node) = (t1.unwrap(), t2.unwrap());
            let val = t1_node.borrow().val + t2_node.borrow().val;
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            
            node.borrow_mut().left = Self::merge_trees(
                t1_node.borrow().left.clone(), t2_node.borrow().left.clone(),
            );
            node.borrow_mut().right = Self::merge_trees(
                t1_node.borrow().right.clone(), t2_node.borrow().right.clone(),
            );
            
            return Some(node)
        }
        if t1.is_some() {
            return t1;
        }
        if t2.is_some() {
            return t2;
        }
        return None;
    }
}
