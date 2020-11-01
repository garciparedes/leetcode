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
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut n = 0;
        let mut head2 = &head;
        while let Some(node) = head2 {
            n += 1;
            head2 = &node.next;
        }
        
        let mut ans = 0;
        while let Some(node) = head {
            n -= 1;
            if node.val == 1 {
                ans += i32::pow(2, n);
            }
            head = node.next;
        }
        return ans;
    }
}
