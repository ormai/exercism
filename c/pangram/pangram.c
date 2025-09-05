#include "pangram.h"

#include <ctype.h>
#include <stdint.h>

bool is_pangram(const char *sentence) {
  if (!sentence) {
    return false;
  }
  uint32_t set = 0;
  for (int i = 0; sentence[i] != '\0'; ++i) {
    if (isalpha(sentence[i])) {
      set |= 1 << (tolower(sentence[i]) - 'a');
    }
  }
  return set >= 0x3FFFFFF;
}
