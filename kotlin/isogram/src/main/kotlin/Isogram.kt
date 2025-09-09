object Isogram {
    fun isIsogram(input: String) = input.lowercase()
        .filter(Char::isLetter)
        .let { it.length == it.toSet().size }
}
