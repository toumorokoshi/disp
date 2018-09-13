#include <stdio.h>

int fib(int n) {
  return n > 2 ? fib(n - 1) + fib(n - 2) : n;
}

int main() {
  printf("%d\n", fib(30));
}
