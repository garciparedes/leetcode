class Solution:
    def executeInstructions(self, n: int, startPos: List[int], s: str) -> List[int]:
        ans = list()
        for i in range(len(s)):
            current = startPos.copy()

            ans_i = 0
            for j in range(i, len(s)):
                a = s[j]
                if a == "L":
                    current[1] -= 1
                elif a == "R":
                    current[1] += 1
                elif a == "U":
                    current[0] -= 1                
                else:
                    current[0] += 1
                    
                if current[0] < 0 or current[0] >= n or current[1] < 0 or current[1] >= n:
                    break
                    
                ans_i += 1
                                
            ans.append(ans_i)
            
        return ans
