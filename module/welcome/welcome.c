#include <stdio.h>
#include "libjojo.h"

void welcome() {
  printf("  welcome to jojo ^-^/\n");
}

void export() {
  defprim("welcome", welcome);
}
