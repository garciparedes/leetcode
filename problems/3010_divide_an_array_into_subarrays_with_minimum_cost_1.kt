class Solution {
    fun minimumCost(nums: IntArray): Int {
        val first = nums[0]
        val sorted = nums.drop(1).sorted()
        return first + sorted[0] + sorted[1]
    }
}
