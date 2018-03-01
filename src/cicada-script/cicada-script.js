      let print = console.log;
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
          return typeof x === 'string';
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
              throw new Error('assert fail!');
          }
      }
      function error ()
      {
          print ("");
          print ("");
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
        // {
        //     print ("- run_one_step");
        //     print ("  exp :", exp);
        //     print ("  scope :", scope);
        //     print ("  env :", env);
        //     print ("");
        // }
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
        let obj_vect = closure_to_obj_vect (env, closure);
        assert (obj_vect.length === 1);
        return obj_vect[0];
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
            // {
            //     print ("- call_exp");
            //     print (this.name);
            //     print (scope);
            //     print (env);
            //     print ("");
            // }
            if (obj) {
                if (obj instanceof closure_t)
                    closure_apply (env, obj);
                else
                    data_stack_push (env, obj);
            }
            else {
                let den = name_dict_get (env, this.name);
                if (! den) {
                    print ("- exe call_exp_t");
                    print ("  unknown name :", this.name);
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
            let exp_vect = this.case_clause_dict.get (obj.type_name);
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
                    print ("- case mismatch!");
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
            assert (data instanceof data_t);
            let obj = data.field_dict.get (this.field_name);
            assert (obj);
            if (obj instanceof closure_t)
                closure_apply (env, obj);
            else
                data_stack_push (env, obj);
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
            if (a.eq_p (b)) {
                data_stack_push (env, new true_t ());
            }
            else {
                data_stack_push (env, new false_t ());
            }
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
        constructor ()
        {

        }

        den_exe ()
        {

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
                if (type_name === a.type_name) {
                    data_stack_push (env, new true_t ());
                    return;
                }
            }
            data_stack_push (env, new false_t ());
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
            data_stack_push (env, new_bool (
                a.type_name === this.type_name));
        }
    }
    class top_macro_den_t
    {
        constructor ()
        {
        }

        den_exe (env)
        {
        }
    }
    class macro_den_t
    {
        constructor ()
        {
        }

        den_exe (env)
        {
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
    class data_t
    {
        constructor (type_name, field_dict)
        {
            this.type_name = type_name;
            this.field_dict = field_dict;
        }

        eq_p (that)
        {
            if (this.type_name !== that.type_name)
                return false;
            else
                return this.field_dict.eq_p (that.field_dict);
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
            if (this.type_name !== that.type_name)
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
            if (this.type_name !== that.type_name)
                return false;
            if (dict_length (this.dict) !== dict_length (that.dict))
                return false;
            for (let [field_name, obj] of this.dict) {
                if (! (obj.eq_p (that.dict.get (field_name))))
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
            let fun_den = new fun_den_t (exp_vect);
            name_dict_set (env, name, fun_den);
        }
    );


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
            let sexp_vect = list_to_vect (sexp_list);
            return [new closure_exp_t (sexp_vect)];
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
        "field",
        function (env, sexp_list)
        {
            return [new field_exp_t (sexp_list.car)];
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
    let the_prim_dict = new Map ();
    function new_prim (name, prim_fn)
    {
        let prim_den = new prim_den_t (prim_fn);
        the_prim_dict.set (name, prim_den);
    }
      class true_t
      {
          constructor ()
          {
              this.type_name = "true-t";
          }

          eq_p (that)
          {
            if (this.type_name !== that.type_name)
                return false;
            else
                return true;
          }
      }
      class false_t
      {
          constructor ()
          {
              this.type_name = "false-t";
          }

          eq_p (that)
          {
            if (this.type_name !== that.type_name)
                return false;
            else
                return true;
          }
      }
      function new_bool (bool)
      {
          if (bool)
              return new true_t ();
          else
              return new false_t ();
      }
      new_prim (
          "true-c",
          function (env)
          {
              data_stack_push (env, new true_t ());
          }
      );
      new_prim (
          "false-c",
          function (env)
          {
              data_stack_push (env, new false_t ());
          }
      );
      new_prim (
          "bool-and",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              if (a instanceof false_t)
                  data_stack_push (env, new false_t ());
              else if (b instanceof false_t)
                  data_stack_push (env, new false_t ());
              else
                  data_stack_push (env, new true_t ());
          }
      );
      new_prim (
          "bool-or",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              if (a instanceof true_t)
                  data_stack_push (env, new true_t ());
              else if (b instanceof true_t)
                  data_stack_push (env, new true_t ());
              else
                  data_stack_push (env, new false_t ());
          }
      );
      new_prim (
          "bool-not",
          function (env)
          {
              let a = data_stack_pop (env);
              if (a instanceof false_t)
                  data_stack_push (env, new true_t ());
              else
                  data_stack_push (env, new false_t ());
          }
      );
      new_prim (
          "bool-p",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, new_bool (
                  ((a instanceof false_t) ||
                   (a instanceof true_t))));
          }
      );
      class number_t
      {
          constructor (number)
          {
              this.type_name = "number-t";
              this.number = number;
          }

          eq_p (that)
          {
            if (this.type_name !== that.type_name)
                return false;
            else
                return this.number === that.number;
          }
      }
      new_prim (
          "number-p",
          function (env)
          {
              let obj = data_stack_pop (env);
              data_stack_push (env, new_bool (
                  a.type_name === "number-t"));
          }
      );
      new_prim (
          "number-inc",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (a.number +1));
          }
      );
      new_prim (
          "number-dec",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (a.number -1));
          }
      );
      new_prim (
          "number-neg",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (- a.number));
          }
      );
      new_prim (
          "number-add",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (
                  a.number + b.number));
          }
      );
      new_prim (
          "number-sub",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (
                  a.number - b.number));
          }
      );
      new_prim (
          "number-mul",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (
                  a.number * b.number));
          }
      );
      new_prim (
          "number-div",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (
                  a.number / b.number));
          }
      );

      new_prim (
          "number-mod",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (
                  a.number % b.number));
          }
      );
      new_prim (
          "number-divmod",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (
                  a.number / b.number));
              data_stack_push (env, new number_t (
                  a.number % b.number));
          }
      );
      new_prim (
          "number-moddiv",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (
                  a.number % b.number));
              data_stack_push (env, new number_t (
                  a.number / b.number));
          }
      );
      new_prim (
          "number-lt-p",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new_bool (
                  a.number < b.number));
          }
      );
      new_prim (
          "number-lteq-p",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new_bool (
                  a.number <= b.number));
          }
      );
      new_prim (
          "number-gt-p",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new_bool (
                  a.number > b.number));
          }
      );
      new_prim (
          "number-gteq-p",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new_bool (
                  a.number >= b.number));
          }
      );
      class string_t
      {
          constructor (string)
          {
              this.type_name = "string-t";
              this.string = string;
          }

          eq_p (that)
          {
            if (this.type_name !== that.type_name)
                return false;
            else
                return this.string === that.string;
          }
      }
      new_prim (
          "string-p",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, new_bool (
                  a.type_name === "string-t"));
          }
      );
      new_prim (
          "string-length",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, new number_t (a.string.length));
          }
      );
      new_prim (
          "string-ref",
          function (env)
          {
              let index = data_stack_pop (env);
              let string = data_stack_pop (env);
              let char = string.string[index.number];
              data_stack_push (env, new string_t (char));
          }
      );
      new_prim (
          "string-append",
          function (env)
          {
              let b = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new string_t (
                  a.string.concat (b.string)));
          }
      );
      new_prim (
          "string-slice",
          function (env)
          {
              let end = data_stack_pop (env);
              let begin = data_stack_pop (env);
              let a = data_stack_pop (env);
              data_stack_push (env, new string_t (
                  a.string.slice (begin.number, end.number)));
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
              if (this.type_name !== that.type_name)
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
              if (this.type_name !== that.type_name)
                  return false;
              else if (! (this.car.eq_p (that.car)))
                  return false;
              else if (! (this.cdr.eq_p (that.cdr)))
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
              data_stack_push (env, new_bool (
                  a.type_name === "null-t"));
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
              data_stack_push (env, new_bool (
                  a.type_name === "cons-t"));
          }
      );

      new_prim (
          "list-p",
          function (env)
          {
              let a = data_stack_pop (env);
              data_stack_push (env, new_bool (
                  ((a.type_name === "cons-t") ||
                   (a.type_name === "null-t"))));
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
                    print ("- code_scan fail")
                    print ("  doublequote mismatch")
                    print ("  string : {}".format(string))
                    error ()
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
            if ((char === '.') && (! (digital_char_p (next))))
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
    function sexp_repr (sexp)
    {
        if (null_p (sexp))
            return "null-c";
        else if (cons_p (sexp))
            return "(" +  sexp_list_repr (sexp) +  ")";
        else
            return sexp;
    }
    function sexp_list_repr (sexp_cons)
    {
        if (null_p (sexp_cons.cdr))
            return sexp_repr (sexp_cons.car);
        else {
            let car_repr = sexp_repr (sexp_cons.car);
            let cdr_repr = sexp_list_repr (sexp_cons.cdr);
            return car_repr + " " + cdr_repr;
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
        for (let sexp of sexp_vect) {
            top_sexp_eval (env, sexp);
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
            else if (top_macro_name_p (env, name)) {
                let den = name_dict_get (env, name);
                data_stack_push (env, sexp_list);
                den.den_exe (env);
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
    function top_macro_name_p (env, name)
    {
        let den = name_dict_get (env, name);
        if (! den)
            return false;
        if (den instanceof top_macro_den_t)
            return true;
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
      function pass_for_fun (sexp)
      {
          if (cons_p (sexp) &&
              (sexp.car === "+fun")) {
              let name = sexp.cdr.car;
              let body = sexp.cdr.cdr;
              body = substitute_recur (name, body);
              return cons_c ("+fun", cons_c (name, body));
          }
          else
              return sexp;
      }

      new_pass (pass_for_fun);
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
      function pass_for_field (sexp)
      {
          if (string_p (sexp)) {
              if (sexp.length <= 1)
                  return sexp;
              let pre_fix =
                  sexp.slice (0, 1);
              if (pre_fix === ".") {
                  sexp = sexp.slice (1, sexp.length);
                  sexp = cons_c (sexp, null_c ());
                  sexp = cons_c ("field", sexp);
                  return sexp;
              }
              else
                  return sexp;
          }
          else if (null_p (sexp)) {
              return null_c ();
          }
          else {
              return cons_c (pass_for_field (sexp.car),
                             pass_for_field (sexp.cdr));
          }
      }

      new_pass (pass_for_field);
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
        if (string_p (sexp)) {
            return string_compile (sexp);
        }
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
                print ("- sexp_compile fail");
                print ("  unknown name :", name);
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
        else if (sexp === ",")
            return [];
        // ><><><
        // drop dup over tuck swap
        else if (string_string_p (sexp)) {
            let string = string_string_to_string (sexp);
            return [new lit_exp_t (new string_t (string))];
        }
        else if (number_string_p (sexp)) {
            let number = number_string_to_number (sexp);
            return [new lit_exp_t (new number_t (number))];
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
        for (let x of number_string) {
            if (digital_char_p (x))
                return true;
            else if (x === ".")
                return true;
        }
        return false;
    }
    function number_string_to_number (number_string)
    {
        return eval (number_string);
    }
    function string_string_to_string (ss)
    {
        return ss.slice (1, ss.length -1);
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
