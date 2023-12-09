#include <stdio.h>

int main() {
  char c = 0;
  unsigned int first_digit = 0;
  unsigned int last_digit = 0;
  unsigned int sum = 0;
  while (c != EOF) {
    c = getchar();
    // putchar(c);
    unsigned char possible_digit = c - 48;
    if (possible_digit < 10) {
      //   printf("%d\n", possible_digit);
      if (first_digit == 0) {
        first_digit = possible_digit * 10;
      }
      last_digit = possible_digit;
    } else if (c == 10) {
      sum += first_digit + last_digit;
      first_digit = 0;
      last_digit = 0;
    }
  }
  printf("%d", sum);
}