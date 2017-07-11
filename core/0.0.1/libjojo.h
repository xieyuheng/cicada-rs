#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <fcntl.h>
#include <ctype.h>
#include <stdint.h>
#include <dlfcn.h>
#include <dirent.h>
#include <signal.h>
#include <limits.h>

typedef intptr_t cell;
typedef void (*primitive)();

typedef struct jo_entry {
  char *key;
  struct jo_entry *tag;
  cell value;
} jotable_entry;

typedef jotable_entry *jo;

extern void as_push(cell value);
extern void as_pop();

extern void define_prim(char* str, primitive fun);
extern void define_primkey(char* str, primitive fun);
