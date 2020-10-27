# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def detectCycle(self, head: ListNode) -> ListNode:
        slow = head
        fast = head
        
        while fast is not None and fast.next is not  None:
            
            fast = fast.next.next
            slow = slow.next
            
            if fast == slow:
                ans = head
                while ans != slow:
                    slow = slow.next
                    ans = ans.next
                return ans
        return None
