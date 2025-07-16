class Solution {
    fun minElement(nums: IntArray): Int = nums.fold(Int.MAX_VALUE) { acc, num -> min(acc, sumDigits(num)) }

    fun sumDigits(num: Int): Int {
        var ans = 0
        var current = num
        while(current > 0) {
            ans += current % 10
            current /= 10
        }
        return ans
    }
}
