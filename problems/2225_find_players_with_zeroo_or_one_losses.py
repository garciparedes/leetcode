class Solution:
    def findWinners(self, matches: List[List[int]]) -> List[List[int]]:
        scores = dict()
        
        for match in matches:
            if match[0] not in scores:
                scores[match[0]] = [0, 0]
            if match[1] not in scores:
                scores[match[1]] = [0, 0]
            
            scores[match[0]][0] += 1
            scores[match[1]][1] += 1 
        
        ans = [list(), list()]
        for name, score in scores.items():
            wins, losses = score
            
            if losses == 0:
                ans[0].append(name)
            elif losses == 1: 
                ans[1].append(name)
                
        ans[0].sort()
        ans[1].sort()
            
        return ans
            
            
