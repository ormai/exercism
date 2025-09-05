#include "eliuds_eggs.h"

int egg_count(int egg) {
  int count = 0;
  while (egg > 0) {
    count += egg & 1;
    egg >>= 1;
  }
  return count;
}
