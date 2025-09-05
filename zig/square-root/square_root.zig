pub fn squareRoot(radicand: usize) usize {
    var d: usize = 1 << 30;
    while (d > radicand) : (d >>= 2) {}

    var x = radicand;
    var c: usize = 0;
    while (d != 0) : (d >>= 2) {
        if (x >= c + d) {
            x -= c + d;
            c = (c >> 1) + d;
        } else {
            c >>= 1;
        }
    }
    return c;
}
