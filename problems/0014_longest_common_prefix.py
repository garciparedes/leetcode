class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        ans = strs[0]
        for word in strs[1:]:
            i = 0
            while i < min(len(ans), len(word)) and ans[i] == word[i]:
                i += 1
            ans = ans[:i]
        return ans
