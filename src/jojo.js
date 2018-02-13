class env_t
{
    constructor ()
    {
        this.name_dict = new name_dict_t ();
        this.data_stack = [];
        this.frame_stack = [];
        this.scope_stack = [];
    }
}

class name_dict_t
{
    constructor ()
    {
        this.dict = new Map ();
    }

    get (name)
    {
        return this.dict.get(name);
    }

    set (name, den)
    {
        this.dict.set(name, den);
    }
}

function name_dict_get (env, name)
{
    return env.name_dict.get (name);
}

function name_dict_set (env, name, den)
{
    env.name_dict.set (name, den);
}

function data_stack_push (env, obj)
{
    env.data_stack.push (obj);
}

function data_stack_pop (env)
{
    return env.data_stack.pop ();
}

function data_stack_tos (env)
{
     let length = data_stack_length (env);
     return env.data_stack[length - 1];
}

function data_stack_drop (env)
{
    data_stack_pop (env);
}

function data_stack_length (env)
{
    return env.data_stack.length;
}

function frame_stack_push (env, frame)
{
    env.frame_stack.push (frame);
}

function frame_stack_pop (env)
{
    return env.frame_stack.pop ();
}

function frame_stack_tos (env)
{
     let length = frame_stack_length (env);
     return env.frame_stack[length - 1];
}

function frame_stack_drop (env)
{
    frame_stack_pop (env);
}

function frame_stack_length (env)
{
    return env.frame_stack.length;
}

class scoping_frame_t
{
    constructor (exp_list)
    {
        this.exp_list = exp_list;
        this.length = exp_list.length;
        this.index = 0;
    }
}

class simple_frame_t
{
    constructor (exp_list)
    {
        this.exp_list = exp_list;
        this.length = exp_list.length;
        this.index = 0;
    }
}

function frame_end_p (frame)
{
    return frame.index == frame.length;
}

function frame_next_exp (frame)
{
    let exp = frame.exp_list[frame.index];
    frame.index = frame.index + 1;
    return exp;
}

function scope_stack_push (env, scope)
{
    env.scope_stack.push (scope);
}

function scope_stack_pop (env)
{
    return env.scope_stack.pop ();
}

function scope_stack_tos (env)
{
     let length = scope_stack_length (env);
     return env.scope_stack[length - 1];
}

function scope_stack_drop (env)
{
    scope_stack_pop (env);
}

function scope_stack_length (env)
{
    return env.scope_stack.length;
}

class scope_t
{
    constructor ()
    {
        this.dict = new Map ();
    }

    get (name)
    {
        return this.dict.get(name);
    }

    set (name, obj)
    {
        this.dict.set(name, obj);
    }

    clone ()
    {
        let scope = new scope_t ();
        for (let [name, obj] of this.dict) {
            scope.set(name, obj);
        }
    }
}

function scope_get (scope, name)
{
    return scope.get(name);
}

function scope_set (scope, name, obj)
{
    scope.set(name, obj);
}

function exp_list_eval (env, exp_list)
{
    let base = frame_stack_length (env);
    let frame = new simple_frame_t (exp_list);
    frame_stack_push (env, frame);
    eval_with_base (env, base);
}

function eval_with_base (env, base)
{
    while (frame_stack_length (env) > base)
        eval_one_step (env);
}

function eval_one_step (env)
{
    let frame = frame_stack_tos (env);
    let scope = scope_stack_tos (env);
    let exp = frame_next_exp (frame);
    if (frame_end_p (frame)) {
        // proper tail call
        frame_stack_drop (env);
        if (frame instanceof scoping_frame_t)
            scope_stack_drop (env);
    }
    exp.exe (env, scope);
}

class call_exp_t
{
    constructor (name)
    {
        this.name = name;
    }

    exe (env, scope)
    {
        let den = name_dict_get (env, this.name);
        den.den_exe (env);
    }
}

class get_exp_t
{
    constructor (name)
    {
        this.name = name;
    }

    exe (env, scope)
    {
        let obj = scope_get (scope, this.name);
        obj.apply (env);
    }
}

class set_exp_t
{
    constructor (local_name)
    {
        this.local_name = local_name;
    }

    exe (env, scope)
    {
        let obj = data_stack_pop (env);
        scope_set (scope, this.local_name, obj);
    }
}

class clo_exp_t
{
    constructor (exp_list)
    {
        this.exp_list = exp_list;
    }

    exe (env, scope)
    {
        let clo_obj = new clo_obj_t (this.exp_list, scope.clone ());
        data_stack_push (env, clo_obj);
    }
}

class apply_exp_t
{
   constructor () { }

   exe (env, scope)
   {
        let clo_obj = data_stack_pop (env);
        let frame = new scoping_frame_t (clo_obj.exp_list);
        frame_stack_push (env, frame);
        scope_stack_push (env, clo_obj.scope);
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

class dot_exp_t
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

// obj list
function collect_obj_list (env, exp_list)
{
}

// obj list
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

    den_exe (env)
    {
        // ><><><
        // handle type_arrow
        let frame = new scoping_frame_t (this.exp_list);
        let scope = new scope_t ();
        frame_stack_push (env, frame);
        scope_stack_push (env, scope);
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
    constructor (type_name, fields)
    {
        this.type_name = type_name;
        this.fields = fields;
    }

    apply (env)
    {
        data_stack_push (env, this);
    }
}

class clo_obj_t
{
    constructor (exp_list, scope)
    {
        this.exp_list = exp_list;
        this.scope = scope;
    }

    apply (env)
    {
        let frame = new scoping_frame_t (this.exp_list);
        frame_stack_push (env, frame);
        scope_stack_push (env, this.scope);
    }
}

function interpret_code (env, code)
{

}

function interpret_sexp_list (env, sexp_list)
{

}

function interpret_sexp (env, sexp)
{

}

function test ()
{
    let env = new env_t ();

    let fun_den = new fun_den_t (
        "dup",
        undefined,
        [
            new set_exp_t ("x"),
            new get_exp_t ("x"),
            new get_exp_t ("x"),
        ]
    );

    data_stack_push (env, new data_obj_t ("nat", "><><><"));
    scope_stack_push (env, new scope_t ());
    name_dict_set (env, "dup", fun_den);
    exp_list_eval (env, [
        new call_exp_t ("dup"),
    ]);
    console.log (env);
}

test ();

function test_many ()
{
    let counter = 0;
    while (counter < 1000000) {
        test ();
        counter = counter + 1;
    }
}

// test_many ();
