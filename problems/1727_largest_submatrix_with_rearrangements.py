class Solution:
    def largestSubmatrix(self, matrix: List[List[int]]) -> int:
        for j in range(len(matrix[0])):
            current = 0
            for i in range(len(matrix)):
                if (matrix[i][j]):
                    current +=1
                else:
                    current = 0
                matrix[i][j] = current

        for i in range(len(matrix)):
            matrix[i].sort(reverse=True)

        ans = 0
        for i in range(len(matrix)):
            for j in range(len(matrix[i])):
                ans = max(ans, matrix[i][j] * (j + 1))
        return ans
