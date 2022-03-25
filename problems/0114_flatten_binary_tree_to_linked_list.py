# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def flatten(self, root: Optional[TreeNode]) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        return self.traverse(root, None)
        
    
    def traverse(self, node, another):
        if node is None:
            return another
        
        another = self.traverse(node.right, another)
        
        node.right = self.traverse(node.left, another)
        node.left = None
        
        return node
