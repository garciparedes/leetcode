// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        let n = Self::len(&head);
        
        let a = Self::get(&head, k - 1);
        let b = Self::get(&head, n - k);
        
        Self::set(&mut head, k - 1, b);
        Self::set(&mut head, n - k, a);
        
        return head;
    }
    
    fn len(head: &Option<Box<ListNode>>) -> usize {
        match head {
            Some(node) => 1 + Self::len(&node.next),
            None => 0,
        }
    }
    
    fn get(head: &Option<Box<ListNode>>, k: usize) -> i32 {
        match head {
            Some(node) => {
                if k == 0 {
                    node.val
                } else {
                    Self::get(&node.next, k - 1)    
                }           
            },
            None => -1,
        }
    }
    
    fn set(head: &mut Option<Box<ListNode>>, k: usize, value: i32) {
        if k == 0 {
            head.as_mut().unwrap().val = value
        } else {
            Self::set(&mut head.as_mut().unwrap().next, k - 1, value)    
        }
        
    }
    
}

