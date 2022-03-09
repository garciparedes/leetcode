# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        current = head
        while current is not None:
            if current.next is not None and current.next.val > 0:
                current.val += current.next.val
                current.next = current.next.next
            elif current.next is not None and current.next.next is None and current.next.val == 0:
                current.next = None
            else:      
                current = current.next
        return head
