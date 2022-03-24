# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return head
        
        if head.next is not None:
            next_ = head.next
            head.next = next_.next
            next_.next = head
            head = next_
            
        current = head.next
        while current is not None and current.next is not None and current.next.next is not None:
            next_ = current.next
            next_next_ = next_.next
            next_next_next_ = next_next_.next
            current.next = next_next_
            next_next_.next = next_
            next_.next = next_next_next_
            current = next_
            
            
        return head
        
        
