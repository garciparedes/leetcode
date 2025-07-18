class Solution {
    fun punishmentNumber(n: Int): Int {
        var ans = 0
        (1..n).forEach { num ->
            if (isPunishmentNumber(num)) {
                ans += num * num
            }
        }
        return ans
    }

    fun isPunishmentNumber(num: Int): Boolean {
        return rec(num * num, 0, 1, 0, num);
    }

    fun rec(remaining: Int, current: Int, position: Int, cumulated: Int, num: Int): Boolean {
        if (remaining + current + cumulated == num) {
            return true
        }
        if (remaining == 0) {
            return false
        }
        val newRemaining = remaining / 10
        val newCurrent = (remaining % 10) * position

        return rec(newRemaining, newCurrent + current, position * 10, cumulated, num)
            || rec(newRemaining, newCurrent, 1, current + cumulated, num)
    }
}
