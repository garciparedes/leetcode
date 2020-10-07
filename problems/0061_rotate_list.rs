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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let size = Self::len(&head);
        if size == 0 {
            return head;
        }
        
        let k = (k as usize) % size;
        if k == 0 {
            return head;
        }
        
        let mut target = Self::search(head.clone(), size - k);
        Self::replace_at(&mut head, size - k - 1, None);
        Self::replace_at(&mut target, k - 1, head);

        return target;
    }
    
    fn len(head: &Option<Box<ListNode>>) -> usize {
        match head {
            Some(node) => 1 + Self::len(&node.next),
            None => 0,
        }
    }
    
    fn search(head: Option<Box<ListNode>>, depth: usize) -> Option<Box<ListNode>> {
        match head {
            Some(node) => {
                if depth == 0 {
                    return Some(node);
                } 
                return Self::search(node.next.clone(), depth - 1);
            }
            None => None,
        }
    }
    
    fn replace_at(head: &mut Option<Box<ListNode>>, depth: usize, tail: Option<Box<ListNode>>) {
        let mut current = head.as_mut();
        for i in 0..depth {
            current = current.map(|node| (&mut node.next).as_mut()).unwrap();
        }
        if let Some(v) = current.as_mut() {
            (*v).next = tail;
        }
    }
}
