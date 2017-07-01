#include <stdio.h>
#include "libjojo.h"

void p_welcome() {
  printf("  welcome to jojo ^-^/\n");
}

void export() {
  define_prim("welcome", p_welcome);
}
