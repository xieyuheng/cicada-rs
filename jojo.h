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
#include <stdarg.h>

typedef enum { false, true } bool;
typedef intptr_t cell;
typedef void (* primitive_t)();

struct jotable_entry {
  char *key;
  struct jotable_entry *tag;
  cell data;
};

typedef struct jotable_entry* jo_t;

// prime table size
//   1000003   about 976 k
//   1000033
//   1000333
//   100003    about 97 k
//   100333
//   997
#define JOTABLE_SIZE 100003
struct jotable_entry jotable[JOTABLE_SIZE];

// thus (jotable + index) is jo

extern void report(char* format, ...);
extern void p_debug();

struct dp {
  jo_t t;
  cell d;
};

typedef enum {
  GC_MARK_FREE,
  GC_MARK_USING,
} gc_mark_t;

struct gp { // gc point
  gc_mark_t mark;
  struct class* class;
  cell p; // actual data point
};

extern void ds_push(jo_t tag, cell data);
extern struct dp ds_pop();

extern void plus_atom(char* class_name, gc_actor_t gc_actor);
extern void plus_prim(char* function_name, primitive_t fun);
