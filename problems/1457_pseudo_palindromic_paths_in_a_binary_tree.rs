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
    pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::dfs(&root, HashMap::new(), 0);
    }
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut counter: HashMap<i32, usize>, n: usize) -> i32 {
        match root {
            Some(node) => {
                *counter.entry(node.borrow().val).or_insert(0) += 1;
                
                if (
                    node.borrow().left.is_none() 
                    && node.borrow().right.is_none() 
                    && Self::check(&counter, n + 1))
                {
                    return 1
                }
                
                return (
                    Self::dfs(&node.borrow().left, counter.clone(), n + 1) 
                    + Self::dfs(&node.borrow().right, counter, n + 1) 
                );
            }
            None => 0,
        }
    }
    
    fn check(counter: &HashMap<i32, usize>, n: usize) -> bool {
        let mut odd = false;
        for count in counter.values() {
            if count % 2 == 0 {
                continue;
            } 
            if n % 2 == 0 {
                return false;
            }
            if !odd {
                odd = true;
            } else {
                return false;
            }
        }
        return true;
    }
}
