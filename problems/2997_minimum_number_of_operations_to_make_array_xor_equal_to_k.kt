class Solution {
    fun minOperations(nums: IntArray, k: Int): Int {
        val xored = nums.reduce { acc, num -> acc xor num}
        return countOnes(xored xor k)
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
