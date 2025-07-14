class Solution {
    fun stableMountains(height: IntArray, threshold: Int): List<Int> {
        val ans = mutableListOf<Int>()
        for (i in 0..<height.size - 1) {
            if ((height[i] ?: 0) > threshold) {
                ans.add(i + 1)
            }
        }
        return ans
    }
}
