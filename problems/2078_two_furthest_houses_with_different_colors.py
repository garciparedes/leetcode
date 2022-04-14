class Solution:
    def maxDistance(self, colors: List[int]) -> int:
        ranges = dict()
        
        for i, color in enumerate(colors):
            if color in ranges:
                ranges[color][1] = i
            else:
                ranges[color] = [i, i]
                
        min_1 = "", len(colors)
        for k, v in ranges.items():
            if v[0] < min_1[1]:
                min_1 = k, v[0]
        
        max_1 = "", -1
        for k, v in ranges.items():
            if k != min_1[0] and v[1] > max_1[1]:
                max_1 = k, v[1]
        
        min_2 = "", len(colors)
        for k, v in ranges.items():
            if k != min_1[0] and v[0] < min_2[1]:
                min_2 = k, v[0]
                
        max_2 = "", -1
        for k, v in ranges.items():
            if  k != min_2[0] and v[1] > max_2[1]:
                max_2 = k, v[1]
    
        return max(max_1[1] - min_1[1], max_2[1] - min_2[1])
        
