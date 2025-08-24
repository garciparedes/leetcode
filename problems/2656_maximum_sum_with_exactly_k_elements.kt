class Solution {
    fun maximizeSum(nums: IntArray, k: Int): Int = nums.max() * k + k * (k - 1) / 2
}
