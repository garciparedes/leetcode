class Solution {
    fun sumIndicesWithKSetBits(nums: List<Int>, k: Int): Int {
        var ans = 0
        nums.forEachIndexed { index, num ->
            if (countOnes(index) == k) {
                ans += num
            }
        }
        return ans
    }

    fun countOnes(num: Int): Int {
        var ans = 0
        var curr = num
        while (curr > 0) {
            ans += curr % 2
            curr /= 2
        }
        return ans
    }
}
