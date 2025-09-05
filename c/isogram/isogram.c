#include "isogram.h"

#include <ctype.h>
#include <stdint.h>

bool is_isogram(const char phrase[]) {
  if (!phrase) {
    return false;
  }
  uint32_t set = 0;
  for (int i = 0; phrase[i] != '\0'; ++i) {
    if (isalpha(phrase[i])) {
      const uint32_t bit = 1 << (tolower(phrase[i]) - 'a');
      if ((set & bit) == bit) {
        return false;
      }
      set |= bit;
    }
  }
  return true;
}
