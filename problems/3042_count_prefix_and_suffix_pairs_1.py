class Solution:
    def countPrefixSuffixPairs(self, words: List[str]) -> int:
        ans = 0
        for i in range(len(words) - 1):
            str1 = words[i]
            for j in range(i + 1, len(words)):
                str2 = words[j]
                if self.isPrefixAndSuffix(str1, str2):
                    ans += 1
        return ans

    def isPrefixAndSuffix(self, str1: str, str2: str) -> bool:
        return str2.startswith(str1) and str2.endswith(str1)
