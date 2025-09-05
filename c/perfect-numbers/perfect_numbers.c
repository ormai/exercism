#include "perfect_numbers.h"

kind classify_number(int n) {
  if (n <= 0) {
    return ERROR;
  }

  int sum = 0;
  for (int f = 1; f < n; ++f) {
    if (n % f == 0) {
      sum += f;
    }
  }

  if (sum == n) {
    return PERFECT_NUMBER;
  }
  if (n < sum) {
    return ABUNDANT_NUMBER;
  }
  return DEFICIENT_NUMBER;
}
