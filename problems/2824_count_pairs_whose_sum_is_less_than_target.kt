class Solution {
    fun countPairs(nums: List<Int>, target: Int): Int {
        var ans = 0
        for (i in 0 ..< nums.size) {
            for (j in i + 1 ..< nums.size) {
                if (nums[i] + nums[j] < target) {
                    ans += 1
                }
            }
        }
        return ans
    }
}
