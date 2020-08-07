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
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut items = HashMap::new();
        
        Self::dfs(&root, 0, 0, &mut items);
        
        let mut items: Vec<_> = items.into_iter().collect();
        items.sort_unstable();
                
        return items
            .into_iter()
            .map(|(_, mut b)| {
                b.sort_unstable();
                return b.into_iter().map(|(_, d)| d).collect();
            }).collect();    
    }
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32, items: &mut HashMap<i32, Vec<(i32, i32)>>) {
        if let Some(node) = root {
            items.entry(x).or_insert_with(Vec::new).push((y, node.borrow().val));
            
            Self::dfs(&node.borrow().left, x - 1, y + 1, items);
            Self::dfs(&node.borrow().right, x + 1, y + 1, items);
        } 
    }
}
