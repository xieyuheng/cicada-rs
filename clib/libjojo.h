#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>
#include <stdint.h>
#include <setjmp.h>
#include <string.h>
#include <dlfcn.h>
#include <unistd.h>

typedef intptr_t cell;
typedef cell jo;

extern void as_push(cell value) ;
extern cell as_pop() ;
