class env_t
{
    constructor (name_dict,
                 data_stack,
                 return_stack,
                 scope_stack)
    {
        this.name_dict = name_dict;
        this.data_stack = data_stack;
        this.return_stack = return_stack;
        this.scope_stack = scope_stack;
    }
}

// env_t
function new_env ()
{
    let env = new env_t (new Map (),
                         [],
                         [],
                         []);
    return env;
}

class call_exp_t
{
    constructor (name)
    {
        this.name = name;
    }
}

class get_local_exp_t
{
    constructor (local_name)
    {
        this.local_name = local_name;
    }
}

class set_local_exp_t
{
    constructor (local_name)
    {
        this.local_name = local_name;
    }
}

class clo_exp_t
{
    constructor (exp_list)
    {
        this.exp_list = exp_list;
    }
}

class arrow_exp_t
{
    constructor (ante, succ)
    {
        this.ante = ante;
        this.succ = succ;
    }
}

class case_exp_t
{
    constructor (arg, clause_dict)
    {
        this.arg = arg;
        this.clause_dict = clause_dict;
    }
}

class create_exp_t
{
   constructor ()
   {

   }
}

class clone_exp_t
{
   constructor ()
   {

   }
}

class apply_exp_t
{ }

function list_eval (env, exp_list)
{

}

function eval_with_base (env, base)
{

}

function eval_one_step (env)
{
    // frame_t *frame = return_stack_tos (env->return_stack);
    // ins_u ins = frame->body[frame->index];
    // frame->index = frame->index + 1;
    // ins_exe (ins, env);
}

// obj_list
function collect_obj_list (env, exp_list)
{
}

// obj_list
function collect_obj (env, exp)
{
}

class union_den_t
{
    constructor (name, type_arrow, union_list)
    {
        this.name = name;
        this.type_arrow = type_arrow;
        this.union_list = union_list;
    }
}

class type_den_t
{
    constructor (name, type_arrow, cons_arrow)
    {
        this.name = name;
        this.type_arrow = type_arrow;
        this.cons_arrow = cons_arrow;
    }
}

class fun_den_t
{
    constructor (name, type_arrow, exp_list)
    {
        this.name = name;
        this.type_arrow = type_arrow;
        this.exp_list = exp_list;
    }
}

class gene_den_t
{
    constructor (name, type_arrow, exp_list)
    {
        this.name = name;
        this.type_arrow = type_arrow;
        this.exp_list = exp_list;
    }
}

class disp_den_t
{
    constructor (name, type_arrow, exp_list)
    {
        this.name = name;
        this.type_arrow = type_arrow;
        this.exp_list = exp_list;
    }
}

class data_obj_t
{
    constructor ()
    {
        this.type_name = type_name;
        this.fields = fields;
    }
}

class clo_obj_t
{
    constructor (exp_list, locals)
    {
        this.exp_list = exp_list;
        this.locals = locals;
    }
}
