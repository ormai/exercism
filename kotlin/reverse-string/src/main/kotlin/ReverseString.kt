fun reverse(input: String): String = input.myReversed()

fun String.myReversed(): String {
  return if (length <= 1) {
    this
  } else {
    Array(length) { index -> this[length - index - 1] }.joinToString("")
  }
}
