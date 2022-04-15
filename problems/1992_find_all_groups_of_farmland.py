class Solution:
    def findFarmland(self, land: List[List[int]]) -> List[List[int]]:
        
        ans = list()
        for i in range(len(land)):
            for j in range(len(land[0])):
                found = self._explore(land, i, j)
                if found is None:
                    continue
                ans.append(found)
        return ans
    
    def _explore(self, land: List[List[int]], i: int, j: int) -> Optional[List[int]]:
        if i < 0 or i >= len(land) or j < 0 or j >= len(land[0]) or land[i][j] != 1:
            return None
        
        land[i][j] = 2
        
        ans = [i, j, i ,j]
        
        tmp = self._explore(land, i - 1, j)
        self._update_ans(ans, tmp)
            
        tmp = self._explore(land, i + 1, j)
        self._update_ans(ans, tmp)
        
        tmp = self._explore(land, i, j - 1)
        self._update_ans(ans, tmp)
                
        tmp = self._explore(land, i, j + 1)
        self._update_ans(ans, tmp)
        
        return ans
    
    @staticmethod
    def _update_ans(ans: List[int], tmp: Optional[List[int]]) -> None:
        if tmp is None:
            return
        
        if tmp[0] < ans[0]:
            ans[0] = tmp[0]
        if tmp[1] < ans[1]:
            ans[1] = tmp[1]
        if tmp[2] > ans[2]:
            ans[2] = tmp[2]
        if tmp[3] > ans[3]:
            ans[3] = tmp[3]

        
