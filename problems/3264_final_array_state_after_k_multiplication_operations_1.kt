class Solution {
    fun getFinalState(nums: IntArray, k: Int, multiplier: Int): IntArray {
        repeat(k) {
            val index = nums.withIndex().minBy { (_, num) -> num }?.index!!
            nums[index] = nums[index]!! * multiplier
        }
        return nums
    }
}
