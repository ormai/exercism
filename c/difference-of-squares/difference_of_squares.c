#include "difference_of_squares.h"

unsigned sum_of_squares(unsigned number) {
  return (2 * number + 1) * (number + 1) * number / 6;
}

unsigned square_of_sum(unsigned number) {
  // Gauss's formula
  const unsigned sum = number * (number + 1) / 2;
  return sum * sum;
}

unsigned difference_of_squares(unsigned number) {
  return square_of_sum(number) - sum_of_squares(number);
}
