#include "hamming.h"

int compute(const char *lhs, const char *rhs) {
  int sum = 0;

  int i = 0;
  for (; lhs[i] != '\0'; ++i) {
    sum += lhs[i] != rhs[i];
  }
  if (rhs[i] != '\0') {
    return -1;
  }

  return sum;
}
