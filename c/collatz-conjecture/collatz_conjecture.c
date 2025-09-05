#include "collatz_conjecture.h"

int steps(int start) {
  if (start < 1) {
    return ERROR_VALUE;
  }
  if (start == 1) {
    return 0;
  }
  return 1 + steps(start % 2 == 0 ? start / 2 : start * 3 + 1);
}
