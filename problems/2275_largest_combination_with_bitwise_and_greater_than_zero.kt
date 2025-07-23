class Solution {
    fun largestCombination(candidates: IntArray): Int {
        val counter = MutableList(24) { 0 }
        candidates.forEach { candidate ->
            (0..<24).forEach { index ->
                if ((candidate and (1 shl index)) != 0) {
                    counter[index] += 1
                }
            }
        }
        return counter.max()
    }
}
