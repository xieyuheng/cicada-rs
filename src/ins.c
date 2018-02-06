#include "jojo_classes.h"

typedef enum {
    PRIM_INS,
    CALL_INS,
    GET_INS,
    SET_INS,
} ins_tag_t;

typedef struct {} prim_ins_t;
typedef struct {} call_ins_t;
typedef struct {} get_ins_t;
typedef struct {} set_ins_t;

struct _ins_t {
    ins_tag_t ins_tag;
    union {
        prim_ins_t prim_ins;
        call_ins_t call_ins;
        get_ins_t get_ins;
        set_ins_t set_ins;
    };
};
