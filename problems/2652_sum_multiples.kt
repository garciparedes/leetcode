class Solution {
    fun sumOfMultiples(n: Int): Int {
        var ans = 0
        for (i in 1..n) {
            if (i % 3 == 0 || i % 5 == 0 || i % 7 == 0) {
                ans += i
            }
        }
        return ans
    }
}
