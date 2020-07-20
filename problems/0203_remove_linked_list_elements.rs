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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut current = &mut head;
        while current.is_some() {
            if current.as_ref().unwrap().val == val {
                *current = current.take().unwrap().next;
            } else {
                current = &mut current.as_mut().unwrap().next;
            }
        }
        return head;
    }
}
