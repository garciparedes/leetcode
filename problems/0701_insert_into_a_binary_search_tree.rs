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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>, val: i32
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                if val < node.borrow().val {
                    let tmp = Self::insert_into_bst(node.borrow_mut().left.take(), val);
                    node.borrow_mut().left = tmp;
                } else {
                    let tmp = Self::insert_into_bst(node.borrow_mut().right.take(), val);
                    node.borrow_mut().right = tmp;
                }
                return Some(node);
            },
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
        }
    }
}
