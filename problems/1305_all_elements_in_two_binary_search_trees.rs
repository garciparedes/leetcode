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
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let list1 = Self::dfs(&root1);
        let list2 = Self::dfs(&root2);
        
        return Self::merge(&list1, &list2);
    }
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(node) => {
                let mut ans = Self::dfs(&node.borrow().left);
                ans.push(node.borrow().val);
                ans.extend(Self::dfs(&node.borrow().right).iter());
                return ans;
            },
            None => Vec::new(),
        }       
    }
    
    fn merge(list1: &Vec<i32>, list2: &Vec<i32>) -> Vec<i32> {
        let (mut i1, mut i2) = (0, 0);
        let mut ans = Vec::new();
        while i1 < list1.len() || i2 < list2.len() {
            if i1 < list1.len() && i2 < list2.len() {
                if list1[i1] < list2[i2] {
                    ans.push(list1[i1]);
                    i1 += 1;
                } else {
                    ans.push(list2[i2]);
                    i2 += 1;
                }
            } else if i1 < list1.len() {
                ans.push(list1[i1]);
                i1 += 1;
            } else {
                ans.push(list2[i2]);
                i2 += 1;
            }
        }
        return ans;
    }
}
