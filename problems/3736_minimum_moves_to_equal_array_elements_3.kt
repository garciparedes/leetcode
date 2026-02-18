class Solution {
    fun minMoves(nums: IntArray): Int {
        val maximum = nums.max()
        return nums.fold(0) { acc, curr -> acc + maximum - curr }
    }
}
