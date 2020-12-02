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

use rand::prelude::*;
use rand::Rng;

struct Solution {
    rng: ThreadRng,
    n: usize,
    head: Option<Box<ListNode>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    pub fn new(head: Option<Box<ListNode>>) -> Self {        
        
        Self { 
            rng: rand::thread_rng(), 
            n: Self::count(&head),
            head: head, 
        }
    }
    
    fn count(head: &Option<Box<ListNode>>) -> usize {
        match head {
            Some(node) => 1 + Self::count(&node.next),
            None => 0,
        }
    }
    
    /** Returns a random node's value. */
    pub fn get_random(&mut self) -> i32 {
        let i = self.rng.gen_range(0, self.n) as usize;
        return Self::get(&self.head, i);
    }
    
    fn get(head: &Option<Box<ListNode>>, i: usize) -> i32 {
        if let Some(node) = head {
            if i == 0 {
                node.val
            } else {
                Self::get(&node.next, i - 1)
            }
        } else {
            panic!()   
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */
