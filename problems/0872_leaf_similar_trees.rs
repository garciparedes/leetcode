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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let leafs1 = Self::extract_leafs(&root1);
        let leafs2 = Self::extract_leafs(&root2);
        
        return (
            (leafs1.len() == leafs2.len())
            && leafs1
                .into_iter()
                .zip(leafs2.into_iter())
                .all(|(a, b)| a == b)
        );
    }
    
    fn extract_leafs(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(node) => {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return vec![node.borrow().val];
                }
                
                let mut result = Self::extract_leafs(&node.borrow().left);
                result.extend(Self::extract_leafs(&node.borrow().right).into_iter());
                return result;
            },
            None => Vec::new(),
        }
    }
}
