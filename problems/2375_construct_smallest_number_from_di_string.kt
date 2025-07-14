class Solution {
    fun smallestNumber(pattern: String): String {
        val available = MutableList(9) { true }
        return dfs(available, "S$pattern", "") ?: "ERROR"
    }

    fun dfs(available: MutableList<Boolean>, pattern: String, current: String): String? {
        if (pattern.isBlank()) {
            return current
        }
        available.forEachIndexed { index, value ->
            if (value) {
                available[index] = false
                val action = pattern.first ()
                val next = (index + 1).digitToChar()
                if (action == 'S' || action == 'I' && current.last() < next || action == 'D' && current.last() > next) {
                    dfs(available, pattern.substring(1), "$current$next")?.let { return it }
                }
                available[index] = true
            }
        }
        return null
    }
}
