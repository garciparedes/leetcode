class Solution {
    fun totalWaviness(num1: Int, num2: Int): Int {
        val start = maxOf(num1, 100)
        val end = maxOf(start, num2)

        var ans = 0
        for (num in start..end) {
            val digits = numToDigits(num)
            ans += findPeaksAndValleys(digits)
        }
        return ans
    }

    fun numToDigits(num: Int): List<Int> {
        var curr = num
        val ans = mutableListOf<Int>()
        while (curr > 0) {
            ans.add(curr % 10)
            curr /= 10
        }
        return ans
    }

    fun findPeaksAndValleys(digits: List<Int>): Int {
        var ans = 0
        for (i in 1..(digits.size - 2)) {
            if ((digits[i - 1] < digits[i] && digits[i] > digits[i + 1]) 
                || (digits[i - 1] > digits[i] && digits[i] < digits[i + 1])) {
                ans += 1
            }
        }
        return ans
    }
}
