class Solution:
    def countPoints(self, rings: str) -> int:
        rods = [set() for _ in range(10)]
        
        for i in range(len(rings) // 2):
            rods[int(rings[2 * i + 1])].add(rings[2 * i])
            
        ans = 0
        for rod in rods:
            if len(rod) == 3:
                ans += 1
        return ans
