"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

class Solution:
    def cloneGraph(self, node: 'Node') -> 'Node':
        graph = dict()
        
        def _fn(deno) -> 'Node':
            if deno is None:
                return None
            
            copied = Node(deno.val)
            graph[deno.val] = copied
            for neighbor in deno.neighbors:
                tmp = graph.get(neighbor.val, None)
                if tmp is None:
                    tmp = _fn(neighbor)
                copied.neighbors.append(tmp)
            return copied
            
        return _fn(node)
