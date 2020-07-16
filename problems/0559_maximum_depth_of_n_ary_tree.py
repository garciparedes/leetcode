"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution:
    def maxDepth(self, root: 'Node', depth: int = 1) -> int:
        if root is None:
            return depth - 1
        max_depth = depth
        for child in root.children:
            max_depth = max(max_depth, self.maxDepth(child, depth + 1))
        return max_depth
