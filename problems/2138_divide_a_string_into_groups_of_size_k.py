class Solution:
    def divideString(self, s: str, k: int, fill: str) -> List[str]:
        ans = list()
        for i in range(0, len(s), k):
            tmp = s[i:i+k]
            while len(tmp) < k:
                tmp += fill
            ans.append(tmp)
        return ans
