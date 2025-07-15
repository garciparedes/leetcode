class Solution {
    fun arithmeticTriplets(nums: IntArray, diff: Int): Int {
        var ans = 0
        for(i in 0..<(nums.size - 2)) {
            for(j in (i + 1)..<(nums.size - 1)) {
                if (nums[j] - nums[i] == diff) {
                    for(k in (j + 1)..<nums.size) {
                        if (nums[k] - nums[j] == diff) {
                            ans += 1
                        }
                    }
                }

            }
        }
        return ans
    }
}
