class Dna(val sequence: String) {
    init {
        require(sequence.matches(Regex("^[ACGT]*$")))
    }

    val nucleotideCounts: Map<Char, Int>
        get() = sequence.groupingBy { it }
            .eachCountTo(mutableMapOf('A' to 0, 'C' to 0, 'G' to 0, 'T' to 0))
}
