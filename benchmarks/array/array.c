#include <stdio.h>

int main() {
  for (int i = 0; i < 10000; i++) {
    int x[3] = {1, 2, 3};
    x[0] = 10;
    printf("[%d, %d, %d]", x[0], x[1], x[2]);
  }
}
