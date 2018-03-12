      function print (x)
      {
          string_print (repr (x));
      }
      function repr (x)
      {
          return (x.toString ());
      }
      function string_print (x)
      {
          process.stdout.write (x);
      }
      function number_to_string (n)
      {
          return n.toString ();
      }
      function vect_eq_p (v1, v2)
      {
          if (v1.length !== v2.length)
              return false;
          let index = 0;
          while (index < v1.length) {
              if (v1[index] !== v2[index])
                  return false;
              index = index + 1;
          }
          return true;
      }
      function function_p (x)
      {
          return x instanceof Function;
      }
      function string_p (x)
      {
          return typeof x === "string";
      }
      function number_p (x)
      {
          return typeof x === "number";
      }
      function vect_p (x)
      {
          return x instanceof Array;
      }
      function vect_empty_p (x)
      {
          assert (x instanceof Array);
          return x.length === 0;
      }
      function vect_member_p (x, vect)
      {
          assert (vect_p (vect));
          for (let y of vect) {
              if (x === y)
                  return true;
          }
          return false;
      }
      function dict_length (dict)
      {
          let length = 0;
          for (let x of dict.keys ())
              length = length +1;
          return length;
      }
      function assert (x) {
          if (! x) {
              throw new Error('assertion fail!');
          }
      }
      function error ()
      {
          print ("\n");
          print ("\n");
          throw new Error('fatal error!');
      }
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
          index = index + 1;
          let length = data_stack_length (env);
          return env.data_stack[length - index];
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
          return frame.index === frame.length;
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
              return scope;
          }
      }
    function run_one_step (env)
    {
        let frame = frame_stack_tos (env);
        if (frame_end_p (frame)) {
            frame_stack_drop (env);
            if (frame instanceof scoping_frame_t)
                scope_stack_drop (env);
            return;
        }
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
    function run_with_base (env, base)
    {
        while (frame_stack_length (env) > base)
            run_one_step (env);
    }
    function exp_vect_run (env, exp_vect)
    {
        let base = frame_stack_length (env);
        let frame = new simple_frame_t (exp_vect);
        frame_stack_push (env, frame);
        run_with_base (env, base);
    }
    function closure_apply (env, closure)
    {
        let frame = new scoping_frame_t (closure.exp_vect);
        frame_stack_push (env, frame);
        scope_stack_push (env, closure.scope);
    }
    function closure_apply_now (env, closure)
    {
        data_stack_push (env, closure);
        let exp_vect = [new apply_exp_t ()];
        exp_vect_run (env, exp_vect);
    }
    function closure_to_obj_vect (env, closure)
    {
        let mark = data_stack_length (env);
        closure_apply_now (env, closure);
        let length = data_stack_length (env);
        let obj_vect = [];
        while (length > mark) {
           let obj = data_stack_pop (env);
           obj_vect.unshift (obj);
           length = length - 1;
        }
        return obj_vect;
    }
    function closure_to_obj (env, closure)
    {
        closure_apply_now (env, closure);
        let obj = data_stack_pop (env);
        return obj;
    }
    class call_exp_t
    {
        constructor (name)
        {
            this.name = name;
        }

        exe (env, scope)
        {
            let obj = scope.get (this.name);
            if (obj !== undefined) {
                if (obj instanceof closure_t)
                    closure_apply (env, obj);
                else
                    data_stack_push (env, obj);
            }
            else {
                let den = name_dict_get (env, this.name);
                if (! den) {
                    print ("- exe call_exp_t\n");
                    print ("  unknown name : ");
                    print (this.name);
                    print ("\n");
                    error ();
                }
                den.den_exe (env);
            }
        }
    }
    class let_exp_t
    {
        constructor (name_vect)
        {
            this.name_vect = name_vect;
        }

        exe (env, scope)
        {
            let name_vect = this.name_vect.slice ();
            while (name_vect.length > 0) {
                let name = name_vect.pop ();
                let obj = data_stack_pop (env);
                scope.set (name, obj);
            }
        }
    }
    class closure_exp_t
    {
        constructor (exp_vect)
        {
            this.exp_vect = exp_vect;
        }

        exe (env, scope)
        {
            let closure =
                new closure_t (
                    this.exp_vect,
                    scope.clone ());
            data_stack_push (env, closure);
        }
    }
    class apply_exp_t
    {
        constructor () { }

        exe (env, scope)
        {
            let closure = data_stack_pop (env);
            let frame = new scoping_frame_t (closure.exp_vect);
            frame_stack_push (env, frame);
            scope_stack_push (env, closure.scope);
        }
    }
    class case_exp_t
    {
        constructor (arg_exp_vect, case_clause_dict)
        {
            this.arg_exp_vect = arg_exp_vect;
            this.case_clause_dict = case_clause_dict;
        }

        exe (env, scope)
        {
            let closure =
                new closure_t (
                    this.arg_exp_vect,
                    scope.clone ());
            let obj = closure_to_obj (env, closure);
            let exp_vect = this.case_clause_dict.get (type_of (obj));
            if (exp_vect) {
                let closure =
                    new closure_t (
                        exp_vect,
                        scope.clone ());
                closure_apply (env, closure);
            }
            else {
                let exp_vect = this.case_clause_dict.get ("else");
                if (exp_vect) {
                    let closure =
                        new closure_t (
                            exp_vect,
                            scope.clone ());
                    closure_apply (env, closure);
                }
                else {
                    print ("- case mismatch!\n");
                    error ();
                }
            }
        }
    }
    class case_clause_dict_t
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
    class field_exp_t
    {
        constructor (field_name)
        {
            this.field_name = field_name;
        }

        exe (env, scope)
        {
            let data = data_stack_pop (env);
            let obj = undefined;
            if (data instanceof data_t)
                obj = data.field_dict.get (this.field_name);
            else
                obj = data[this.field_name];
            if (obj instanceof closure_t)
                closure_apply (env, obj);
            else
                data_stack_push (env, obj);
        }
    }
    class set_field_exp_t
    {
        constructor (field_name)
        {
            this.field_name = field_name;
        }

        exe (env, scope)
        {
            let data = data_stack_pop (env);
            let obj = data_stack_pop (env);
            if (data instanceof data_t)
                data.field_dict.set (this.field_name, obj);
            else
                data[this.field_name] = obj;
        }
    }
    class dot_exp_t
    {
        constructor (reversed_field_name_vect)
        {
            this.reversed_field_name_vect
                = reversed_field_name_vect;
        }

        exe (env, scope)
        {
            let field_dict = new field_dict_t ();
            for (let field_name of this.reversed_field_name_vect) {
                let obj = data_stack_pop (env);
                field_dict.set (field_name, obj)
            }
            data_stack_push (env, field_dict);
        }
    }
    class clone_exp_t
    {
        constructor () { }

        exe (env, scope)
        {
            let data = data_stack_pop (env);
            assert (data instanceof data_t);
            let field_dict = data_stack_pop (env);
            assert (field_dict instanceof field_dict_t);
            let new_field_dict = new field_dict_t ();
            // .dict of field_dict should be hidden
            //   but I used it here
            for (let [field_name, obj] of data.field_dict.dict) {
                new_field_dict.set (field_name, obj);
            }
            for (let [field_name, obj] of field_dict.dict) {
                new_field_dict.set (field_name, obj);
            }
            let new_data =
                new data_t (
                    data.type_name,
                    new_field_dict);
            data_stack_push (env, new_data);
        }
    }
    class lit_exp_t
    {
        constructor (obj)
        {
            this.obj = obj;
        }

        exe (env, scope)
        {
            data_stack_push (env, this.obj);
        }
    }
    class eq_p_exp_t
    {
        constructor () { }

        exe (env, scope)
        {
            let b = data_stack_pop (env);
            let a = data_stack_pop (env);
            if (eq_p (a, b)) {
                data_stack_push (env, true);
            }
            else {
                data_stack_push (env, false);
            }
        }
    }
    class mark_exp_t
    {
        constructor () { }

        exe (env, scope)
        {
            data_stack_push (env, new marker_t ());
        }
    }
    class collect_list_exp_t
    {
        constructor () { }

        exe (env, scope)
        {
            let vect = [];
            while (true) {
                let obj = data_stack_pop (env);
                if (obj instanceof marker_t)
                    break;
                else
                    vect.unshift (obj);
            }
            data_stack_push (env, vect_to_list (vect));
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
    class var_den_t
    {
        constructor (obj)
        {
            this.obj = obj;
        }

        den_exe (env)
        {
            data_stack_push (env, this.obj);
        }
    }
    class union_den_t
    {
        constructor (sub_type_name_vect)
        {
            this.sub_type_name_vect = sub_type_name_vect;
        }

        den_exe (env)
        {
            error ();
        }
    }
    class union_predicate_den_t
    {
        constructor (sub_type_name_vect)
        {
            this.sub_type_name_vect = sub_type_name_vect;
        }

        den_exe (env)
        {
            let a = data_stack_pop (env);
            for (let type_name of this.sub_type_name_vect) {
                if (type_name === type_of (a)) {
                    data_stack_push (env, true);
                    return;
                }
            }
            data_stack_push (env, false);
        }
    }
    class data_den_t
    {
        constructor (reversed_field_name_vect)
        {
            this.reversed_field_name_vect
                = reversed_field_name_vect;
        }

        den_exe (env)
        {
            error ();
        }
    }
    class data_cons_den_t
    {
        constructor (type_name)
        {
            this.type_name = type_name;
        }

        den_exe (env)
        {
            let type_name = this.type_name;
            let data_den = name_dict_get (env, type_name);
            assert (data_den instanceof data_den_t);
            let field_dict = new field_dict_t ();
            for (let field_name of data_den.reversed_field_name_vect) {
                let obj = data_stack_pop (env);
                field_dict.set (field_name, obj)
            }
            let data = new data_t (type_name, field_dict);
            data_stack_push (env, data);
        }
    }
    class data_create_den_t
    {
        constructor (type_name)
        {
            this.type_name = type_name;
        }

        den_exe (env)
        {
            let field_dict = data_stack_pop (env);
            assert (field_dict instanceof field_dict_t);
            let data
                = new data_t (
                    this.type_name,
                    field_dict);
            data_stack_push (env, data);
        }
    }
    class data_predicate_den_t
    {
        constructor (type_name)
        {
            this.type_name = type_name;
        }

        den_exe (env)
        {
            let a = data_stack_pop (env);
            data_stack_push (
                env,
                type_of (a) === this.type_name);
        }
    }
    class macro_den_t
    {
        constructor (exp_vect)
        {
            this.exp_vect = exp_vect;
        }

        den_exe (env)
        {
            exp_vect_run (env, this.exp_vect)
        }
    }
    class prim_den_t
    {
        constructor (prim_fn)
        {
            this.prim_fn = prim_fn;
        }

        den_exe (env)
        {
            this.prim_fn (env);
        }
    }
    class gene_den_t
    {
        constructor (arity, default_fun_den)
        {
            this.arity = arity;
            this.default_fun_den = default_fun_den;
            this.disp_dict = new disp_dict_t ();
        }

        den_exe (env)
        {
            let type_name_vect = [];
            let counter = 0;
            while (counter < this.arity) {
                let obj = data_stack_peek (env, counter);
                type_name_vect.unshift (type_of (obj));
                counter = counter + 1;
            }
            let fun_den = this.disp_dict.find (env, type_name_vect);
            if (fun_den !== undefined)
                fun_den.den_exe (env);
            else
                this.default_fun_den.den_exe (env);
        }
    }
    class disp_dict_t
    {
        constructor ()
        {
            this.dict = new Map ();
        }

        find (env, type_name_vect)
        {
            for (let [key, value] of this.dict) {
                if (vect_eq_p (type_name_vect, key))
                    return value;
            }
            return undefined;
        }

        set (type_name_vect, fun_den)
        {
            for (let key of this.dict.keys ()) {
                if (vect_eq_p (key, type_name_vect)) {
                    this.dict.set (key, fun_den);
                    return;
                }
            }
            this.dict.set (type_name_vect, fun_den)
        }
    }
    function type_of (x)
    {
        let type_name = x.type_name;
        if (type_name)
            return type_name
        else if (string_p (x))
            return "string-t";
        else if (number_p (x))
            return "number-t";
        else if (x === true)
            return "true-t";
        else if (x === false)
            return "false-t";
        else
            // return dashlize (x.constructor.name);
            print ("- type_of fail on : ");
            print (x);
            print ("\n");
            error ();
    }
    function eq_p (x, y)
    {
        if (function_p (x.eq_p))
            return x.eq_p (y);
        if (function_p (y.eq_p))
            return y.eq_p (x);
        else
            return x === y;
    }
    class data_t
    {
        constructor (type_name, field_dict)
        {
            this.type_name = type_name;
            this.field_dict = field_dict;
        }

        eq_p (that)
        {
            if (this.type_name !== type_of (that))
                return false;
            else
                return eq_p (this.field_dict, that.field_dict);
        }
    }
    class closure_t
    {
        constructor (exp_vect, scope)
        {
            this.type_name = "closure-t";
            this.exp_vect = exp_vect;
            this.scope = scope;
        }

        eq_p (that)
        {
            if (this.type_name !== type_of (that))
                return false;
            if (this.exp_vect !== that.exp_vect)
                return false;
            if (this.scope !== that.scope)
                return false;
            else
                return true;
        }
    }
    class field_dict_t
    {
        constructor ()
        {
            this.type_name = "field-dict-t";
            this.dict = new Map ();
        }

        eq_p (that)
        {
            if (this.type_name !== type_of (that))
                return false;
            if (dict_length (this.dict) !== dict_length (that.dict))
                return false;
            for (let [field_name, obj] of this.dict) {
                if (! (eq_p (obj, that.dict.get (field_name))))
                    return false;
            }
            return true;
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
    class marker_t
    {
        constructor (exp_vect, scope)
        {
            this.type_name = "marker-t";
        }

        eq_p (that)
        {
            if (this.type_name !== type_of (that))
                return false;
            else
                return true;
        }
    }
    let the_top_keyword_dict = new Map ();
    function env_merge (env, den_dict)
    {
        for (let [name, den] of den_dict) {
            name_dict_set (env, name, den);
        }
    }
    function new_top_keyword (name, prim_fn)
    {
        the_top_keyword_dict.set (name, prim_fn);
    }
    new_top_keyword (
        "+union",
        function (env, sexp_list)
        {
            let name = sexp_list.car;
            assert (union_name_p (name));
            let rest_list = sexp_list.cdr;
            let rest_vect = list_to_vect (rest_list);
            let sub_type_name_vect = [];
            for (let type_name of rest_vect) {
                sub_type_name_vect.push (type_name);
            }
            name_dict_set (
                env, name,
                new union_den_t (sub_type_name_vect));
            let prefix = union_name_prefix (name);
            name_dict_set (
                env, prefix.concat ("-p"),
                new union_predicate_den_t (sub_type_name_vect));
        }
    );
    function union_name_p (x)
    {
        if (! (string_p (x)))
            return false;
        if (x.length <= 2)
            return false;
        if (x.slice (x.length -2, x.length) === "-u")
            return true;
        else
            return false;
    }
    function union_name_prefix (x)
    {
        return x.slice (0, x.length -2);
    }
    new_top_keyword (
        "+data",
        function (env, sexp_list)
        {
            let name = sexp_list.car;
            assert (data_name_p (name));
            let rest_list = sexp_list.cdr;
            let rest_vect = list_to_vect (rest_list);
            let reversed_field_name_vect = [];
            for (let sexp of rest_vect) {
                reversed_field_name_vect.unshift (sexp);
            }
            name_dict_set (
                env, name,
                new data_den_t (reversed_field_name_vect));
            let prefix = data_name_prefix (name);
            name_dict_set (
                env, prefix.concat ("-c"),
                new data_cons_den_t (name));
            name_dict_set (
                env, prefix.concat ("-p"),
                new data_predicate_den_t (name));
            name_dict_set (
                env, prefix.concat ("-cr"),
                new data_create_den_t (name));
        }
    );
    function data_name_p (x)
    {
        if (! (string_p (x)))
            return false;
        if (x.length <= 2)
            return false;
        if (x.slice (x.length -2, x.length) === "-t")
            return true;
        else
            return false;
    }
    function data_name_prefix (x)
    {
        return x.slice (0, x.length -2);
    }
    new_top_keyword (
        "+fun",
        function (env, sexp_list)
        {
            let name = sexp_list.car;
            let rest_list = sexp_list.cdr;
            let exp_vect = sexp_list_compile (env, rest_list);
            name_dict_set (
                env, name,
                new fun_den_t (exp_vect));
        }
    );
    new_top_keyword (
        "+var",
        function (env, sexp_list)
        {
            let name = sexp_list.car;
            let rest_list = sexp_list.cdr;
            let exp_vect = sexp_list_compile (env, rest_list);
            exp_vect_run (env, exp_vect);
            let obj = data_stack_pop (env);
            name_dict_set (
                env, name,
                new var_den_t (obj));
        }
    );
    new_top_keyword (
        "+macro",
        function (env, sexp_list)
        {
            let name = sexp_list.car;
            let rest_list = sexp_list.cdr;
            let exp_vect = sexp_list_compile (env, rest_list);
            name_dict_set (
                env, name,
                new macro_den_t (exp_vect));
        }
    );
    new_top_keyword (
        "+gene",
        function (env, sexp_list)
        {
            let name = sexp_list.car;
            let arity = eval (sexp_list.cdr.car);
            let rest_list = sexp_list.cdr.cdr;
            let exp_vect = sexp_list_compile (env, rest_list);
            name_dict_set (
                env, name,
                new gene_den_t (arity, new fun_den_t (exp_vect)));
        }
    );
    new_top_keyword (
        "+disp",
        function (env, sexp_list)
        {
            let name = sexp_list.car;
            let type_name_list = sexp_list.cdr.car.cdr;
            let rest_list = sexp_list.cdr.cdr;
            let exp_vect = sexp_list_compile (env, rest_list);
            let fun_den = new fun_den_t (exp_vect);
            let type_name_vect = list_to_vect (type_name_list);
            let gene_den = name_dict_get (env, name);
            if (! (gene_den instanceof gene_den_t)) {
                print ("- (+disp) missing gene\n");
                print ("  name : ");
                print (name);
                print ("\n");
                print ("  type_name_vect : ");
                print (type_name_vect);
                print ("\n");
                error ();
            }
            let vect_vect = expand_type_name_vect (env, type_name_vect);
            for (let vect of vect_vect) {
                gene_den.disp_dict.set (vect, fun_den);
            }
        }
    );
    function expand_type_name_vect (env, type_name_vect)
    {
        let vect_vect = [];
        for (let type_name of type_name_vect) {
            let den = name_dict_get (env, type_name);
            if (den instanceof union_den_t)
                vect_vect = vect_vect_bind (
                    den.sub_type_name_vect,
                    vect_vect);
            else
                vect_vect = vect_vect_bind (
                    [type_name],
                    vect_vect);
        }
        return vect_vect;
    }
    function vect_vect_bind (vect, vect_vect)
    {
        let new_vect_vect = [];
        if (vect_vect.length === 0)
            for (let x of vect)
                new_vect_vect.push ([x]);
        else
            for (let x of vect)
                for (let v of vect_vect)
                    new_vect_vect.push ([x].concat (v));
        return new_vect_vect;
    }
    let the_keyword_dict = new Map ();
    function new_keyword (name, prim_fn)
    {
        the_keyword_dict.set (name, prim_fn);
    }
    new_keyword (
        "let",
        function (env, sexp_list)
        {
            let sexp_vect = list_to_vect (sexp_list);
            return [new let_exp_t (sexp_vect)];
        }
    );
    new_keyword (
        "begin",
        function (env, sexp_list)
        {
            return sexp_list_compile (env, sexp_list);
        }
    );
    new_keyword (
        "closure",
        function (env, sexp_list)
        {
            let exp_vect = sexp_list_compile (env, sexp_list);
            return [new closure_exp_t (exp_vect)];
        }
    )
    new_keyword (
        "case",
        function (env, sexp_list)
        {
            let case_clause_dict = new case_clause_dict_t ();
            let arg_exp_vect = sexp_compile (env, sexp_list.car);
            let rest_vect = list_to_vect (sexp_list.cdr);
            for (let sexp of rest_vect) {
                let case_name = sexp.car;
                let exp_vect = sexp_list_compile (env, sexp.cdr)
                case_clause_dict.set (case_name, exp_vect);
            }
            return [new case_exp_t (arg_exp_vect, case_clause_dict)];
        }
    );
    new_keyword (
        ".",
        function (env, sexp_list)
        {
            let sexp_vect = list_to_vect (sexp_list);
            let reversed_field_name_vect = [];
            for (let field_name of sexp_vect) {
                reversed_field_name_vect.unshift (field_name);
            }
            return [new dot_exp_t (reversed_field_name_vect)];
        }
    );
    new_keyword (
        "note",
        function (env, sexp_list)
        {
            return [];
        }
    );
    new_keyword (
        "quote",
        function (env, sexp_list)
        {
            let exp_vect = [];
            let sexp_vect = list_to_vect (sexp_list);
            for (let sexp of sexp_vect) {
                exp_vect.push (new lit_exp_t (sexp));
            }
            return exp_vect;
        }
    );
    new_keyword (
        "partquote",
        partquote_compile);
    function partquote_compile (env, sexp_list)
    {
        let exp_vect = [];
        let sexp_vect = list_to_vect (sexp_list);
        for (let sexp of sexp_vect) {
            exp_vect = exp_vect.concat (partquote_compile_one (env, sexp));
        }
        return exp_vect;
    }
    function partquote_compile_one (env, sexp)
    {
        if (string_p (sexp)) {
            return [new lit_exp_t (sexp)];
        }
        else {
            assert (cons_p (sexp));
            if (sexp.car === "@")
                return sexp_list_compile (env, sexp.cdr);
            else {
                let exp_vect = [];
                exp_vect.push (new mark_exp_t ());
                exp_vect = exp_vect.concat (partquote_compile (env, sexp));
                exp_vect.push (new collect_list_exp_t ());
                return exp_vect;
            }
        }
    }
    let the_prim_dict = new Map ();
    function new_prim (name, prim_fn)
    {
        let prim_den = new prim_den_t (prim_fn);
        the_prim_dict.set (name, prim_den);
    }
      new_prim (
          "true-c",
          function (env)
          {
              data_stack_push (env, true);
          }
      );
      new_prim (
          "false-c",
          function (env)
          {
              data_stack_push (env, false);
          }
      );
      new_prim (
          "bool/and",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, (a && b));
          }
      );
      new_prim (
          "bool/or",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, (a || b));
          }
      );
      new_prim (
          "bool/not",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, (! a));
          }
      );
      new_prim (
          "number-p",
          function (env)
          {
              let obj = data_stack_pop (env);
              data_stack_push (env, (
                  type_of (a) === "number-t"));
          }
      );
      new_prim (
          "number/inc",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, a +1);
          }
      );
      new_prim (
          "number/dec",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, a -1);
          }
      );
      new_prim (
          "number/neg",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, - a);
          }
      );
      new_prim (
          "number/add",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, a + b);
          }
      );
      new_prim (
          "number/sub",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, a - b);
          }
      );
      new_prim (
          "number/mul",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, a * b);
          }
      );
      new_prim (
          "number/div",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, a / b);
          }
      );

      new_prim (
          "number/mod",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, a % b);
          }
      );
      new_prim (
          "number/divmod",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, a / b);
              data_stack_push (env, a % b);
          }
      );
      new_prim (
          "number/moddiv",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, a % b);
              data_stack_push (env, a / b);
          }
      );
      new_prim (
          "number/lt-p",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, (
                  a < b));
          }
      );
      new_prim (
          "number/lteq-p",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, (
                  a <= b));
          }
      );
      new_prim (
          "number/gt-p",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, (
                  a > b));
          }
      );
      new_prim (
          "number/gteq-p",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, (
                  a >= b));
          }
      );
      new_prim (
          "string-p",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, (
                  type_of (a) === "string-t"));
          }
      );
      new_prim (
          "string/length",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, a.length);
          }
      );
      new_prim (
          "string/ref",
          function (env)
          {
              let index = data_stack_pop (env);
              let string = data_stack_pop (env);
              let char = string[index];
              data_stack_push (env, char);
          }
      );
      new_prim (
          "string/append",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, a.concat (b));
          }
      );
      new_prim (
          "string/slice",
          function (env)
          {
              let end = data_stack_pop (env);
              let begin = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, a.slice (begin, end));
          }
      );
      new_prim (
          "number->string",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, number_to_string (a));
          }
      );
      new_prim (
          "string/print",
          function (env)
          {
              let a = data_stack_pop (env);
              print (a);
          }
      );
      new_prim (
          "nl",
          function (env)
          {
              print ("\n");
          }
      );
      new_prim (
          "doublequote/string",
          function (env)
          {
              data_stack_push (env, '"');
          }
      );
      new_prim (
          "singlequote/string",
          function (env)
          {
              data_stack_push (env, "'");
          }
      );
      function default_repr (env, obj)
      {
          if (obj instanceof data_t)
              return data_repr (env, obj);
          else
              return repr (obj);
      }
      function data_repr (env, obj)
      {
          let prefix = data_name_prefix (obj.type_name);
          let repr_string = prefix.concat ("-c");
          let data_den = name_dict_get (env, obj.type_name);
          for (let field_name of data_den.reversed_field_name_vect) {
              let field_obj = obj.field_dict.get (field_name);
              repr_string = repr_string.concat (" ");
              repr_string = repr_string.concat (data_repr (env, field_obj));
          }
          return repr_string;
      }
      new_prim (
          "default-repr",
          function (env)
          {
              let obj = data_stack_pop (env);
              data_stack_push (env, default_repr (env, obj));
          }
      );
      class null_t
      {
          constructor ()
          {
              this.type_name = "null-t";
          }

          eq_p (that)
          {
              if (this.type_name !== type_of (that))
                  return false;
              else
                  return true;
          }
      }
      function null_c ()
      {
          return new null_t ();
      }
      function null_p (x)
      {
          return x instanceof null_t;
      }
      class cons_t
      {
          constructor (car, cdr)
          {
              this.type_name = "cons-t";
              this.car = car;
              this.cdr = cdr;
          }

          eq_p (that)
          {
              if (this.type_name !== type_of (that))
                  return false;
              else if (! (eq_p (this.car, that.car)))
                  return false;
              else if (! (eq_p (this.cdr, (that.cdr))))
                  return false;
              else
                  return true;
          }
      }
      function cons_c (car, cdr)
      {
          assert (list_p (cdr));
          return new cons_t (car, cdr);
      }
      function cons_p (x)
      {
          return x instanceof cons_t;
      }
      function list_p (x)
      {
          return (null_p (x) || cons_p (x));
      }
      new_prim (
          "null-c",
          function (env)
          {
              data_stack_push (env, new null_t ());
          }
      );
      new_prim (
          "null-p",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, (
                  type_of (a) === "null-t"));
          }
      );
      new_prim (
          "cons-c",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new cons_t (a, b));
          }
      );
      new_prim (
          "cons-p",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, (
                  type_of (a) === "cons-t"));
          }
      );
      new_prim (
          "list/spread",
          function (env)
          {
              let list = data_stack_pop (env);
              let vect = list_to_vect (list);
              for (let x of vect) {
                  data_stack_push (env, x);
              }
          }
      );
      new_prim (
          "sexp/print",
          function (env)
          {
              let sexp = data_stack_pop (env);
              sexp_print (sexp);
          }
      );
      new_prim (
          "sexp-list/print",
          function (env)
          {
              let sexp_list = data_stack_pop (env);
              sexp_list_print (sexp_list);
          }
      );
      new_prim (
          "error",
          function (env)
          {
              error ();
          }
      );
    function code_scan (string)
    {
        let string_vect = [];
        let i = 0;
        let length = string.length;
        while (i < length) {
            let char = string[i];
            if (space_p (char))
                i = i + 1;
            else if (char === ';') {
                let end = string.indexOf ('\n', i+1);
                if (end === -1)
                    break;
                else
                    i = end + 1;
            }
            else if (delimiter_p (char)) {
                string_vect.push (char);
                i = i + 1;
            }
            else if (char === '"') {
                let end = string.indexOf ('"', i+1);
                if (end === -1) {
                    print ("- code_scan fail\n");
                    print ("  doublequote mismatch\n");
                    print ("  string : ");
                    print (string);
                    print ("\n");
                    error ();
                }
                string_vect.push (string.slice (i, end + 1));
                i = end + 1;
            }
            else {
                let end = find_end (string, i+1);
                string_vect.push (string.slice (i, end + 1));
                i = end + 1;
            }
        }
        return string_vect;
    }
    function space_p (char)
    {
        return (char == ' ' ||
                char == '\n' ||
                char == '\t');
    }
    function delimiter_p (char)
    {
        return (char == '(' ||
                char == ')' ||
                char == '[' ||
                char == ']' ||
                char == '{' ||
                char == '}' ||
                char == ',' ||
                char == ';' ||
                char == '`' ||
                char == "'");
    }
    function find_end (string, begin)
    {
        let length = string.length;
        let i = begin;
        while (true) {
            if (i === length)
                return i - 1;
            let char = string[i];
            let next = string[i+1];
            if (space_p (char) ||
                delimiter_p (char) ||
                (char === '"'))
                return i - 1;
            else
                i = i + 1;
        }
    }
    function digital_char_p (x)
    {
        return ((x === "0") ||
                (x === "1") ||
                (x === "2") ||
                (x === "3") ||
                (x === "4") ||
                (x === "5") ||
                (x === "6") ||
                (x === "7") ||
                (x === "8") ||
                (x === "9"));
    }
    function parse_sexp_vect (string_vect)
    {
        let length = string_vect.length;
        let i = 0;
        let sexp_vect = [];
        while (i < length) {
            let v = parse_sexp_with_index (string_vect, i);
            let s = v[0];
            i = v[1];
            sexp_vect.push (s);
        }
        return sexp_vect;
    }
    function parse_sexp_with_index (string_vect, i)
    {
        let string = string_vect[i];
        if (string === '(')
            return parse_sexp_cons_until_ket (string_vect, i+1, ')');
        else if (string === '[') {
            let v = parse_sexp_cons_until_ket (string_vect, i+1, ']');
            let sc = v[0];
            let i1 = v[1];
            return [cons_c ('begin', sc), i1];
        }
        else if (string === '{') {
            let v = parse_sexp_cons_until_ket (string_vect, i+1, '}');
            let sc = v[0];
            let i1 = v[1];
            return [cons_c ('closure', sc), i1];
        }
        else if (string === "'") {
            let v = parse_sexp_with_index (string_vect, i+1);
            let s = v[0];
            let i1 = v[1];
            let sc = cons_c (s, null_c ());
            return [cons_c ('quote', sc), i1];
        }
        else if (string === "`") {
            let v = parse_sexp_with_index (string_vect, i+1);
            let s = v[0];
            let i1 = v[1];
            let sc = cons_c (s, null_c ());
            return [cons_c ('partquote', sc), i1];
        }
        else
            return [string, i+1];
    }
    function parse_sexp_cons_until_ket (string_vect, i, ket)
    {
        let string = string_vect[i];
        if (string == ket)
            return [null_c (), i+1];
        else {
            let v = parse_sexp_with_index (string_vect, i);
            let s = v[0];
            let i1 = v[1];
            let v2 =
                parse_sexp_cons_until_ket (string_vect, i1, ket);
            let sc = v2[0];
            let i2 = v2[1];
            return [cons_c (s, sc), i2];
        }
    }
    function sexp_print (sexp)
    {
        if (null_p (sexp))
            print ("null-c");
        else if (cons_p (sexp)) {
        print ("(");
            sexp_list_print (sexp);
            print (")");
        }
        else
            print (sexp);
    }
    function sexp_list_print (sexp_cons)
    {
        if (null_p (sexp_cons.cdr))
            sexp_print (sexp_cons.car);
        else {
            sexp_print (sexp_cons.car);
            print (" ");
            sexp_list_print (sexp_cons.cdr);
        }
    }
    function list_to_vect (list)
    {
        if (null_p (list))
            return [];
        else {
            let e = list.car;
            let vect = [e];
            let rest = list.cdr;
            return vect.concat (list_to_vect (rest));
        }
    }
    function vect_to_list (vect)
    {
        if (vect.length === 0)
            return null_c ();
        else
            return cons_c (vect[0], vect_to_list (vect.slice (1)));
    }
    function code_eval (env, code)
    {
        let string_vect = code_scan (code);
        let sexp_vect = parse_sexp_vect (string_vect);
        top_sexp_vect_eval (env, sexp_vect);
    }
    function top_sexp_vect_eval (env, sexp_vect)
    {
        let index = 0;
        let length = sexp_vect.length;
        while (index < length) {
            let sexp = sexp_vect [index];
            if (sexp === "#note")
                index = index +1;
            else
                top_sexp_eval (env, sexp);
            index = index +1;
        }
    }
    function top_sexp_eval (env, sexp)
    {
        sexp = apply_all_passes (sexp);
        if (string_p (sexp)) {
            let exp_vect = sexp_compile (env, sexp);
            exp_vect_run (env, exp_vect);
        }
        else {
            assert (cons_p (sexp));
            let name = sexp.car;
            let sexp_list = sexp.cdr;
            if (top_keyword_name_p (env, name)) {
                let top_keyword_fn = the_top_keyword_dict.get (name);
                top_keyword_fn (env, sexp_list);
            }
            else if (macro_name_p (env, name)) {
                let den = name_dict_get (env, name);
                data_stack_push (env, sexp_list);
                den.den_exe (env);
                let new_sexp = data_stack_pop (env);
                top_sexp_eval (env, new_sexp)
            }
            else {
                let exp_vect = sexp_compile (env, sexp);
                exp_vect_run (env, exp_vect);
            }
        }
    }
    function top_keyword_name_p (env, name)
    {
        let top_keyword_fn = the_top_keyword_dict.get (name);
        if (top_keyword_fn) {
            assert (function_p (top_keyword_fn));
            return true;
        }
        else
            return false;
    }
    let the_pass_vect = [];
    function new_pass (pass_fn)
    {
        the_pass_vect.push (pass_fn);
    }
    function apply_all_passes (sexp)
    {
        for (let pass_fn of the_pass_vect) {
            assert (pass_fn instanceof Function);
            sexp = pass_fn (sexp);
        }
        return sexp;
    }
      function pass_for_recur (sexp)
      {
          let keyword = sexp.car;
          if (cons_p (sexp) &&
              (keyword === "+fun" ||
               keyword === "+gene" ||
               keyword === "+disp" ||
               keyword === "+macro")) {
              let name = sexp.cdr.car;
              let body = sexp.cdr.cdr;
              body = substitute_recur (name, body);
              return cons_c (keyword, cons_c (name, body));
          }
          else
              return sexp;
      }

      new_pass (pass_for_recur);
      function substitute_recur (name, sexp)
      {
          if (string_p (sexp)) {
              if (sexp === "recur")
                  return name;
              else
                  return sexp;
          }
          else if (null_p (sexp)) {
              return null_c ();
          }
          else {
              return cons_c (substitute_recur (name, sexp.car),
                             substitute_recur (name, sexp.cdr));
          }
      }
    function sexp_list_compile (env, sexp_list)
    {
        let sexp_vect = list_to_vect (sexp_list);
        let exp_vect = [];
        for (let sexp of sexp_vect) {
            exp_vect = exp_vect.concat (sexp_compile (env, sexp));
        }
        return exp_vect;
    }
    function sexp_compile (env, sexp)
    {
        if (string_p (sexp))
            return string_compile (sexp);
        else {
            assert (cons_p (sexp));
            let name = sexp.car;
            let rest_list = sexp.cdr;
            if (keyword_name_p (env, name)) {
                let keyword_fn = the_keyword_dict.get (name);
                return keyword_fn (env, rest_list);
            }
            else if (macro_name_p (env, name)) {
                let den = name_dict_get (env, name);
                data_stack_push (env, rest_list);
                den.den_exe (env);
                let new_sexp = data_stack_pop (env);
                return sexp_compile (env, new_sexp);
            }
            else {
                print ("- sexp_compile fail\n");
                print ("  unknown name : ");
                print (name);
                print ("\n");
                error ();
            }
        }
    }
    function string_compile (sexp)
    {
        if (sexp === "apply")
            return [new apply_exp_t ()];
        else if (sexp === "eq-p")
            return [new eq_p_exp_t ()];
        else if (sexp === "clone")
            return [new clone_exp_t ()];
        else if (sexp === "mark")
            return [new mark_exp_t ()];
        else if (sexp === "collect-list")
            return [new collect_list_exp_t ()];
        else if (sexp === ",")
            return [];
        else if (string_string_p (sexp)) {
            let string = string_string_to_string (sexp);
            return [new lit_exp_t (string)];
        }
        else if (number_string_p (sexp)) {
            let number = number_string_to_number (sexp);
            return [new lit_exp_t (number)];
        }
        else if (dot_string_p (sexp)) {
            return dot_string_to_exp_vect (sexp);
        }
        else {
            let name = sexp;
            return [new call_exp_t (name)];
        }
    }
    function keyword_name_p (env, name)
    {
        let keyword_fn = the_keyword_dict.get (name);
        if (keyword_fn) {
            assert (function_p (keyword_fn));
            return true;
        }
        else
            return false;
    }
    function macro_name_p (env, name)
    {
        let den = name_dict_get (env, name);
        if (! den)
            return false;
        if (den instanceof macro_den_t)
            return true;
        else
            return false;
    }
    function string_string_p (x)
    {
        if (! (string_p (x)))
            return false;
        else if (x.length <= 1)
            return false;
        else if (x[0] !== '"')
            return false;
        else if (x[x.length -1] !== '"')
            return false;
        else
            return true;
    }
    function number_string_p (number_string)
    {
        if (! (string_p (number_string)))
            return false;
        let number_of_dots = 0;
        for (let x of number_string) {
            if (x === ".")
                number_of_dots = number_of_dots +1;
            else if (! (digital_char_p (x)))
                return false;
        }
        if (number_of_dots > 1)
            return false;
        else
            return true;
    }
    function number_string_to_number (number_string)
    {
        return eval (number_string);
    }
    function string_string_to_string (ss)
    {
        return ss.slice (1, ss.length -1);
    }
    function dot_string_p (x)
    {
        return ((string_p (x)) &&
                (x.indexOf (".") !== -1));
    }
    function dot_string_to_exp_vect (dot_string)
    {
        let string_vect = dot_string.split (".");
        let name = string_vect[0];
        let exp_vect = [];
        if (name !== "")
            exp_vect.push (new call_exp_t (name));
        let length = string_vect.length;
        for (let field_name of string_vect.slice (1, length -1)) {
            exp_vect.push (new field_exp_t (field_name));
        }
        let last_field_name = string_vect[length -1];
        if (last_field_name[last_field_name.length -1] === "!")
            exp_vect.push (new set_field_exp_t (
                last_field_name.slice (0, last_field_name.length -1)));
        else
            exp_vect.push (new field_exp_t (last_field_name));
        return exp_vect;
    }
    function eval_script (code)
    {
        assert (string_p (code));
        let env = new env_t ();
        let top_level_scope = new scope_t ();
        scope_stack_push (env, top_level_scope);
        env_merge (env, the_prim_dict);
        code_eval (env, code);
        return env;
    }
    module.exports.eval_script = eval_script;
    // export eval_script;
