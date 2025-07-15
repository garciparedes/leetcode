class Solution {
    fun garbageCollection(garbage: Array<String>, travel: IntArray): Int {
        val positions = mutableListOf(0, 0, 0)
        var ans = 0
        garbage.forEachIndexed { index, home ->
            home.forEach { g ->
                ans += 1
                val positionIndex = when(g) {
                    'G' -> 0
                    'P' -> 1
                    else -> 2
                }
                while (positions[positionIndex] < index) {
                    ans += travel[positions[positionIndex]]
                    positions[positionIndex] += 1
                }
            }

        }
        return ans
    }
}
