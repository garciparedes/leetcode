# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def convertBST(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        self._traverse(root, 0)
        return root
    
    def _traverse(self, root: Optional[TreeNode], cum: int) -> int:
        if root is None:
            return cum
        
        cum = self._traverse(root.right, cum)
        
        cum += root.val
        root.val = cum
        
        cum = self._traverse(root.left, cum)
        
        return cum
