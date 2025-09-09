object Bob {
    fun hey(input: String): String {
        val isShouting = { input.matches(Regex(".*[A-Z]+.*")) && input.matches(Regex("^[^a-z]*$")) }
        return if (input.trimEnd().endsWith("?")) {
            if (isShouting()) {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure."
            }
        } else if (isShouting()) {
            "Whoa, chill out!"
        } else if (input.matches(Regex("^\\s*$"))) {
            "Fine. Be that way!"
        } else {
            "Whatever."
        }
    }
}
