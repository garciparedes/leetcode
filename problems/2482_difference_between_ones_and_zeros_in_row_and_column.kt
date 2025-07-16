class Solution {
    fun onesMinusZeros(grid: Array<IntArray>): Array<IntArray> {
        val n = grid.size
        val m = grid[0].size
        val rows = (0..<n).map {
                i -> grid[i].sum()
        }
        val columns = (0..<m).map { j ->
            (0..<n).fold(0) { acc, i ->
                acc + grid[i][j]
            }
        }


        return (0..<n).map { i ->
            (0..<m).map { j ->
                rows[i] + columns[j] -  (n - rows[i]) - (m - columns[j])
            }.toIntArray()
        }.toTypedArray()
    }
}
