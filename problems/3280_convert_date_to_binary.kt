class Solution {
    fun convertDateToBinary(date: String) = date
        .split("-")
        .map { it.toInt().toString(2) }
        .joinToString("-")
}
