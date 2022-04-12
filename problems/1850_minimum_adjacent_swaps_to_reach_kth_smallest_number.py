from itertools import permutations


class Solution:
    def getMinSwaps(self, num: str, k: int) -> int:
        num = list(num)
        
        smallest = num.copy()
        for _ in range(k):
            self.next_smallest(smallest)
                        
        return self.minimum_swaps(num, smallest)
    
    def next_smallest(self, num: list[str]) -> None:
        n = len(num)
        i = n - 2
        while i >= 0 and num[i] >= num[i + 1]:
            i -= 1
        
        if i == -1:
            return
        
        j = i + 1
        while j < n and num[i] < num[j]:
            j += 1
            
        j -= 1
        
        num[i], num[j] = num[j], num[i]
        
        num[i+1:] = num[i+1:][::-1]

        
        
    def minimum_swaps(self, a: list[str], b: list[str]) -> int:
        ans = 0
        for i in range(len(a)):
            j = i
            while a[j] != b[i]:
                j += 1
                
            while i < j:
                a[j], a[j - 1] = a[j - 1], a[j]
                j -= 1
                ans += 1
        return ans        
        
