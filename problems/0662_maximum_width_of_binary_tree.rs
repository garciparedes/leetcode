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

use std::collections::HashMap;
use std::cmp;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut memory = HashMap::new();
        
        Self::dfs(root, 0, 1, &mut memory);
        
        println!("{:?}", memory);
        
        return memory
            .into_iter()
            .fold(0, |acc, x| cmp::max(acc, x.1[x.1.len() - 1] - x.1[0] + 1));
    }
    
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>, 
        depth: usize, 
        dist: i32, 
        memory: &mut HashMap<usize, Vec<i32>>,
    ) {
        if let Some(node) =  root {
            memory.entry(depth).or_insert_with(Vec::new).push(dist);
            Self::dfs(node.borrow().left.clone(), depth + 1, 2 * dist, memory);
            Self::dfs(node.borrow().right.clone(), depth + 1, 2 * dist + 1, memory);
        }
    }
}
