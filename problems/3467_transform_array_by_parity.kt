class Solution {
    fun transformArray(nums: IntArray): IntArray {
        var zeros = 0
        var ones = 0
        nums.forEach {
            if (it % 2 == 0) {
                zeros += 1
            } else {
                ones += 1
            }
        }
        return (List(zeros) { 0 } + List(ones) { 1 }).toIntArray()
    }
}
