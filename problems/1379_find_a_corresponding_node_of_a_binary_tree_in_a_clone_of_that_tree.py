# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None
from typing import (
    Optional,
)

class Solution:
    def getTargetCopy(self, original: TreeNode, cloned: TreeNode, target: TreeNode) -> TreeNode:        
        return self._search(cloned, target.val)
    
    def _search(self, root: TreeNode, target_value: int) -> Optional[TreeNode]:
        if root is None:
            return None
        
        if root.val == target_value:
            return root
        
        left = self._search(root.left, target_value)
        if left is not None:
            return left
        
        right = self._search(root.right, target_value)
        if right is not None:
            return right
        
        return None
