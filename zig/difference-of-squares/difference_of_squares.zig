pub fn squareOfSum(number: usize) usize {
    const sum = number * (number + 1) / 2;
    return sum * sum;
}

pub fn sumOfSquares(number: usize) usize {
    return (2 * number + 1) * (number + 1) * number / 6;
}

pub fn differenceOfSquares(number: usize) usize {
    return squareOfSum(number) - sumOfSquares(number);
}
