#include "luhn.h"

#include <ctype.h>

bool luhn(const char *num) {
  int digits[128];
  int cursor = 0;
  for (int i = 0; num[i] != '\0'; ++i) {
    if (isdigit(num[i])) {
      digits[cursor++] = num[i] - '0';
    } else if (num[i] != ' ') {
      return false;
    }
  }

  int i = 1;
  while (i < cursor) {
    const int doubled = digits[cursor - i - 1] * 2;
    digits[cursor - i - 1] = doubled > 9 ? doubled - 9 : doubled;
    i += 2;
  }

  int sum = 0;
  for (int i = 0; i < cursor; ++i) {
    sum += digits[i];
  }
  return cursor > 1 && sum % 10 == 0;
}
