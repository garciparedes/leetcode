class Solution {
    fun minCosts(cost: IntArray): IntArray {
        var minimum = Int.MAX_VALUE
        return cost.map { c ->
            minimum = min(minimum, c)
            minimum
        }.toIntArray()
    }
}
