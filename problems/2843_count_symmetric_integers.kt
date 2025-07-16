class Solution {
    fun countSymmetricIntegers(low: Int, high: Int): Int = (low..high).fold(0) { acc, num ->
        acc + if (areSymmetric(extractDigits(num))) { 1 } else { 0 }
    }


    fun extractDigits(num: Int): List<Int> {
        val ans = mutableListOf<Int>()
        var current = num
        while (current > 0) {
            ans.add(current % 10)
            current /= 10
        }
        return ans
    }

    fun areSymmetric(digits: List<Int>): Boolean {
        if (digits.size % 2 == 1) {
            return false
        }

        var left = 0
        var right = 0
        for (i in 0..<(digits.size / 2)) {
            left += digits[i]
            right += digits[digits.size / 2 + i]
        }

        return left == right
    }
}
