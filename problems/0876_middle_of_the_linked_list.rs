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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let n = Self::get_length(&head);
        return Self::get(head,  n / 2)
    }
    
    fn get_length(head: &Option<Box<ListNode>>) -> usize {
        match head {
            Some(item) => 1 + Self::get_length(&item.next),
            None => 0,
        }
    }
    
    fn get(head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        return Self::get(head.unwrap().next, k - 1);
    }
}
