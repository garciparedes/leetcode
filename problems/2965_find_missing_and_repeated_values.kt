class Solution {
    fun findMissingAndRepeatedValues(grid: Array<IntArray>): IntArray {
        val n = grid.size
        var values = (1..(n * n)).toMutableSet()
        var duplicated = 0
        grid.forEach { row ->
            row.forEach { value ->
                if (values.contains(value)) {
                    values.remove(value)
                } else {
                    duplicated = value
                }
            }
        }
        return listOf(duplicated, values.first()).toIntArray()
    }
}
