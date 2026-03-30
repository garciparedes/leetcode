class Solution:
    def findDifferentBinaryString(self, nums: List[str]) -> str:
        n = len(nums[0])
        unique = set(nums)
        for case in self.binary_generator(n):
            if (case not in unique):
                return case
        return ""

    def binary_generator(self, n: int) -> Generator[str]:
        if (n <= 0):
            yield ""
            return

        for case in self.binary_generator(n - 1):
            yield "0" + case
            yield "1" + case
