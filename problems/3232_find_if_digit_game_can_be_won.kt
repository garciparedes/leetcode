class Solution {
    fun canAliceWin(nums: IntArray): Boolean {
        var single = 0
        var double = 0
        nums.forEach { num ->
            if (num > 9) {
                single += num
            } else {
                double += num
            }
        }
        return single != double
    }
}
