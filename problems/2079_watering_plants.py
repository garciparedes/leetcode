class Solution:
    def wateringPlants(self, plants: List[int], capacity: int) -> int:
        ans = 0
        can = capacity
        for steps, plant in enumerate(plants, 1):
            if can >= plant:
                can -= plant
                ans += 1
            else:   
                can = capacity - plant
                ans += steps * 2 - 1
                
        return ans
