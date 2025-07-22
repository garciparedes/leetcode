class Solution {
    fun sumOfSquares(nums: IntArray): Int {
        var ans = 0
        nums.forEachIndexed { i, num ->
            if (nums.size % (i + 1) == 0) {
                ans += num * num
            }
        }
        return ans
    }
}
