#include "armstrong_numbers.h"

#include <math.h>

bool is_armstrong_number(int candidate) {
  int digits[64];
  int i = 0;
  int n = candidate;
  while (n > 0) {
    digits[i] = n % 10;
    n /= 10;
    i += 1;
  }

  int sum = 0;
  for (int j = 0; j < i; ++j) {
    sum += pow(digits[j], i);
  }
  return sum == candidate;
}
