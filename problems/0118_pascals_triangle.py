class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        triangle = [[1]]
        
        for i in range(1, numRows):
            
            row = [1]
            for j in range(1, i):
                row.append(triangle[-1][j - 1] + triangle[-1][j])
            row.append(1)
            
            triangle.append(row)
            
        return triangle
