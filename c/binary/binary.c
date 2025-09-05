#include "binary.h"

#include <string.h>

int convert(const char *input) {
  int n = 0;

  const int len = strlen(input);
  for (int i = 0; i < len; ++i) {
    if (input[len - i - 1] == '1') {
      n += 1 << i;
    } else if (input[len - i - 1] != '0') {
      return INVALID;
    }
  }

  return n;
}
