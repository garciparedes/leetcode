class Solution {
    fun isBalanced(num: String): Boolean {
        var odd = 0
        var even = 0
        num.forEachIndexed { index, digit ->
            if (index % 2 == 0) {
                even += digit.digitToInt()
            } else {
                odd += digit.digitToInt()
            }
        }
        return odd == even
    }
}
