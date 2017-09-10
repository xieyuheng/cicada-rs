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

extern void plus_atom(char* str, primitive fun);
extern void plus_prim(char* str, primitive fun);
