#include <stdio.h>

int main() {
  int a = 1, b = 0, c = 0, d = 0, e = 0, f = 0, g = 0, h = 0;
  b = 99;
  c = b;
  if (a != 0) {
    b = b * 100 + 100000;
    c = b + 17000;
  }
  do {
    f = 1;
    d = 2;
    e = 2;
    for (d = 2; d * d <= b; d++) { // check if b is a prime
      // the assembly doesn't have a % operator,
      // so it does 2 for loops with d and e and checks if d*e==b.
      if ((b % d == 0)) {
        f = 0;
        break;
      }
    }
    if (f == 0) // not a prime
      h++;
    g = b - c;
    b += 17;
  } while (g != 0); // stop when b==c (1000 iterations)

  printf("%d\n", h);
  return 0;
}