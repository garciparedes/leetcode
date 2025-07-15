class Solution {
    fun subarraySum(nums: IntArray): Int {
        var ans = 0
        nums.forEachIndexed { i, num ->
            val start = max(0, i - num)
            for (j in start..i) {
                ans += nums[j] ?: 0
            }
        }
        return ans
    }
}
