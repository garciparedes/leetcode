"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""
from collections import deque


class Solution:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        if root is None:
            return list()
        
        queue = deque()
        queue.append(root)
        queue.append(None)
        
        result = list()
        current = list()
        while queue:
            item = queue.popleft()
            
            if item is not None:
                queue.extend(item.children)
                current.append(item.val)
                continue

            result.append(current)
            current = list()
            if len(queue):
                queue.append(None)
                
        return result
            
            
        
