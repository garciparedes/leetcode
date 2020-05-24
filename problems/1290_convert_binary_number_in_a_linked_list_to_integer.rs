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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut degree = 0; 
        let mut current = head.clone();
        while let Some(node) = current {
            degree += 1;
            current = node.next;
        }
        
        let mut value = 0;
        let mut current = head.clone();
        for i in (0..degree).rev() {
            let node = current.unwrap(); 
            if node.val != 0 {
                value += 2i32.pow(i);   
            }
            current = node.next;
        }
        return value;
    }
}
