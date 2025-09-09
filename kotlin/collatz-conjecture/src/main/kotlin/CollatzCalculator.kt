object CollatzCalculator {
    fun computeStepCount(n: Int): Int {
        require(n > 0)
        return if (n != 1) {
            1 + computeStepCount(if (n % 2 == 0) n / 2 else n * 3 + 1)
        } else {
            0
        }
    }
}
