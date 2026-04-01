class Solution:
    def minAbsDiff(self, grid: List[List[int]], k: int) -> List[List[int]]:
        n = len(grid)
        m = len(grid[0])
        
        ans = list()

        for i in range(n - k + 1):
            ans_row = list()
            for j in range(m - k + 1):
                
                values_set = set()
                for ii in range(i, i + k):
                    for jj in range(j, j + k):
                        values_set.add(grid[ii][jj])

                if len(values_set) == 1:
                    ans_row.append(0)
                else:
                    values = sorted(values_set)
                    minimum = values[1] - values[0]
                    for l in range(1, len(values) -1):
                        minimum = min(minimum, values[l + 1] - values[l])
                    ans_row.append(minimum)
            ans.append(ans_row)

        return ans
