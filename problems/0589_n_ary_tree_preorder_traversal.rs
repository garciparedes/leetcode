"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution:
    def preorder(self, root: 'Node') -> List[int]:
        nodes = list()
        nodes.append(root)
        
        result = list()
        while any(nodes): 
            node = nodes.pop()
            for child in reversed(node.children):
                nodes.append(child)
            result.append(node.val)
        return result
