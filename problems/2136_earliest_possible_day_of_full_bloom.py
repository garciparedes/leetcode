class Solution:
    def earliestFullBloom(self, plantTime: List[int], growTime: List[int]) -> int:
        n = len(plantTime)
        times = list()
        for i in range(n):
            times.append((plantTime[i], growTime[i]))
        
        total = 0
        current = 0
        times.sort(key=lambda x: (-x[1], x[0]))
        for i in range(n):
            total = max(total, current + times[i][0] + times[i][1])
            current += times[i][0]
            
        return total
