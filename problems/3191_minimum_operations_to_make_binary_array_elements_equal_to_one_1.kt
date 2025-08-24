class Solution {
    fun minOperations(nums: IntArray): Int {
        var ans = 0
        for (i in nums.indices) {
            if (nums[i] == 0) {
                ans += 1
                for (j in 0..<3) {
                    if (i + j >= nums.size) {
                        return -1
                    }
                    nums[i + j] = 1 - nums[i + j]
                }
            }
        }
        return ans
    }
}
