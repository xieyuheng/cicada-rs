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
      function assert (x) {
          if (! x) {
              throw new Error('assert fail!');
          }
      }
      function error ()
      {
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

        print ()
        {
            print (this.data_stack);
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
        //     print ("  env :", env);
        //     print ("  exp :", exp);
        //     print ("  scope :", scope);
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
    function exp_vect_to_obj_vect (env, exp_vect)
    {
        let mark = data_stack_length (env);
        exp_vect_run (env, exp_vect);
        let length = data_stack_length (env);
        let obj_vect = [];
        while (length > mark) {
           let obj = data_stack_pop (env);
           obj_vect.unshift (obj);
           length = length - 1;
        }
        return obj_vect;

    }
    function exp_vect_to_obj (env, exp_vect)
    {
        let obj_vect = exp_vect_to_obj_vect (env, exp_vect);
        assert (obj_vect.length === 1);
        return obj_vect[0];
    }
    function exp_to_obj (env, exp)
    {
        return exp_vect_to_obj (env, [exp]);
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
        constructor (exp_vect, case_clause_dict)
        {
            this.exp_vect = exp_vect;
            this.case_clause_dict = case_clause_dict;
        }

        exe (env, scope)
        {
            let obj = exp_vect_to_obj (env, exp_vect);
            assert (obj instanceof data_obj_t);
            let exp_vect = this.case_clause_dict.get (obj.type_name);
            if (exp_vect) {
                let frame = new simple_frame_t (exp_vect);
                frame_stack_push (env, frame);
            }
            else {
                let exp_vect = this.case_clause_dict.get ("else");
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
    class construct_exp_t
    {
        constructor (type_name)
        {
            this.type_name = type_name;
        }

        exe (env, scope)
        {
            let type_name = this.type_name;
            let type_den = name_dict_get (env, type_name);
            assert (type_den instanceof type_den_t);
            let field_dict = new field_dict_t ();
            for (let field_name of type_den.reversed_field_name_vect) {
                let obj = data_stack_pop (env);
                field_dict.set (field_name, obj)
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
        constructor (reversed_field_name_vect,
                     dot_clause_map)
        {
            this.reversed_field_name_vect
                = reversed_field_name_vect;
            this.dot_clause_map
                = dot_clause_map;
        }

        exe (env, scope)
        {
            let field_dict = new field_dict_t ();
            for (let field_name of this.reversed_field_name_vect) {
                let obj = data_stack_pop (env);
                field_dict.set (field_name, obj)
            }
            for (let [field_name, exp_vect] of this.dot_clause_map) {
                let obj = exp_vect_to_obj (exp_vect);
                field_dict.set (field_name, obj)
            }
            let dot_obj = new dot_obj_t (field_dict);
            data_stack_push (env, dot_obj);
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
            let dot_obj = data_stack_pop (env);
            assert (dot_obj instanceof dot_obj_t);
            let data_obj
                = new data_obj_t (
                    this.type_name,
                    dot_obj.field_dict);
            data_stack_push (env, data_obj);
        }
    }
    class clone_exp_t
    {
        constructor () { }

        exe (env, scope)
        {
            let data_obj = data_stack_pop (env);
            assert (data_obj instanceof data_obj_t);
            let dot_obj = data_stack_pop (env);
            assert (dot_obj instanceof dot_obj_t);
            let new_field_dict = new field_dict_t ();
            for (let [field_name, obj] of data_obj.field_dict) {
                new_field_dict.set (field_name, obj);
            }
            for (let [field_name, obj] of dot_obj.field_dict) {
                new_field_dict.set (field_name, obj);
            }
            let new_data_obj =
                new data_obj_t (
                    data_obj.type_name,
                    new_field_dict);
            data_stack_push (env, new_data_obj);
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
    class dot_obj_t
    {
        constructor (field_dict)
        {
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

        get (type_name_vect)
        {
            for (let [key, value] of this.dict) {
                if (type_name_vect_lteq_p
                    (env, type_name_vect, key))
                    return value;
            }
            return undefined;
        }

        set (type_name_vect, gene_den)
        {
            for (let key of this.dict.keys ()) {
                if (vect_eq_p (key, type_name_vect)) {
                    this.dict.set (key, gene_den);
                    return;
                }
            }
            this.dict.set (type_name_vect, gene_den)
        }
    }
    function type_name_vect_lteq_p (env, v1, v2)
    {
        for (let [t1, t2] of [v1, v2]) {
            if (type_name_lteq_p (env, t1, t2))
                return false;
        }
        return true;
    }
    function type_name_lteq_p (env, t1, t2)
    {
        if (t1 === t2)
            return true;
        let union_den = name_dict_get (env, t2);
        if (! union_den instanceof union_den_t)
            return false;
        if (t1 in union_den.union_vect)
            return true;
        else
            return false;
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
        constructor ()
        {
            this.dict = new Map ();
        }

        get (type_name_vect)
        {
            for (let [key, value] of this.dict) {
                if (vect_equal_p (type_name_vect, key))
                    return value;
            }
            return undefined;
        }

        set (type_name_vect, fun_den)
        {
            for (let key of this.dict.keys ()) {
                if (vect_equal_p (key, type_name_vect)) {
                    this.dict.set (key, fun_den);
                    return;
                }
            }
            this.dict.set (type_name_vect, fun_den)
        }
    }
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
            if (space_p (char) ||
                delimiter_p (char) ||
                (char === '.') ||
                (char === '"'))
                return i - 1;
            else
                i = i + 1;
        }
    }
    function null_p (x)
    {
        return x === null;
    }
    class cons_t
    {
        constructor (car, cdr)
        {
            this.car = car;
            this.cdr = cdr;
        }
    }
    function cons (car, cdr)
    {
        assert (list_p (cdr));
        return new cons_t (car, cdr);
    }
    function cons_p (x)
    {
        return x instanceof cons_t;
    }
    function car (c)
    {
        assert (cons_p (c));
        return c.car;
    }
    function cdr (c)
    {
        assert (cons_p (c));
        return c.cdr;
    }
    function list_p (x)
    {
        return (null_p (x) || cons_p (x));
    }
    function parse_sexp_vect (string_vect)
    {
        let length = string_vect.length;
        let i = 0;
        let sexp_vect = [];
        while (i < length) {
            let v = parse_sexp (string_vect, i);
            let s = v[0];
            i = v[1];
            sexp_vect.push (s);
        }
        return sexp_vect;
    }
    function parse_sexp (string_vect, i)
    {
        let string = string_vect[i];
        if (string === '(')
            return parse_sexp_cons_until_ket (string_vect, i+1, ')');
        else if (string === '[') {
            let v = parse_sexp_cons_until_ket (string_vect, i+1, ']');
            let sc = v[0];
            let i1 = v[1];
            return [cons ('begin', sc), i1];
        }
        else if (string === '{') {
            let v = parse_sexp_cons_until_ket (string_vect, i+1, '}');
            let sc = v[0];
            let i1 = v[1];
            return [cons ('clo', sc), i1];
        }
        else if (string === "'") {
            let v = parse_sexp (string_vect, i+1);
            let s = v[0];
            let i1 = v[1];
            let sc = cons (s, null);
            return [cons ('quote', cs), i1];
        }
        else if (string === "`") {
            let v = parse_sexp (string_vect, i+1);
            let s = v[0];
            let i1 = v[1];
            let sc = cons (s, null);
            return [cons ('partquote', cs), i1];
        }
        else
            return [string, i+1];
    }
    function parse_sexp_cons_until_ket (string_vect, i, ket)
    {
        let string = string_vect[i];
        if (string == ket)
            return [null, i+1];
        else {
            let v = parse_sexp (string_vect, i);
            let s = v[0];
            let i1 = v[1];
            let v2 =
                parse_sexp_cons_until_ket (string_vect, i1, ket);
            let sc = v2[0];
            let i2 = v2[1];
            return [cons (s, sc), i2];
        }
    }
    function sexp_repr (sexp)
    {
        if (null_p (sexp))
            return "null";
        else if (cons_p (sexp))
            return "(" +  sexp_list_repr (sexp) +  ")";
        else
            return sexp;
    }
    function sexp_list_repr (sexp_cons)
    {
        if (null_p (cdr (sexp_cons)))
            return sexp_repr (car (sexp_cons));
        else {
            let car_repr = sexp_repr (car (sexp_cons));
            let cdr_repr = sexp_list_repr (cdr (sexp_cons));
            return car_repr + " " + cdr_repr;
        }
    }
    function sexp_list_to_vect (sexp_list)
    {
        if (null_p (sexp_list))
            return [];
        else {
            let sexp = car (sexp_list);
            let sexp_vect = [sexp];
            let rest_list = (cdr (sexp_list));
            return sexp_vect.concat (sexp_list_to_vect (rest_list));
        }
    }
    function sexp_vect_to_list (sexp_vect)
    {
        if (vect_empty_p (sexp_vect))
            return null;
        else {
            let sexp = sexp_vect [0];
            let rest_list = sexp_vect_to_list (sexp_vect.slice (1));
            return cons (sexp, rest_list);
        }
    }
    function code_eval (env, code)
    {
        let string_vect = code_scan (code);
        let sexp_vect = parse_sexp_vect (string_vect);
        sexp_vect_eval (env, sexp_vect);
    }
    function sexp_vect_eval (env, sexp_vect)
    {
        for (let sexp of sexp_vect) {
            sexp_eval (env, sexp);
        }
    }
    function sexp_eval (env, sexp)
    {
        assert (cons_p (sexp));
        sexp = apply_all_passes (sexp);
        let keyword = car (sexp);
        let sexp_list = cdr (sexp);
        top_keyword_apply (env, keyword, sexp_list);
    }
    let pass_vect = [];
    function new_pass (pass_fn)
    {
        pass_vect.push (pass_fn);
    }
    function apply_all_passes (sexp)
    {
        for (let pass_fn of pass_vect) {
            assert (pass_fn instanceof Function);
            sexp = pass_fn (sexp);
        }
        return sexp;
    }
      function pass_for_construct (sexp)
      {
          if (string_p (sexp)) {
              if (sexp.length <= 2)
                  return sexp
              let post_fix =
                  sexp.slice (sexp.length -2,
                              sexp.length);
              if (post_fix === "-c") {
                  sexp = sexp.slice (0, sexp.length -2);
                  sexp = sexp.concat ("-t");
                  sexp = cons (sexp, null);
                  sexp = cons ("construct", sexp);
                  return sexp;
              }
              else
                  return sexp;

          }
          else if (null_p (sexp)) {
              return null;
          }
          else {
              return cons (pass_for_construct (car (sexp)),
                           pass_for_construct (cdr (sexp)));
          }
      }

      new_pass (pass_for_construct);
      function field_name_p (x)
      {
          if (! string_p (x))
              return false;
          else if (x[0] !== '.')
              return false;
          else
              return true;
      }
    function sexp_list_compile (sexp_list)
    {
        let sexp_vect = sexp_list_to_vect (sexp_list);
        let exp_vect = [];
        for (let sexp of sexp_vect) {
            exp_vect = exp_vect.concat (sexp_compile (sexp));
        }
        return exp_vect;
    }
    function sexp_compile (sexp)
    {
        if (string_p (sexp)) {
            if (sexp === "apply")
                return [new apply_exp_t ()];
            else if (sexp === "clone")
                return [new clone_exp_t ()];
            // ><><><
            // drop dup over tuck swap
            else {
                let name = sexp;
                let call_exp = new call_exp_t (name);
                return [call_exp];
            }
        }
        else {
            assert (cons_p (sexp));
            let keyword = car (sexp);
            let rest_list = cdr (sexp);
            let new_exp_vect =
                keyword_apply (keyword, rest_list);
            assert (vect_p (new_exp_vect));
            return new_exp_vect;
        }
    }
    let keyword_dict = new Map ();
    function new_keyword (keyword, keyword_fn)
    {
        keyword_dict.set (keyword, keyword_fn);
    }
    function top_keyword_apply (env, keyword, sexp_list)
    {
        let top_keyword_fn = keyword_dict.get (keyword);
        assert (top_keyword_fn instanceof Function);
        top_keyword_fn (env, sexp_list);
    }
    function keyword_apply (keyword, sexp_list)
    {
        let keyword_fn = keyword_dict.get (keyword);
        assert (keyword_fn instanceof Function);
        return keyword_fn (sexp_list);
    }
    new_keyword (
        "+union",
        function (env, sexp_list)
        {
            let name = car (sexp_list);
            let rest_list = cdr (sexp_list);
            let union_vect = [];
            let rest_vect = sexp_list_to_vect (rest_list);
            for (let type_name of rest_vect) {
                union_vect.push (type_name);
            }
            let union_den = new union_den_t (union_vect);
            name_dict_set (env, name, union_den);
        }
    );
    new_keyword (
        "+data",
        function (env, sexp_list)
        {
            let name = car (sexp_list);
            let rest_list = cdr (sexp_list);
            let rest_vect = sexp_list_to_vect (rest_list);
            let reversed_field_name_vect = [];
            for (let field_name of rest_vect) {
                reversed_field_name_vect.unshift (field_name);
            }
            let type_den =
                new type_den_t (reversed_field_name_vect);
            name_dict_set (env, name, type_den);
        }
    );
    new_keyword (
        "+fun",
        function (env, sexp_list)
        {
            let name = car (sexp_list);
            let rest_list = cdr (sexp_list);
            let exp_vect = sexp_list_compile (rest_list);
            let fun_den = new fun_den_t (exp_vect);
            name_dict_set (env, name, fun_den);
        }
    );
    new_keyword (
        "+gene",
        function (env, sexp_list)
        {
            let name = car (sexp_list);
            let rest_list = cdr (sexp_list);
            let exp_vect = sexp_list_compile (rest_list);
            let fun_den = new fun_den_t (exp_vect);
            name_dict_set (env, name, fun_den);
        }
    );
    new_keyword (
        "+disp",
        function (env, sexp_list)
        {
            let name = car (sexp_list);
            let rest_list = cdr (sexp_list);
            let exp_vect = sexp_list_compile (rest_list);
            let fun_den = new fun_den_t (exp_vect);
            name_dict_set (env, name, fun_den);
        }
    );
    new_keyword (
        "main",
        function (env, sexp_list)
        {
            let exp_vect = sexp_list_compile (sexp_list);
            exp_vect_run (env, exp_vect);
        }
    );
    new_keyword (
        ".",
        function (sexp_list)
        {

        }
    );
    new_keyword (
        "construct",
        function (sexp_list)
        {
            let name = car (sexp_list);
            return [new construct_exp_t (name)];
        }
    );
    new_keyword (
        "create",
        function (sexp_list)
        {
            let name = car (sexp_list);
            return [new create_exp_t (name)];
        }
    );
    function test_env ()
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
        exp_vect_run (env, [
            new call_exp_t ("dup"),
        ]);
        print (env);
    }

    // test_env ();
    function test_code_scan ()
    {
        let code = "                                    \
        (+fun ref                                       \
          : (-> l : [:t list-u], index : nat-u -- :t)   \
          (case index                                   \
            (zero-t l.car)                              \
            (succ-t l.cdr index.prev recur)))           \
        ";
        let string_vect = code_scan (code + code);
        print (code);
        print (string_vect);
    }

    // test_code_scan ();
    function test_parse_sexp_vect ()
    {
        let code = "                                    \
        (+fun ref                                       \
          : (-> l : [:t list-u], index : nat-u -- :t)   \
          (case index                                   \
            (zero-t l.car)                              \
            (succ-t l.cdr index.prev recur)))           \
        ";
        let string_vect = code_scan (code + code);
        let sexp_vect = parse_sexp_vect (string_vect);
        for (let sexp of sexp_vect) {
            print (sexp_repr (sexp));
        }
    }

    // test_parse_sexp_vect ();
    function eval_code (code)
    {
        assert (string_p (code));
        let env = new env_t ();
        code_eval (env, code);
        return env;
    }
    module.exports.eval_code = eval_code;
