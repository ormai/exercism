#include "triangle.h"

static bool is_valid(triangle_t triangle) {
  return triangle.a + triangle.b >= triangle.c &&
         triangle.b + triangle.c >= triangle.a &&
         triangle.c + triangle.a >= triangle.b;
}

bool is_equilateral(triangle_t triangle) {
  return triangle.a > 0 && is_valid(triangle) && triangle.a == triangle.b &&
         triangle.b == triangle.c;
}

bool is_isosceles(triangle_t triangle) {
  return is_valid(triangle) &&
         (triangle.a == triangle.b || triangle.b == triangle.c ||
          triangle.c == triangle.a);
}

bool is_scalene(triangle_t triangle) {
  return is_valid(triangle) && triangle.a != triangle.b &&
         triangle.b != triangle.c && triangle.c != triangle.a;
}
