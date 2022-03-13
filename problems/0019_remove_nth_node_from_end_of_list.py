# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        length = self.get_length(head)
        
        if length == n:
            return head.next
        
        target = length - n
        
        current = head
        for _ in range(target - 1):
            current = current.next
                
        current.next = current.next.next
        
        return head
        
    def get_length(self, head: Optional[ListNode]) -> int:
        if head is None:
            return 0
        return self.get_length(head.next) + 1
