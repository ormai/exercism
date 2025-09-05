#include "nucleotide_count.h"

#include <stdlib.h>
#include <stdio.h>

char *count(const char *dna_strand) {
  char *fmt = malloc(32);
  int a = 0, c = 0, g = 0, t = 0;
  for (int i = 0; dna_strand[i] != '\0'; ++i) {
    switch (dna_strand[i]) {
    case 'A':
      ++a;
      break;
    case 'C':
      ++c;
      break;
    case 'G':
      ++g;
      break;
    case 'T':
      ++t;
      break;
    default:
      fmt[0] = '\0';
      return fmt;
    }
  }
  sprintf(fmt, "A:%d C:%d G:%d T:%d", a, c, g, t);
  return fmt;
}
