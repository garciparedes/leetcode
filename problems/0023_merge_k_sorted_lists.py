from heapq import heapify, heappop, heappush

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        vals = [(x.val, i) for i, x in enumerate(lists) if x is not None]
        heapify(vals)
        
        return self._merge(vals, lists)
        
    def _merge(self, vals, lists: list[Optional[ListNode]]) -> Optional[ListNode]:    
        if not len(vals):
            return None
        
        i = heappop(vals)[1]
    
        current = lists[i]
        
        if current.next is not None:
            lists[i] = current.next 
            heappush(vals, (lists[i].val, i))
            
        current.next = self._merge(vals, lists)
        
        return current
        
