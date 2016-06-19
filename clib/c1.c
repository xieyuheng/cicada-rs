#include <stdio.h>
#include "../libjojo.h"

void c1() {
  printf("c1c1c1");
}

void export() {
  defprim("c1", c1);
}
