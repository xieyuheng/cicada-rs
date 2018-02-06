#include "jojo_classes.h"

typedef enum {
    STR_OBJ,
} obj_tag_t;

typedef struct {} str_obj_t;

struct _obj_t {
    obj_tag_t obj_tag;
    union {
        str_obj_t str_obj;
    };
};
