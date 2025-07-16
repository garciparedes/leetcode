class Solution {
    fun minimumAverage(nums: IntArray): Double {
        nums.sort()
        var ans = Double.MAX_VALUE
        for (i in 0..(nums.size / 2)) {
            ans = min(ans, (nums[i] + nums[nums.size - i - 1]).toDouble() / 2)
        }
        return ans
    }
}
