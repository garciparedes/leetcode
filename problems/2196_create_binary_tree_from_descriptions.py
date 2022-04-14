# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def createBinaryTree(self, descriptions: List[List[int]]) -> Optional[TreeNode]:
        nodes = dict()
        children = set()
        
        for description in descriptions:
            parent, child, is_left = description

            children.add(child)
            
            if parent in nodes:
                parent = nodes[parent]
            else:
                parent = TreeNode(parent)
                nodes[parent.val] = parent
                
            if child in nodes:
                child = nodes[child]
            else:
                child = TreeNode(child)
                nodes[child.val] = child
            
            if is_left:
                parent.left = child
            else:
                parent.right = child
                
        index = next(iter(nodes.keys() - children), None)
        if index is None:
            return None
        return nodes[index]
