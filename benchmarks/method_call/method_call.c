#include <stdio.h>


int doMath(int a, int b) {
    return a + b;
}

int main() {
  for (int i = 0; i < 10000; i++) {
    printf("%d", doMath(1, 2));
  }
}
