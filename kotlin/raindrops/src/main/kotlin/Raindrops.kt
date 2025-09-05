object Raindrops {
    fun convert(n: Int) = buildString {
      if (n % 3 == 0) append("Pling")
      if (n % 5 == 0) append("Plang")
      if (n % 7 == 0) append("Plong")
    }.ifEmpty { return n.toString() }
}
