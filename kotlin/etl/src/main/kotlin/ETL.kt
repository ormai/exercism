object ETL {
    fun transform(source: Map<Int, Collection<Char>>): Map<Char, Int> {
        val map = HashMap<Char, Int>()
        for ((score, letters) in source) {
            for (letter in letters) {
                map[letter.lowercaseChar()] = score
            }
        }
        return map
    }
}
