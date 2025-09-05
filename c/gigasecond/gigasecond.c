#include "gigasecond.h"

#include <stdio.h>
#include <time.h>

void gigasecond(time_t input, char *output, size_t size) {
  if (size > 0) {
    input += 1000000000;
    const struct tm *time = gmtime(&input);
    strftime(output, size, "%Y-%m-%d %H:%M:%S", time);
  }
}
