#include "resistor_color_trio.h"

#include <math.h>

resistor_value_t color_code(resistor_band_t bands[]) {
  resistor_value_t value = {(bands[0] * 10 + bands[1]) * pow(10, bands[2]),
                            OHMS};
  if (value.value >= 10E8) {
    value.value /= 10E8;
    value.unit = GIGAOHMS;
  } else if (value.value >= 10E5) {
    value.value /= 10E5;
    value.unit = MEGAOHMS;
  } else if (value.value >= 10E2) {
    value.value /= 1000;
    value.unit = KILOOHMS;
  }
  return value;
}
