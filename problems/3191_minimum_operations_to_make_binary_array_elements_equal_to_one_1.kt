class Solution {
    fun separateDigits(nums: IntArray): IntArray {
        val ans = mutableListOf<Int>()
        nums.reversed().forEach { num ->
            var current = num
            while (current > 0) {
                ans.add(current % 10)
                current /= 10
            }
        }
        return ans.reversed().toIntArray()
    }
}
