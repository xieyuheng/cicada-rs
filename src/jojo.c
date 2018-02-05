#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <stdio.h>
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
// #include <assert.h>
#include <stdlib.h>

typedef struct {} name_dict_t;
typedef struct {} data_stack_t;
typedef struct {} return_stack_t;

typedef struct {
    name_dict_t *name_dict;
    data_stack_t *data_stack;
    return_stack_t *return_stack;
} env_t;

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

typedef struct {
    ins_tag_t ins_tag;
    union {
        prim_ins_t prim_ins;
        call_ins_t call_ins;
        get_ins_t get_ins;
        set_ins_t set_ins;
    };
} ins_t;

typedef struct {
    size_t index;
    size_t length;
    ins_t *body;
} frame_t;

frame_t *
return_stack_tos (return_stack_t *return_stack) {

}

void
ins_exe (ins_t ins, env_t *env)
{
    return;
}

void
next (env_t *env)
{
    frame_t *frame = return_stack_tos (env->return_stack);
    ins_t ins = frame->body[frame->index];
    frame->index = frame->index + 1;
    ins_exe (ins, env);
}

typedef enum {
    STR_OBJ,
} obj_tag_t;

typedef struct {} str_obj_t;

typedef struct {
    obj_tag_t obj_tag;
    union {
        str_obj_t str_obj;
    };
} obj_t;

int
main (void) {

}
