class Solution {
    fun numberGame(nums: IntArray): IntArray {
        nums.sort()
        val ans = mutableListOf<Int>()
        for (i in 0..<(nums.size / 2)) {
            ans.add(nums[2 * i + 1])
            ans.add(nums[2 * i])
        }
        return ans.toIntArray()
    }
}
