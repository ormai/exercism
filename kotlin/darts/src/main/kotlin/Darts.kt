import kotlin.math.hypot

object Darts {
    fun score(x: Number, y: Number): Int {
        val d = hypot(x.toDouble(), y.toDouble())
        return if (d <= 1) 10 else if (d <= 5) 5 else if (d <= 10) 1 else 0
    }
}
