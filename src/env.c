#include "jojo_classes.h"

//  Structure of our class

typedef struct {} name_dict_t;
typedef struct {} data_stack_t;
typedef struct {} return_stack_t;

struct _env_t {
    name_dict_t *name_dict;
    data_stack_t *data_stack;
    return_stack_t *return_stack;
};

typedef struct {
    size_t index;
    size_t length;
    ins_t **body;
} frame_t;


frame_t *
return_stack_tos (return_stack_t *return_stack)
{

}


void
ins_exe (ins_t *ins, env_t *env)
{
    return;
}


void
next (env_t *env)
{
    frame_t *frame = return_stack_tos (env->return_stack);
    ins_t *ins = frame->body[frame->index];
    frame->index = frame->index + 1;
    ins_exe (ins, env);
}
