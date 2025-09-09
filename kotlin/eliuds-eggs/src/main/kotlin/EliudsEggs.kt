object EliudsEggs {
    fun eggCount(number: Int) = number.myCountOneBits()

    fun Int.myCountOneBits(): Int = (0..Int.SIZE_BITS).reduce { acc, bit ->
        acc + if (and(1.shl(bit)) == 0) 0 else 1
    }
}
