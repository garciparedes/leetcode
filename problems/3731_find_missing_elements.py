class Solution:
    def findMissingElements(self, nums: List[int]) -> List[int]:
        values = set(range(min(nums), max(nums)))
        for num in nums:
            values.discard(num)
        return sorted(values)
