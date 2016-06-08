#include <stdio.h>
#include "../jojo.h"

void k2() {
  as_push(222);
  printf("k2k2k2");
}

// string lib_file = "./play/libk2.so";
// string func_to_run = "k2";
// void* lib = dlopen(lib_file, RTLD_LAZY);
// if (lib == NULL) {
//   printf("Failed to open the library %s: %s\n",
//          lib_file, dlerror());
// };

// primitive func = dlsym(lib, func_to_run);
// if (func == NULL) {
//   printf("Did not find %s function in the library %s: %s\n",
//          func_to_run, lib_file, dlerror());
// };

// func();

// cell rc = dlclose(lib);
// if (rc != 0) {
//   printf("Failed to close %s\n", lib_file);
// };
