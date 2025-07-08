class Solution {
    fun minimumOperations(nums: IntArray): Int {
        var acc = 0
        nums.forEach { num ->
            if (num % 3 != 0) {
                acc += 1
            }
        }
        return acc
    }
}
