function error ()
{
    assert (false);
}

class env_t
{
    constructor ()
    {
        this.name_dict = new name_dict_t ();
        this.data_stack = [];
        this.mark_stack = [];
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
        return this.dict.get (name);
    }

    set (name, den)
    {
        this.dict.set (name, den);
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

function data_stack_peek (env, index)
{
    // index start from 1
    let length = data_stack_length (env);
    return env.data_stack[length - index];
}

function data_stack_length (env)
{
    return env.data_stack.length;
}

function mark_stack_push (env, mark)
{
    env.mark_stack.push (mark);
}

function mark_stack_pop (env)
{
    return env.mark_stack.pop ();
}

function mark_stack_tos (env)
{
    let length = mark_stack_length (env);
    return env.mark_stack[length - 1];
}

function mark_stack_drop (env)
{
    mark_stack_pop (env);
}

function mark_stack_length (env)
{
    return env.mark_stack.length;
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
    constructor (exp_vect)
    {
        this.exp_vect = exp_vect;
        this.length = exp_vect.length;
        this.index = 0;
    }
}

class simple_frame_t
{
    constructor (exp_vect)
    {
        this.exp_vect = exp_vect;
        this.length = exp_vect.length;
        this.index = 0;
    }
}

function frame_end_p (frame)
{
    return frame.index == frame.length;
}

function frame_next_exp (frame)
{
    let exp = frame.exp_vect[frame.index];
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
        return this.dict.get (name);
    }

    set (name, obj)
    {
        this.dict.set (name, obj);
    }

    clone ()
    {
        let scope = new scope_t ();
        for (let [name, obj] of this.dict) {
            scope.set (name, obj);
        }
    }
}

function exp_vect_eval (env, exp_vect)
{
    let base = frame_stack_length (env);
    let frame = new simple_frame_t (exp_vect);
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
    // {
    //     console.log ("- eval_one_step");
    //     console.log ("  env :", env);
    //     console.log ("  exp :", exp);
    //     console.log ("  scope :", scope);
    // }
    exp.exe (env, scope);
}

function exp_vect_to_obj_vect (env, exp_vect)
{
}

function exp_vect_to_obj (env, exp_vect)
{
}

function exp_to_obj (env, exp)
{

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
    constructor (local_name)
    {
        this.local_name = local_name;
    }

    exe (env, scope)
    {
        let obj = scope.get (this.local_name);
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
        scope.set (this.local_name, obj);
    }
}

class clo_exp_t
{
    constructor (exp_vect)
    {
        this.exp_vect = exp_vect;
    }

    exe (env, scope)
    {
        let clo_obj = new clo_obj_t (this.exp_vect, scope.clone ());
        data_stack_push (env, clo_obj);
    }
}

class apply_exp_t
{
    constructor () { }

    exe (env, scope)
    {
        let clo_obj = data_stack_pop (env);
        let frame = new scoping_frame_t (clo_obj.exp_vect);
        frame_stack_push (env, frame);
        scope_stack_push (env, clo_obj.scope);
    }
}

class case_exp_t
{
    constructor (exp_vect, clause_dict)
    {
        this.exp_vect = exp_vect;
        this.clause_dict = clause_dict;
    }

    exe (env, scope)
    {
        let obj = exp_vect_to_obj (env, exp_vect);
        assert (obj instanceof data_obj_t);
        let exp_vect = this.clause_dict.get (obj.type_name);
        if (exp_vect) {
            let frame = new simple_frame_t (exp_vect);
            frame_stack_push (env, frame);
        }
        else {
            let exp_vect = this.clause_dict.get ("else");
            if (exp_vect) {
                let frame = new simple_frame_t (exp_vect);
                frame_stack_push (env, frame);
            }
            else {
                error ();
            }
        }
    }
}

class clause_dict_t
{
    constructor ()
    {
        this.dict = new Map ();
    }

    get (type_name)
    {
        return this.dict.get (type_name);
    }

    set (type_name, exp_vect)
    {
        this.dict.set (type_name, exp_vect);
    }
}

class cons_exp_t
{
    constructor (type_name)
    {
        this.type_name = type_name;
    }

    exe (env, scope)
    {
        let type_name = this.type_name;
        let type_den = name_dict_get (type_name);
        assert (type_den instanceof type_den_t);
        let field_dict = new field_dict_t ();
        for (let field of type_den.reversed_field_vect) {
            let obj = data_stack_pop (env);
            field_dict.set (field, obj)
        }
        let data_obj = new data_obj_t (type_name, field_dict);
        data_stack_push (env, data_obj);
    }
}

class field_dict_t
{
    constructor ()
    {
        this.dict = new Map ();
    }

    get (field_name)
    {
        return this.dict.get (field_name);
    }

    set (field_name, obj)
    {
        this.dict.set (field_name, obj);
    }
}

class field_exp_t
{
    constructor (field_name)
    {
        this.field_name = field_name;
    }

    exe (env, scope)
    {
        let data_obj = data_stack_pop (env);
        assert (data_obj instanceof data_obj_t);
        let obj = data_obj.field_dict.get (field_name);
        obj.apply (env);
    }
}

class dot_exp_t
{
    constructor ()
    {

    }

    exe (env, scope)
    {

    }
}

class create_exp_t
{
    constructor (type_name)
    {
        this.type_name = type_name;
    }

    exe (env, scope)
    {

    }
}

class clone_exp_t
{
    constructor () { }

    exe (env, scope)
    {

    }
}

class data_obj_t
{
    constructor (type_name, field_dict)
    {
        this.type_name = type_name;
        this.field_dict = field_dict;
    }

    apply (env)
    {
        data_stack_push (env, this);
    }
}

class clo_obj_t
{
    constructor (exp_vect, scope)
    {
        this.type_name = "$arrow-t";
        this.exp_vect = exp_vect;
        this.scope = scope;
    }

    apply (env)
    {
        let frame = new scoping_frame_t (this.exp_vect);
        frame_stack_push (env, frame);
        scope_stack_push (env, this.scope);
    }
}

class dot_obj_t
{
    constructor ()
    {

    }

    apply (env)
    {
        data_stack_push (env, this);
    }
}

class union_den_t
{
    constructor (union_vect)
    {
        this.union_vect = union_vect;
    }

    den_exe (env)
    {
        error ();
    }
}

class type_den_t
{
    constructor (reversed_field_vect)
    {
        this.reversed_field_vect = reversed_field_vect;
    }

    den_exe (env)
    {
        error ();
    }
}

class fun_den_t
{
    constructor (exp_vect)
    {
        this.exp_vect = exp_vect;
    }

    den_exe (env)
    {
        let frame = new scoping_frame_t (this.exp_vect);
        let scope = new scope_t ();
        frame_stack_push (env, frame);
        scope_stack_push (env, scope);
    }
}

class sig_den_t
{
    constructor (input_arity, output_arity)
    {
        this.input_arity = input_arity;
        this.output_arity = output_arity;
        this.gene_dict = new gene_dict_t ();
    }

    den_exe (env)
    {
        let type_name_vect = [];
        let counter = 0;
        while (counter < this.input_arity) {
            let obj = data_stack_peek (env, counter);
            type_name_vect.unshift (obj.type_name);
            counter = counter + 1;
        }
        let gene_den = this.gene_dict.get (type_name_vect);
        assert (gene_den);
        gene_den.gene_den_exe (env, type_name_vect);
    }
}

class gene_dict_t
{
    constructor ()
    {
        this.dict = new Map ();
    }

    get (gene_name)
    {

    }

    set (gene_name, gene_den)
    {

    }
}

class gene_den_t
{
    constructor (default_fun_den)
    {
        this.default_fun_den = default_fun_den;
        this.disp_dict = new disp_dict_t ();
    }

    gene_den_exe (env, type_name_vect)
    {
        let fun_den = this.disp_dict.get (type_name_vect);
        if (fun_den)
            fun_den.den_exe (env);
        else
            this.default_fun_den.den_exe (env);
    }
}

class disp_dict_t
{

}

function interpret_code (env, code)
{

}

function interpret_sexp_vect (env, sexp_vect)
{

}

function interpret_sexp (env, sexp)
{

}

function test ()
{
    let env = new env_t ();

    let fun_den = new fun_den_t (
        [
            new set_exp_t ("x"),
            new get_exp_t ("x"),
            new get_exp_t ("x"),
        ]
    );

    data_stack_push (env, new data_obj_t ("nat", "><><><"));
    scope_stack_push (env, new scope_t ());
    name_dict_set (env, "dup", fun_den);
    exp_vect_eval (env, [
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
