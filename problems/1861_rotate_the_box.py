class Solution:
    def rotateTheBox(self, box: List[List[str]]) -> List[List[str]]:
        ans = list()
        for j in range(len(box[0])):
            row = list()
            for i in reversed(range(len(box))):
                row.append(box[i][j])
            ans.append(row)
                        
        for j in range(len(ans[0])):    
            displaced = True
            while displaced:
                displaced = False
                for i in reversed(range(1, len(ans))):
                    if ans[i][j] == '.' and ans[i - 1][j] == '#':
                        ans[i][j] = '#'
                        ans[i - 1][j] = '.'
                        displaced = True
            
        return ans
