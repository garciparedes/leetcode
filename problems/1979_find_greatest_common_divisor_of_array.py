class Solution:
    def findGCD(self, nums: List[int]) -> int:
        greatest = max(nums)
        smallest = min(nums)
        
        return self.gcd(greatest, smallest)
    
    @staticmethod
    def gcd(a, b):
        while b > 0:
            tmp = a % b
            a = b
            b = tmp
        return a
