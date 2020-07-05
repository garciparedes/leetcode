from collections import deque

"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution:
    def postorder(self, root: 'Node') -> List[int]:
        nodes = list()
        nodes.append(root)
        
        result = deque()
        while any(nodes): 
            node = nodes.pop()
            for child in node.children:
                nodes.append(child)
            result.appendleft(node.val)
        return result
