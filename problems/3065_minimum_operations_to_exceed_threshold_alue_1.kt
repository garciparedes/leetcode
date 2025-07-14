class Solution {
    fun minOperations(nums: IntArray, k: Int): Int {
        var ans = 0
        nums.forEach { num ->
            if (num < k) {
                ans += 1
            }
        }
        return ans
    }
}
