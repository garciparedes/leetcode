class Solution {
    fun decodeMessage(key: String, message: String): String {
        val mapper = createMapper(key)
        return message.map { c -> mapper.get(c) ?: ' ' }.joinToString("")
    }

    fun createMapper(key: String): Map<Char, Char> {
        val ans = mutableMapOf<Char, Char>()
        var index = 0
        key.forEach { c ->
            if (c != ' ' && !ans.contains(c)) {
                ans.put(c, ((97 + index).toChar()))
                index += 1
            }
        }
        println(ans)
        return ans
    }
}
