from itertools import count 

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        nodes = [list(), list()]
        
        current = head
        for i in count(1):
            if current is None:
                break    
            nodes[i % 2].append(current)
            
            current = current.next
                    
        current = head
        for node in nodes[1]:
            current.next = node
            current = node
            
        for node in nodes[0]:
            current.next = node
            current = node
            
        if current is not None:
            current.next = None
            
        return head
