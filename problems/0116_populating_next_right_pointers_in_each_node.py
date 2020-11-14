"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""
from collections import deque

class Solution:
    def connect(self, root: 'Node') -> 'Node':
        next_to = None

        queue = deque()
        queue.append(root)
        queue.append(None)
 
        while len(queue):
            item = queue.popleft()
            if item is None:
                next_to = None
                if queue[0] is None:
                    break
                queue.append(None)
            else: 
                item.next = next_to
                next_to = item
                queue.append(item.right)
                queue.append(item.left)
        return root

