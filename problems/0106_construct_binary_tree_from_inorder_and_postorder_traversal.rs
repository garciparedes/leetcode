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

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_indexer: HashMap<i32, i32> = inorder
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i as i32))
            .collect();
        
        let n = postorder.len() as i32;
        
        return Self::rec(&mut postorder, &inorder_indexer, 0, n - 1);
    }
    
    fn rec(
        postorder: &mut Vec<i32>, inorder_indexer: &HashMap<i32, i32>, low: i32, high: i32
    ) -> Option<Rc<RefCell<TreeNode>>>{
        if low > high {
            return None;
        }

        let val = postorder.pop().unwrap();
        let index = inorder_indexer[&val];

        let mut tree = TreeNode::new(val);
        
        tree.right = Self::rec(postorder, inorder_indexer, index + 1, high);
        tree.left = Self::rec(postorder, inorder_indexer, low, index - 1);
        
        return Some(Rc::new(RefCell::new(tree)));
    }
}
