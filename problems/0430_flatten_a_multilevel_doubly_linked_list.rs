"""
# Definition for a Node.
class Node:
    def __init__(self, val, prev, next, child):
        self.val = val
        self.prev = prev
        self.next = next
        self.child = child
"""

class Solution:
    def flatten(self, head: 'Node') -> 'Node':
        if head is None:
            return None
        
        if head.child:
            last = self.tail(head.child)
            last.next = head.next
            if head.next is not None:
                head.next.prev = last
            
            head.next = head.child
            head.next.prev = head
            head.child = None
            
        self.flatten(head.next)
        
        return head
    
    def tail(self, head):
        if head.next:
            return self.tail(head.next)
        return head
        
