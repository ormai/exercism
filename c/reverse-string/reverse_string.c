#include "reverse_string.h"

#include <stdlib.h>
#include <string.h>

char *reverse(const char *value) {
  const size_t len = strlen(value);
  char *reversed = malloc(len + 1);
  for (size_t i = 0; i < len; ++i) {
    reversed[i] = value[len - i - 1];
  }
  reversed[len] = '\0';
  return reversed;
}
