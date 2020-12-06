"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""
from typing import List

class Solution:
    def connect(self, root: 'Node') -> 'Node':
        self.dfs(root, 0, [])
        return root
        
    def dfs(self, root: 'Node', depth: int, nexts: List['Node']) -> None:
        if root is None:
            return
        
        while not (len(nexts) > depth):
            nexts.append(None)
        
        self.dfs(root.right, depth + 1, nexts)
                    
        root.next = nexts[depth]
        nexts[depth] = root
        
        self.dfs(root.left, depth + 1, nexts)
