class Solution {
    fun validStrings(n: Int): List<String> {
        val ans = mutableListOf<String>()

        generate(ans, "", n)

        return ans
    }

    fun generate(ans: MutableList<String>, current: String, n: Int) {
        if (n == 0) {
            ans.add(current)
        } else {
            if (current.length == 0 || current.last() != '0') {
                generate(ans, current + '0', n - 1)
            }
            generate(ans, current + '1', n - 1)
        }
    }
}
