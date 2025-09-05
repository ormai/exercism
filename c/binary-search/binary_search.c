#include "binary_search.h"

const int *binary_search(int value, const int *arr, size_t length) {
  size_t l = 0, r = length;
  while (l < r) {
    const size_t m = (l + r) / 2;
    if (arr[m] < value) {
      l = m + 1;
    } else if (arr[m] > value) {
      r = m;
    } else {
      return arr + m;
    }
  }
  return NULL;
}
