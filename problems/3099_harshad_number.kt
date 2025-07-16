class Solution {
    fun sumOfTheDigitsOfHarshadNumber(x: Int): Int {
        val s = sumDigits(x)
        return if (x % s == 0) { s } else { -1 }
    }

    fun sumDigits(x: Int): Int {
        var ans = 0
        var current = x
        while (current > 0) {
            ans += current % 10
            current /= 10
        }
        return ans
    }
}
