# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def pairSum(self, head: Optional[ListNode]) -> int:
        values = list()
        
        a = head
        b = head
        while b is not None:
            values.append(a.val)
            
            a = a.next
            b = b.next.next
        
        i = len(values) - 1
        while a is not None:
            values[i] += a.val
            i -= 1
            a = a.next
         
        ans = max(values)
        
        return ans

            
        
