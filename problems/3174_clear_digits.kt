class Solution {
    fun clearDigits(s: String): String {
        val removed = mutableSetOf<Int>()
        s.forEachIndexed { i, c ->
            if (c.isDigit()) {
                var maybeJ: Int? = null
                for (j in (i - 1) downTo 0) {
                    if (!removed.contains(j) && !s[j].isDigit()) {
                        jj = j
                        break
                    }
                }
                maybeJ?.let { j ->
                    removed.add(j)
                    removed.add(i)
                }
            }
        }
        val ans = StringBuilder()
        s.forEachIndexed { i, c ->
            if (!removed.contains(i)) {
                ans.append(c)
            }
        }
        return ans.toString()
    }
}
