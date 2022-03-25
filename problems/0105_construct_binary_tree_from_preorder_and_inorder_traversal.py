# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        
        reversed_inorder = dict()
        for i in range(len(inorder)):
            reversed_inorder[inorder[i]] = i 
            
        return self._traverse(preorder, reversed_inorder, 0, [])[1]
        
        
    def _traverse(self, preorder: List[int], reversed_inorder: Dict[int, int], i: int, path: List[str]) -> Tuple[int, Optional[TreeNode]]:
        if i >= len(preorder) or any(reversed_inorder[p] < reversed_inorder[preorder[i]] for p in path):
            return i, None
        
        
        node = TreeNode(preorder[i])
        
        ii, left = self._traverse(preorder, reversed_inorder, i + 1, [node.val] + path)
        if left is not None:
            i = ii
        node.left = left
                
        ii, right = self._traverse(preorder, reversed_inorder, i + 1, path)
        if right is not None:
            i = ii
        node.right = right
            
        return i, node
        
        
        
        
