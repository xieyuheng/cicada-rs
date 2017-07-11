  #include <sys/types.h>
  #include <sys/stat.h>
  #include <unistd.h>
  #include <stdio.h>
  #include <stdlib.h>
  #include <errno.h>
  #include <string.h>
  #include <fcntl.h>
  #include <ctype.h>
  #include <stdint.h>
  #include <dlfcn.h>
  #include <dirent.h>
  #include <signal.h>
  #include <limits.h>
  typedef enum { false, true } bool;
  typedef intptr_t cell;
  typedef unsigned char byte;
  typedef void (*primitive)();

  #define CELL_SIZE (sizeof(cell))
    cell max(cell a, cell b) {
      if (a < b) {
        return b;
      }
      else {
        return a;
      }
    }

    cell min(cell a, cell b) {
      if (a > b) {
        return b;
      }
      else {
        return a;
      }
    }

    cell power(cell a, cell n) {
      cell result = 1;
      while (n >= 1) {
        result = result * a;
        n--;
      }
      return result;
    }
    bool isbarcket(char c) {
      return (c == '(' ||
              c == ')' ||
              c == '[' ||
              c == ']' ||
              c == '{' ||
              c == '}' ||
              c == '"');
    }
    cell char_to_nat(char c) {
      if (c >= '0' && c <= '9') {
        return (c - '0');
      }
      else if (c >= 'A' && c <= 'Z') {
        return (c - 'A') + 10;
      }
      else if (c >= 'a' && c <= 'z') {
        return (c - 'a') + 10;
      }
      else {
        return 0;
      }
    }
      bool string_equal(char* s1, char* s2) {
        if (strcmp(s1, s2) == 0) {
          return true;
        }
        else {
          return false;
        }
      }
      bool nat_string_p(char* str) {
        cell i = 0;
        while (str[i] != 0) {
          if (!isdigit(str[i])) {
            return false;
            }
          i++;
        }
        return true;
      }
      bool int_string_p(char* str) {
        if (str[0] == '-' ||
            str[0] == '+') {
          return nat_string_p(str + 1);
        }
        else {
          return nat_string_p(str);
        }
      }
      cell string_to_based_nat(char* str, cell base) {
        cell result = 0;
        cell len = strlen(str);
        cell i = 0;
        while (i < len) {
          result = result + (char_to_nat(str[i]) * power(base, (len - i - 1)));
          i++;
        }
        return result;
      }

      cell string_to_based_int(char* str, cell base) {
        if (str[0] == '-') {
          return - string_to_based_nat(str, base);
        }
        else {
          return string_to_based_nat(str, base);
        }
      }

      cell string_to_int(char* str) { return string_to_based_int(str, 10); }
    char string_area[1024 * 1024];
    cell string_area_counter = 0;
    char* copy_to_string_area(char* str) {
      char *str1;
      cell i = 0;
      str1 = (string_area + string_area_counter);
      while (true) {
        if (str[i] == 0) {
          str1[i] = str[i];
          i++;
          break;
        }
        else {
          str1[i] = str[i];
          i++;
        }
      }
      string_area_counter = i + string_area_counter;
      return str1;
    }
    typedef struct jo_entry {
      char *key;
      struct jo_entry *tag;
      cell value;
    } jotable_entry;

    typedef jotable_entry *jo;

    // prime table size
    //   1000003   about 976 k
    //   1000033
    //   1000333
    //   100003    about 97 k
    //   100333
    //   997
    #define jotable_size 100003
    jotable_entry jotable[jotable_size];

    // thus (jotable + index) is jo
    bool used_jo_p(jo jo) {
      return jo->tag != 0;
    }
    cell string_to_sum(char* str) {
      cell sum = 0;
      cell max_step = 10;
      cell i = 0;
      while (i < strlen(str)) {
        sum = sum + ((byte) str[i]) * (2 << min(i, max_step));
        i++;
      }
      return sum;
    }
    // a hash an index into jotable
    cell jotable_hash(cell sum, cell counter) {
      return (counter + sum) % jotable_size;
    }
    p_debug();

    jo jotable_insert(char* key) {
      // in C : [string] -> [jo]
      cell sum = string_to_sum(key);
      cell counter = 0;
      while (true) {
        cell index = jotable_hash(sum, counter);
        jo jo = (jotable + index);
        if (jo->key == 0) {
          key = copy_to_string_area(key);
          jo->key = key;
          return jo;
        }
        else if (string_equal(key, jo->key)) {
          return jo;
        }
        else if (counter == jotable_size) {
          printf("- jotable_insert fail\n");
          printf("  the hash_table is filled\n");
          p_debug();
          return NULL;
        }
        else {
          counter++;
        }
      }
    }
    jo str2jo(char* str) {
      return jotable_insert(str);
    }
    char* jo2str(jo jo) {
      return jo->key;
    }
    jo EMPTY_JO;
    jo TAG_PRIM;
    jo TAG_JOJO;
    jo TAG_PRIM_KEYWORD;
    jo TAG_KEYWORD;
    jo TAG_DATA;

    jo JO_DECLARED;

    jo ROUND_BAR;
    jo ROUND_KET;
    jo SQUARE_BAR;
    jo SQUARE_KET;
    jo FLOWER_BAR;
    jo FLOWER_KET;
    jo DOUBLE_QUOTE;

    jo JO_INS_INT;
    jo JO_INS_JO;
    jo JO_INS_STRING;
    jo JO_INS_BYTE;
    jo JO_INS_BARE_JOJO;
    jo JO_INS_ADDRESS;

    jo JO_INS_JUMP;
    jo JO_INS_JUMP_IF_FALSE;

    jo JO_INS_TAIL_CALL;
    jo JO_INS_LOOP;
    jo JO_INS_RECUR;

    jo JO_NULL;
    jo JO_THEN;
    jo JO_ELSE;

    jo JO_APPLY;
    jo JO_END;

    jo JO_LOCAL_DATA_IN;
    jo JO_LOCAL_DATA_OUT;

    jo JO_LOCAL_TAG_IN;
    jo JO_LOCAL_TAG_OUT;

    jo JO_LOCAL_IN;
    jo JO_LOCAL_OUT;
    typedef jo* compiling_stack_t[1024];

    compiling_stack_t compiling_stack;
    cell compiling_stack_base = 0;
    cell compiling_stack_pointer = 0;

    compiling_stack_push(jo* value) {
      compiling_stack[compiling_stack_pointer] = value;
      compiling_stack_pointer++;
    }

    jo* compiling_stack_pop() {
      compiling_stack_pointer--;
      return compiling_stack[compiling_stack_pointer];
    }

    compiling_stack_inc() {
      compiling_stack[compiling_stack_pointer - 1] =
        compiling_stack[compiling_stack_pointer - 1] + 1;
    }


    jo* compiling_stack_tos() {
      return compiling_stack[compiling_stack_pointer - 1];
    }

    bool compiling_stack_empty_p() {
      return compiling_stack_pointer == compiling_stack_base;
    }
    here(cell n) {
      jo* pointer = compiling_stack_pop();
      pointer[0] = n;
      compiling_stack_push(pointer + 1);
    }
    jo jojo_area[1024 * 1024];

    init_compiling_stack() {
      compiling_stack_push(jojo_area);
    }
    typedef cell data_stack_t[1024 * 4];

    data_stack_t data_stack;
    cell data_stack_base = 64;
    cell data_stack_pointer = 64;

    data_stack_push(cell value) {
      data_stack[data_stack_pointer] = value;
      data_stack_pointer++;
    }

    void* data_stack_pop() {
      data_stack_pointer--;
      return data_stack[data_stack_pointer];
    }

    cell data_stack_tos() {
      return data_stack[data_stack_pointer - 1];
    }

    bool data_stack_empty_p() {
      return data_stack_base == data_stack_pointer;
    }
    typedef struct {
      jo name;
      cell local_tag;
      cell local_data;
    } local_point;

    local_point local_area[1024 * 1024];
    cell current_local_pointer = 0;
    typedef struct {
      jo* jojo;
      cell local_pointer;
    } return_point;

    typedef return_point return_stack_t[1024 * 4];

    return_stack_t return_stack;
    cell return_stack_base = 64;
    cell return_stack_pointer = 64;

    return_stack_push(return_point value) {
      return_stack[return_stack_pointer] = value;
      return_stack_pointer++;
    }

    return_point return_stack_pop() {
      return_stack_pointer--;
      return return_stack[return_stack_pointer];
    }

    return_point return_stack_tos() {
      return return_stack[return_stack_pointer - 1];
    }

    bool return_stack_empty_p() {
      return return_stack_base == return_stack_pointer;
    }

    return_stack_make_point(jo* jojo, cell local_pointer) {
      return_point rp = {.jojo = jojo, .local_pointer = local_pointer};
      return_stack[return_stack_pointer] = rp;
      return_stack_pointer++;
    }

    return_stack_new_point(jo* jojo) {
      return_stack_make_point(jojo, current_local_pointer);
    }

    return_stack_inc() {
      return_point rp = return_stack_pop();
      return_point rp1 = {.jojo = rp.jojo + 1,
                          .local_pointer = rp.local_pointer};
      return_stack_push(rp1);
    }
    jo name_record[16 * 1024];
    cell name_record_counter = 0;
    p_name_record() {
      data_stack_push(name_record);
    }
    p_name_report() {
      printf("- p_name_report // counter : %ld\n", name_record_counter);
      cell i = 0;
      while (i < name_record_counter) {
        printf("  %s\n", jo2str(name_record[i]));
        i++;
      }
      printf("\n");
    }
      typedef jo binding_filter_stack_t[128];
      binding_filter_stack_t binding_filter_stack;

      cell binding_filter_stack_base = 0;
      cell binding_filter_stack_pointer = 0;

      binding_filter_stack_push(jo value) {
        binding_filter_stack[binding_filter_stack_pointer] = value;
        binding_filter_stack_pointer++;
      }

      jo binding_filter_stack_pop() {
        binding_filter_stack_pointer--;
        return binding_filter_stack[binding_filter_stack_pointer];
      }

      jo binding_filter_stack_tos() {
        return binding_filter_stack[binding_filter_stack_pointer - 1];
      }

      bool binding_filter_stack_empty_p() {
        return binding_filter_stack_pointer == binding_filter_stack_base;
      }
      p_binding_filter_stack_push() {
        binding_filter_stack_push(data_stack_pop());
      }
      p_binding_filter_stack_pop() {
        data_stack_push(binding_filter_stack_pop());
      }
      jo_apply_now(jo jo);

      run_binding_filter() {
        // [name] -> [name]
        cell i = binding_filter_stack_pointer;
        while (i > binding_filter_stack_base) {
          jo_apply_now(binding_filter_stack[i-1]);
          i--;
        }
      }
    bool name_can_bind_p(jo name) {
      if (name->tag == JO_DECLARED) {
        return true;
      }
      else if (used_jo_p(name)) {
        return false;
      }
      else {
        return true;
      }
    }
    p_rebind_name() {
      // [data tag name] -> {set-jotable}
      run_binding_filter();

      jo name = data_stack_pop();
      jo tag = data_stack_pop();
      cell data = data_stack_pop();

      if (!used_jo_p(name)) {
        name_record[name_record_counter] = name;
        name_record_counter++;
        name_record[name_record_counter] = 0;
      }

      name->tag = tag;
      name->value = data;
    }
    p_bind_name() {
      // [data tag name] -> {set-jotable}
      run_binding_filter();

      jo name = data_stack_pop();
      jo tag = data_stack_pop();
      cell data = data_stack_pop();

      if (!name_can_bind_p(name)) {
        printf("- p_bind_name can not rebind\n");
        printf("  name : %s\n", jo2str(name));
        printf("  tag : %s\n", jo2str(tag));
        printf("  data : %ld\n", data);
        printf("  it has been bound as a %s\n", jo2str(name->tag));
        return;
      }

      name_record[name_record_counter] = name;
      name_record_counter++;
      name_record[name_record_counter] = 0;

      name->tag = tag;
      name->value = data;
    }
    define_prim(char* str, primitive fun) {
      jo name = str2jo(str);
      data_stack_push(fun);
      data_stack_push(TAG_PRIM);
      data_stack_push(name);
      p_bind_name();
    }
    define_primkey(char* str, primitive fun) {
      jo name = str2jo(str);
      data_stack_push(fun);
      data_stack_push(TAG_PRIM_KEYWORD);
      data_stack_push(name);
      p_bind_name();
    }
    expose_bind() {
      define_prim("name-report", p_name_report);
      define_prim("name-record", p_name_record);

      define_prim("binding-filter-stack-push", p_binding_filter_stack_push);
      define_prim("binding-filter-stack-pop", p_binding_filter_stack_pop);
    }
    typedef jo keyword_stack_t[128];
    keyword_stack_t keyword_stack;

    cell keyword_stack_base = 0;
    cell keyword_stack_pointer = 0;

    keyword_stack_push(jo value) {
      keyword_stack[keyword_stack_pointer] = value;
      keyword_stack_pointer++;
    }

    jo keyword_stack_pop() {
      keyword_stack_pointer--;
      return keyword_stack[keyword_stack_pointer];
    }

    jo keyword_stack_tos() {
      return keyword_stack[keyword_stack_pointer - 1];
    }

    bool keyword_stack_empty_p() {
      return keyword_stack_pointer == keyword_stack_base;
    }
    typedef struct {
      jo nick;
      jo name;
    } alias;
    typedef alias alias_stack_t[1024];
    alias_stack_t alias_stack;

    cell alias_stack_base = 0;
    cell alias_stack_pointer = 0;

    alias_stack_push(alias value) {
      alias_stack[alias_stack_pointer] = value;
      alias_stack_pointer++;
    }

    alias alias_stack_pop() {
      alias_stack_pointer--;
      return alias_stack[alias_stack_pointer];
    }

    alias alias_stack_tos() {
      return alias_stack[alias_stack_pointer - 1];
    }

    bool alias_stack_empty_p() {
      return alias_stack_pointer == alias_stack_base;
    }
    eval();
    p_debug();

    jo_apply(jo jo) {
      if (!used_jo_p(jo)) {
        printf("- jo_apply meet undefined jo : %s\n", jo2str(jo));
        p_debug();
        return;
      }

      cell tag = jo->tag;

      if (tag == TAG_PRIM) {
        primitive primitive = jo->value;
        primitive();
      }
      else if (tag == TAG_JOJO) {
        cell jojo = jo->value;
        return_stack_new_point(jojo);
      }

      else if (tag == TAG_PRIM_KEYWORD) {
        keyword_stack_push(alias_stack_pointer);
        primitive primitive = jo->value;
        primitive();
        alias_stack_pointer = keyword_stack_pop();
      }
      else if (tag == TAG_KEYWORD) {
        // keywords are always evaled
        keyword_stack_push(alias_stack_pointer);
        cell jojo = jo->value;
        return_stack_new_point(jojo);
        eval();
        alias_stack_pointer = keyword_stack_pop();
      }

      else if (tag == TAG_DATA) {
        cell cell = jo->value;
        data_stack_push(cell);
      }
      else {
        cell cell = jo->value;
        data_stack_push(cell);
        data_stack_push(tag);
      }
    }
    jo_apply_now(jo jo) {
      cell tag = jo->tag;
      if (tag == TAG_JOJO) {
        cell jojo = jo->value;
        return_stack_new_point(jojo);
        eval();
        return;
      }
      else {
        jo_apply(jo);
        return;
      }
    }
    jo_apply_with_local_pointer(jo jo, cell local_pointer) {
      cell tag = jo->tag;
      if (tag == TAG_JOJO) {
        cell jojo = jo->value;
        return_stack_make_point(jojo, local_pointer);
        return;
      }
      else {
        jo_apply(jo);
        return;
      }
    }
    bool step_flag = false;

    stepper();

    eval() {
      cell return_stack_base = return_stack_pointer;
      while (return_stack_pointer >= return_stack_base) {
        return_point rp = return_stack_tos();
        return_stack_inc();
        jo* jojo = rp.jojo;
        jo jo = jojo[0];
        jo_apply(jo);
        if (step_flag == true) {
          stepper();
        }
      }
    }
    p_apply() {
      return_stack_new_point(data_stack_pop());
    }
    p_apply_with_local_pointer() {
      // [local_pointer jojo] -> [*]
      jo* jojo = data_stack_pop();
      cell local_pointer = data_stack_pop();
      return_stack_make_point(jojo, local_pointer);
    }
    p_jo_apply() {
      jo_apply(data_stack_pop());
    }
    p_jo_apply_with_local_pointer() {
      jo jo = data_stack_pop();
      cell local_pointer = data_stack_pop();
      jo_apply_with_local_pointer(jo, local_pointer);
    }
    expose_apply() {
      define_prim("apply", p_apply);
      define_prim("apply-with-local-pointer", p_apply_with_local_pointer);

      define_prim("jo/apply", p_jo_apply);
      define_prim("jo/apply-with-local-pointer", p_jo_apply_with_local_pointer);
    }
    p_drop() {
      data_stack_pop();
    }
    p_2drop() {
      data_stack_pop();
      data_stack_pop();
    }
    p_dup() {
      // a a -> a
      cell a = data_stack_pop();
      data_stack_push(a);
      data_stack_push(a);
    }
    p_2dup() {
      // b a -> b a b a
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(b);
      data_stack_push(a);
      data_stack_push(b);
      data_stack_push(a);
    }
    p_over() {
      // b a -> b a b
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(b);
      data_stack_push(a);
      data_stack_push(b);
    }
    p_2over() {
      // d c  b a -> d c  b a  d c
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      cell c = data_stack_pop();
      cell d = data_stack_pop();
      data_stack_push(d);
      data_stack_push(c);
      data_stack_push(b);
      data_stack_push(a);
      data_stack_push(d);
      data_stack_push(c);
    }
    p_tuck() {
      // b a -> a b a
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(a);
      data_stack_push(b);
      data_stack_push(a);
    }
    p_2tuck() {
      // d c  b a -> b a  d c  b a
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      cell c = data_stack_pop();
      cell d = data_stack_pop();
      data_stack_push(b);
      data_stack_push(a);
      data_stack_push(d);
      data_stack_push(c);
      data_stack_push(b);
      data_stack_push(a);
    }
    p_swap() {
      // b a -> a b
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(a);
      data_stack_push(b);
    }
    p_2swap() {
      // d c  b a -> b a  d c
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      cell c = data_stack_pop();
      cell d = data_stack_pop();
      data_stack_push(b);
      data_stack_push(a);
      data_stack_push(d);
      data_stack_push(c);
    }
    p_print_data_stack() {
      // {terminal-output}
      if (data_stack_pointer < data_stack_base) {
        printf("  * %ld *  ", (data_stack_pointer - data_stack_base));
        printf("-- below the stack --\n");
      }
      else {
        printf("  * %ld *  ", (data_stack_pointer - data_stack_base));
        printf("-- ");
        cell i = data_stack_base;
        while (i < data_stack_pointer) {
          printf("%ld ", data_stack[i]);
          i++;
        }
        printf("--\n");
      }
    }
    p_stack_base() {
      data_stack_push(data_stack + data_stack_base);
    }
    p_stack_pointer() {
      data_stack_push(data_stack + data_stack_pointer);
    }
    expose_stack_operation() {
      define_prim("drop", p_drop);
      define_prim("2drop", p_2drop);
      define_prim("dup", p_dup);
      define_prim("2dup", p_2dup);
      define_prim("over", p_over);
      define_prim("2over", p_2over);
      define_prim("tuck", p_tuck);
      define_prim("2tuck", p_2tuck);
      define_prim("swap", p_swap);
      define_prim("2swap", p_2swap);
      define_prim("print-data-stack", p_print_data_stack);
      define_prim("stack-pointer", p_stack_pointer);
      define_prim("stack-base", p_stack_base);
    }
    p_end() {
      return_point rp = return_stack_pop();
      current_local_pointer = rp.local_pointer;
    }
    p_bye() {
      printf("bye bye ^-^/\n");
      exit(0);
    }
    expose_ending() {
      define_prim("end", p_end);
      define_prim("bye", p_bye);
    }
    p_true() {
      data_stack_push(true);
    }
    p_false() {
      data_stack_push(false);
    }
    p_not() {
      // bool -> bool
      cell a = data_stack_pop();
      data_stack_push(!a);
    }
    p_and() {
      // bool bool -> bool
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(a&&b);
    }
    p_or() {
      // bool bool -> bool
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(a||b);
    }
    expose_bool() {
      define_prim("true", p_true);
      define_prim("false", p_false);
      define_prim("not", p_not);
      define_prim("and", p_and);
      define_prim("or", p_or);
    }
    p_inc() {
      cell a = data_stack_pop();
      data_stack_push(a + 1);
    }
    p_dec() {
      cell a = data_stack_pop();
      data_stack_push(a - 1);
    }
    p_neg() {
      cell a = data_stack_pop();
      data_stack_push(- a);
    }
    p_add() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a + b);
    }
    p_sub() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a - b);
    }
    p_mul() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a * b);
    }
    p_div() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a / b);
    }
    p_mod() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a % b);
    }
    p_n_eq_p() {
      // a ... b ... n -> bool
      cell n = data_stack_pop();
      cell old_n = n;
      cell* cursor1 = (data_stack + data_stack_pointer - n);
      cell* cursor2 = (data_stack + data_stack_pointer - n - n);
      while (n > 0) {
        if (cursor1[n-1] != cursor2[n-1]) {
          data_stack_pointer = data_stack_pointer - old_n - old_n;
          data_stack_push(false);
          return;
        }
        n--;
      }
      data_stack_pointer = data_stack_pointer - old_n - old_n;
      data_stack_push(true);
    }
    p_eq_p() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a == b);
    }
    p_gt_p() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a > b);
    }
    p_lt_p() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a < b);
    }
    p_gteq_p() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a >= b);
    }
    p_lteq_p() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a <= b);
    }
    i_int() {
      // [] -> [cell] {return_stack}
      return_point rp = return_stack_tos();
      return_stack_inc();
      jo* jojo = rp.jojo;
      jo jo = jojo[0];
      data_stack_push(jo);
    }
    jo read_raw_jo();

    k_int() {
      // (int ...)
      while (true) {
        jo s = read_raw_jo();
        if (s == ROUND_KET) {
          break;
        }
        else {
          here(JO_INS_INT);
          here(string_to_int(jo2str(s)));
        }
      }
    }
    p_int_print() { printf("%ld", data_stack_pop()); }
    p_dot() { printf("%ld ", data_stack_pop()); }
    p_int_dot() { printf("%ld ", data_stack_pop()); }
    expose_int() {
      define_prim("inc", p_inc);
      define_prim("dec", p_dec);
      define_prim("neg", p_neg);

      define_prim("add", p_add);
      define_prim("sub", p_sub);

      define_prim("mul", p_mul);
      define_prim("div", p_div);
      define_prim("mod", p_mod);

      define_prim("n-eq?", p_n_eq_p);

      define_prim("eq?", p_eq_p);
      define_prim("gt?", p_gt_p);
      define_prim("lt?", p_lt_p);
      define_prim("gteq?", p_gteq_p);
      define_prim("lteq?", p_lteq_p);

      define_prim("ins/int", i_int);
      define_primkey("int", k_int);

      define_prim("int/print", p_int_print);

      define_prim("dot", p_dot);
      define_prim("int/dot", p_int_dot);
    }
    p_allocate () {
      // size -> addr
      data_stack_push(calloc(data_stack_pop(), 1));
    }
    p_free () {
      // addr ->
      free(data_stack_pop());
    }
    k_ignore();

    k_address() {
      // (address ...)
      here(JO_INS_ADDRESS);
      jo name = read_raw_jo();
      here(&(name->value));
      k_ignore();
    }
    p_jo_as_var() {
      jo jo = data_stack_pop();
      data_stack_push(&(jo->value));
    }
    p_set_cell() {
      // cell address ->
      cell* address = data_stack_pop();
      cell value = data_stack_pop();
      address[0] = value;
    }
    p_get_cell() {
      // address -> cell
      cell* address = data_stack_pop();
      data_stack_push(address[0]);
    }
    p_set_byte() {
      // byte address ->
      char* address = data_stack_pop();
      cell value = data_stack_pop();
      address[0] = value;
    }
    p_get_byte() {
      // address -> byte
      char* address = data_stack_pop();
      data_stack_push(address[0]);
    }
    expose_memory() {
      define_prim("allocate", p_allocate);
      define_prim("free", p_free);

      define_prim("ins/address", i_int);
      define_primkey("address", k_address);

      define_prim("jo-as-var", p_jo_as_var);
      define_prim("set-cell", p_set_cell);
      define_prim("get-cell", p_get_cell);
      define_prim("set-byte", p_set_byte);
      define_prim("get-byte", p_get_byte);
    }
    typedef FILE* reading_stack_t[64];

    reading_stack_t reading_stack;
    cell reading_stack_base = 0;
    cell reading_stack_pointer = 0;

    reading_stack_push(FILE* value) {
      reading_stack[reading_stack_pointer] = value;
      reading_stack_pointer++;
    }

    FILE* reading_stack_pop() {
      reading_stack_pointer--;
      return reading_stack[reading_stack_pointer];
    }

    FILE* reading_stack_tos() {
      return reading_stack[reading_stack_pointer - 1];
    }

    bool reading_stack_empty_p() {
      return reading_stack_pointer == reading_stack_base;
    }
    erase_real_path_to_dir(char* path) {
      cell cursor = strlen(path);
      while (path[cursor] != '/') {
        path[cursor] = '\0';
        cursor--;
      }
      path[cursor] = '\0';
    }

    char* get_real_reading_path(char* path) {
      // caller of this function
      // should free its return value
      char* real_reading_path = malloc(PATH_MAX);
      if (path[0] == '/' ||
          reading_stack_empty_p() ||
          reading_stack_tos() == stdin) {
        realpath(path, real_reading_path);
        return real_reading_path;
      }
      else {
        char* proc_link_path = malloc(PATH_MAX);
        sprintf(proc_link_path, "/proc/self/fd/%d", fileno(reading_stack_tos()));
        ssize_t real_bytes = readlink(proc_link_path, real_reading_path, PATH_MAX);
        if (real_bytes == -1) {
          printf("- get_real_reading_path fail to readlink\n");
          printf("  proc_link_path : %s\n", proc_link_path);
          perror("  readlink : ");
          free(proc_link_path);
          free(real_reading_path);
          p_debug();
          return NULL; // to fool the compiler
        }
        free(proc_link_path);
        real_reading_path[real_bytes] = '\0';
        erase_real_path_to_dir(real_reading_path);
        strcat(real_reading_path, "/");
        strcat(real_reading_path, path);
        return real_reading_path;
      }
    }
    bool has_byte_p() {
    FILE* file;
      if (reading_stack_empty_p()) {
        file = stdin;
      }
      else {
        file = reading_stack_tos();
      }

      if (feof(file) == 0) {
        return true;
      }
      else {
        return false;
      }
    }
    p_has_byte_p() {
      data_stack_push(has_byte_p());
    }
    byte read_byte() {
      if (reading_stack_empty_p()) {
        return fgetc(stdin);
      }
      else {
        return fgetc(reading_stack_tos());
      }
    }
    byte_unread(byte c) {
      if (reading_stack_empty_p()) {
        ungetc(c, stdin);
      }
      else {
        ungetc(c, reading_stack_tos());
      }
    }
    p_read_byte() {
      // -> byte
      data_stack_push(read_byte());
    }
    p_byte_unread() {
      // byte -> {reading_stack}
      byte_unread(data_stack_pop());
    }
    p_byte_print() {
      // byte ->
      printf("%c", data_stack_pop());
    }
    p_ignore_until_double_quote() {
      while (true) {
        jo jo = read_raw_jo();
        if (jo == DOUBLE_QUOTE) {
          return;
        }
        else {
          // loop
        }
      }
    }
    k_one_byte() {
      byte byte = read_byte();
      p_ignore_until_double_quote();
      here(JO_INS_BYTE);
      here(byte);
    }
    k_byte() {
      // (byte ...)
      while (true) {
        jo jo = read_raw_jo();
        if (jo == ROUND_KET) {
          return;
        }
        else if (jo == DOUBLE_QUOTE) {
          k_one_byte();
          // loop
        }
        else {
          // loop
        }
      }
    }
    expose_byte() {
      define_prim("has-byte?", p_has_byte_p);
      define_prim("read/byte", p_read_byte);
      define_prim("byte/unread", p_byte_unread);
      define_prim("byte/print", p_byte_print);
      define_prim("ignore-until-double-quote", p_ignore_until_double_quote);

      define_prim("ins/byte", i_int);
      define_primkey("byte", k_byte);
    }
    k_one_string() {
      // "..."
      char buffer[1024 * 1024];
      cell cursor = 0;
      while (true) {
        char c = read_byte();
        if (c == '"') {
          buffer[cursor] = 0;
          cursor++;
          break;
        }
        else {
          buffer[cursor] = c;
          cursor++;
        }
      }
      char* str = malloc(cursor);
      strcpy(str, buffer);
      here(JO_INS_STRING);
      here(str);
    }
    k_string() {
      // (string "...")
      while (true) {
        jo s = read_raw_jo();
        if (s == ROUND_KET) {
          return;
        }
        else if (s == DOUBLE_QUOTE) {
          k_one_string();
        }
        else {
          // do nothing
        }
      }
    }
    p_string_length() {
      // string -> length
      data_stack_push(strlen(data_stack_pop()));
    }
    p_string_print() {
      // string -> {terminal-output}
      printf("%s", data_stack_pop());
    }
    p_string_dot() {
      // string -> {terminal-output}
      printf("\"%s \"", data_stack_pop());
    }
    p_string_append_to_buffer() {
      // buffer string -> buffer
      char* str = data_stack_pop();
      char* buffer = data_stack_tos();
      strcat(buffer, str);
    }
    p_string_first_byte() {
      char* s = data_stack_pop();
      data_stack_push(s[0]);
    }
    p_string_last_byte() {
      char* s = data_stack_pop();
      cell i = 0;
      while (s[i+1] != 0) {
        i++;
      }
      data_stack_push(s[i]);
    }
    p_string_member_p() {
      // non-zero-byte string -> true or false
      char* s = data_stack_pop();
      byte b = data_stack_pop();
      cell i = 0;
      while (s[i] != 0) {
        if (s[i] == b) {
          data_stack_push(true);
          return;
        }
        else {
          i++;
        }
      }
      data_stack_push(false);
    }
    p_string_find_byte() {
      // byte string -> [index true] or [false]
      char* s = data_stack_pop();
      byte b = data_stack_pop();
      cell i = 0;
      while (s[i] != 0) {
        if (s[i] == b) {
          data_stack_push(i);
          data_stack_push(true);
          return;
        }
        else {
          i++;
        }
      }
      data_stack_push(false);
    }
    p_string_equal_p() {
      data_stack_push(string_equal(data_stack_pop(), data_stack_pop()));
    }
    expose_string() {
      define_prim("ins/string", i_int);
      define_primkey("string", k_string);
      define_primkey("one-string", k_one_string);
      define_prim("string/print", p_string_print);
      define_prim("string/dot", p_string_dot);
      define_prim("string/length", p_string_length);
      define_prim("string/append-to-buffer", p_string_append_to_buffer);
      define_prim("string/first-byte", p_string_first_byte);
      define_prim("string/last-byte", p_string_last_byte);
      define_prim("string/member?", p_string_member_p);
      define_prim("string/find-byte", p_string_find_byte);
      define_prim("string/equal?", p_string_equal_p);
    }
    p_alias_push() {
      jo name = data_stack_pop();
      jo nick = data_stack_pop();
      alias a = {.nick = nick, .name = name};
      alias_stack_push(a);
    }
    p_alias_filter() {
      jo nick = data_stack_pop();
      cell base = keyword_stack_tos();
      cell i = alias_stack_pointer;
      while (i >= base) {
        if (alias_stack[i].nick == nick) {
          data_stack_push(alias_stack[i].name);
          return;
        }
        else {
          i--;
        }
      }
      data_stack_push(nick);
    }
    bool has_jo_p() {
      byte c;
      while (true) {

        if (!has_byte_p()) {
          return false;
        }

        c = read_byte();

        if (isspace(c)) {
          // loop
        }
        else {
          byte_unread(c);
          return true;
        }
      }
    }
    p_has_jo_p() {
      data_stack_push(has_jo_p());
    }
    p_read_raw_jo() {
      // {reading_stack} -> jo
      byte buf[1024];
      cell cur = 0;
      cell collecting = false;
      byte c;
      byte go = true;

      while (go) {

        if (!has_byte_p()) {
          if (!collecting) {
            printf("- p_read_raw_jo meet end-of-file\n");
            return;
          }
          else {
            break;
          }
        }

        c = read_byte();

        if (!collecting) {
          if (isspace(c)) {
            // loop
          }
          else {
            collecting = true;
            buf[cur] = c;
            cur++;
            if (isbarcket(c)) {
              go = false;
            }
          }
        }

        else {
          if (isbarcket(c) ||
              isspace(c)) {
            byte_unread(c);
            go = false;
          }
          else {
            buf[cur] = c;
            cur++;
          }
        }
      }

      buf[cur] = 0;
      data_stack_push(str2jo(buf));
    }
      typedef jo reading_filter;

      typedef reading_filter jo_filter_stack_t[128];
      jo_filter_stack_t jo_filter_stack;

      cell jo_filter_stack_base = 0;
      cell jo_filter_stack_pointer = 0;

      jo_filter_stack_push(reading_filter value) {
        jo_filter_stack[jo_filter_stack_pointer] = value;
        jo_filter_stack_pointer++;
      }

      reading_filter jo_filter_stack_pop() {
        jo_filter_stack_pointer--;
        return jo_filter_stack[jo_filter_stack_pointer];
      }

      reading_filter jo_filter_stack_tos() {
        return jo_filter_stack[jo_filter_stack_pointer - 1];
      }

      bool jo_filter_stack_empty_p() {
        return jo_filter_stack_pointer == jo_filter_stack_base;
      }
      p_jo_filter_stack_push() {
        jo_filter_stack_push(data_stack_pop());
      }
      p_jo_filter_stack_pop() {
        data_stack_push(jo_filter_stack_pop());
      }
      run_jo_filter() {
        cell i = jo_filter_stack_pointer;
        while (i > jo_filter_stack_base) {
          jo_apply_now(jo_filter_stack[i-1]);
          i--;
        }
      }
      init_jo_filter_stack() {
        jo_filter_stack_push(str2jo("alias-filter"));
      }
    p_read_jo() {
      p_read_raw_jo();
      run_jo_filter();
    }
    jo read_jo() {
      p_read_jo();
      return data_stack_pop();
    }
    jo read_raw_jo() {
      p_read_raw_jo();
      return data_stack_pop();
    }
    jo cat_2_jo(jo x, jo y) {
      char str[2 * 1024];
      str[0] = 0;
      strcat(str, jo2str(x));
      strcat(str, jo2str(y));
      return str2jo(str);
    }
    jo cat_3_jo(jo x, jo y, jo z) {
      char str[3 * 1024];
      str[0] = 0;
      strcat(str, jo2str(x));
      strcat(str, jo2str(y));
      strcat(str, jo2str(z));
      return str2jo(str);
    }
    p_jo_append() {
      jo jo2 = data_stack_pop();
      jo jo1 = data_stack_pop();
      data_stack_push(cat_2_jo(jo1, jo2));
    }
    p_empty_jo() {
      data_stack_push(EMPTY_JO);
    }
    p_jo_used_p() {
      // jo -> bool
      jo jo = data_stack_pop();
      data_stack_push(used_jo_p(jo));
    }
    p_jo_to_string() {
      // jo -> string
      jo jo = data_stack_pop();
      data_stack_push(jo2str(jo));
    }
    p_string_length_to_jo() {
      // string length -> jo
      cell len = data_stack_pop();
      cell str = data_stack_pop();
      char buffer[2 * 1024];
      strncpy(buffer, str, len);
      buffer[len] = 0;
      data_stack_push(str2jo(buffer));
    }
    p_string_to_jo() {
      // string -> jo
      char* str = data_stack_pop();
      data_stack_push(str2jo(str));
    }
    p_null() {
      data_stack_push(JO_NULL);
    }
    k_raw_jo() {
      // (raw-jo ...)
      while (true) {
        jo s = read_raw_jo();
        if (s == ROUND_BAR) {
          jo_apply(read_jo());
        }
        else if (s == ROUND_KET) {
          break;
        }
        else {
          here(JO_INS_JO);
          here(s);
        }
      }
    }
    k_jo() {
      // (jo ...)
      while (true) {
        jo s = read_jo();
        if (s == ROUND_BAR) {
          jo_apply(read_jo());
        }
        else if (s == ROUND_KET) {
          break;
        }
        else {
          here(JO_INS_JO);
          here(s);
        }
      }
    }
    p_jo_print() {
      // jo -> {terminal-output}
      printf("%s", jo2str(data_stack_pop()));
    }
    p_jo_dot() {
      // jo -> {terminal-output}
      printf("%s ", jo2str(data_stack_pop()));
    }
    cell p_generate_jo_counter = 0;
    p_generate_jo() {
      char* s = data_stack_pop();
      char buffer [1024];
      sprintf(buffer, "%s:generated-jo#%ld", jo2str(s), p_generate_jo_counter);
      p_generate_jo_counter++;
      data_stack_push(str2jo(buffer));
    }
    p_jo_find_byte() {
      // byte jo -> [index true] or [false]
      p_jo_to_string();
      p_string_find_byte();
    }
    p_jo_right_part() {
      // index jo -> jo
      jo jo = data_stack_pop();
      cell index = data_stack_pop();
      char* s = jo2str(jo);
      data_stack_push(str2jo(s + index));
    }
    p_jo_left_part() {
      // index jo -> jo
      char target[1024];
      jo jo = data_stack_pop();
      cell index = data_stack_pop();
      char* source = jo2str(jo);
      cell i = 0;
      while (i < index) {
        target[i] = source[i];
        i++;
      }
      target[index] = 0;
      data_stack_push(str2jo(target));
    }
    p_jo_part() {
      // index-begin index-end jo -> jo
      char target[1024];
      jo jo = data_stack_pop();
      cell index_end = data_stack_pop();
      cell index_begin = data_stack_pop();
      char* source = jo2str(jo);
      cell i = index_begin;
      while (i < index_end) {
        target[i] = source[i];
        i++;
      }
      target[index_end] = 0;
      data_stack_push(str2jo(target + index_begin));
    }
    expose_jo() {
      define_prim("null", p_null);

      define_prim("jo-filter-stack-push", p_jo_filter_stack_push);
      define_prim("jo-filter-stack-pop", p_jo_filter_stack_pop);

      define_prim("alias-push", p_alias_push);
      define_prim("alias-filter", p_alias_filter);

      define_prim("has-jo?", p_has_jo_p);

      define_prim("read/raw-jo", p_read_raw_jo);
      define_prim("read/jo", p_read_jo);

      define_prim("ins/jo", i_int);
      define_primkey("jo", k_jo);
      define_primkey("raw-jo", k_raw_jo);

      define_prim("jo/used?", p_jo_used_p);
      define_prim("jo/append", p_jo_append);
      define_prim("empty-jo", p_empty_jo);
      define_prim("jo->string", p_jo_to_string);
      define_prim("string->jo", p_string_to_jo);
      define_prim("string/length->jo", p_string_length_to_jo);
      define_prim("jo/print", p_jo_print);
      define_prim("jo/dot", p_jo_dot);
      define_prim("generate-jo", p_generate_jo);

      define_prim("jo/find-byte", p_jo_find_byte);
      define_prim("jo/left-part", p_jo_left_part);
      define_prim("jo/right-part", p_jo_right_part);
      define_prim("jo/part", p_jo_part);
    }
    p_error_number_print() {
      // errno -> {terminal-output}
      int no = data_stack_pop();
      printf("%s", strerror(no));
    }
    p_path_open_read() {
      // [path] -> [file true] or [errno false]
      char* path = data_stack_pop();

      FILE* file = fopen(path, "r");
      if (file == NULL) {
        data_stack_push(errno);
        data_stack_push(false);
      }
      else {
        data_stack_push(file);
        data_stack_push(true);
      }
    }
    p_path_open_write() {
      // [path] -> [file true] or [errno false]
      char* path = data_stack_pop();

      FILE* file = fopen(path, "wx");
      if (file == NULL) {
        data_stack_push(errno);
        data_stack_push(false);
      }
      else {
        data_stack_push(file);
        data_stack_push(true);
      }
    }
    p_path_open_read_and_write() {
      // [path] -> [file true] or [errno false]
      char* path = data_stack_pop();

      FILE* file = fopen(path, "r+");
      if (file == NULL) {
        data_stack_push(errno);
        data_stack_push(false);
      }
      else {
        data_stack_push(file);
        data_stack_push(true);
      }
    }
    p_path_open_create() {
      // [path] -> [file true] or [errno false]
      char* path = data_stack_pop();

      FILE* file = fopen(path, "w+");
      if (file == NULL) {
        data_stack_push(errno);
        data_stack_push(false);
      }
      else {
        data_stack_push(file);
        data_stack_push(true);
      }
    }
    p_file_close() {
      // [file] -> [true] or [errno false]
      // - error reasons
      // 1. to close an unopened file descriptor
      // 2. close the same file descriptor twice
      // 3. error conditions for specific file system
      //    to diagnose during a close operation
      //    - for example, NFS (Network File System)
      FILE* file = data_stack_pop();

      if (fclose(file) == EOF) {
        data_stack_push(errno);
        data_stack_push(false);
      }
      else {
        data_stack_push(true);
      }
    }
    p_file_end_p() {
      // file -> true or false
      FILE* file = data_stack_pop();

      if (feof(file)) {
        data_stack_push(true);
      }
      else {
        data_stack_push(false);
      }
    }
    p_file_read() {
      // [file buffer requested-bytes] ->
      // [real-bytes true] or [errno false]
      // - partial read reasons
      //   1. [regular-file] end-of-file is reached
      //   2. [terminal] meets '\n'
      size_t want_bytes = data_stack_pop();
      void* buffer = data_stack_pop();
      FILE* file = data_stack_pop();

      size_t real_bytes = fread(buffer, 1, file, want_bytes);
      if (real_bytes != want_bytes) {
        if (ferror(file)) {
          data_stack_push(errno);
          data_stack_push(false);
        }
        else {
          data_stack_push(real_bytes);
          data_stack_push(true);
        }
      }
      else {
        data_stack_push(real_bytes);
        data_stack_push(true);
      }
    }
    p_file_write() {
      // [file buffer want-bytes] ->
      // [true] or [errno false]
      // - partial write reasons
      //   1. disk was filled
      //   2. the process resource limit on file sizes was reached
      size_t want_bytes = data_stack_pop();
      void* buffer = data_stack_pop();
      FILE* file = data_stack_pop();

      ssize_t real_bytes = fwrite(buffer, 1, want_bytes, file);
      if (real_bytes != want_bytes) {
        data_stack_push(errno);
        data_stack_push(false);
      }
      else {
        data_stack_push(true);
      }
    }
    p_file_size() {
      // file -> int
      FILE* file = data_stack_pop();
      struct stat file_state;
      fstat(fileno(file), &file_state);
      data_stack_push(file_state.st_size);
    }
    p_file_regular_file_p() {
      // file -> true or false
      FILE* file = data_stack_pop();
      struct stat file_state;
      fstat(fileno(file), &file_state);
      if ((file_state.st_mode & S_IFMT) == S_IFREG) {
        data_stack_push(true);
      }
      else {
        data_stack_push(false);
      }
    }
    p_file_directory_p() {
      // file -> true or false
      FILE* file = data_stack_pop();
      struct stat file_state;
      fstat(fileno(file), &file_state);
      if ((file_state.st_mode & S_IFMT) == S_IFDIR) {
        data_stack_push(true);
      }
      else {
        data_stack_push(false);
      }
    }
    p_file_character_device_p() {
      // file -> true or false
      FILE* file = data_stack_pop();
      struct stat file_state;
      fstat(fileno(file), &file_state);
      if ((file_state.st_mode & S_IFMT) == S_IFCHR) {
        data_stack_push(true);
      }
      else {
        data_stack_push(false);
      }
    }
    p_file_block_device_p() {
      // file -> true or false
      FILE* file = data_stack_pop();
      struct stat file_state;
      fstat(fileno(file), &file_state);
      if ((file_state.st_mode & S_IFMT) == S_IFBLK) {
        data_stack_push(true);
      }
      else {
        data_stack_push(false);
      }
    }
    p_file_fifo_p() {
      // file -> true or false
      FILE* file = data_stack_pop();
      struct stat file_state;
      fstat(fileno(file), &file_state);
      if ((file_state.st_mode & S_IFMT) == S_IFIFO) {
        data_stack_push(true);
      }
      else {
        data_stack_push(false);
      }
    }
    p_file_socket_p() {
      // file -> true or false
      FILE* file = data_stack_pop();
      struct stat file_state;
      fstat(fileno(file), &file_state);
      if ((file_state.st_mode & S_IFMT) == S_IFSOCK) {
        data_stack_push(true);
      }
      else {
        data_stack_push(false);
      }
    }
    p_path_exist_p() {
      // path -> true or false
      char* path = data_stack_pop();

      if (access(path, F_OK) == -1) {
        data_stack_push(false);
      }
      else {
        data_stack_push(true);
      }
    }
    p_path_readable_p() {
      // path -> true or false
      char* path = data_stack_pop();

      if (access(path, R_OK) == -1) {
        data_stack_push(false);
      }
      else {
        data_stack_push(true);
      }
    }
    p_path_writable_p() {
      // path -> true or false
      char* path = data_stack_pop();

      if (access(path, W_OK) == -1) {
        data_stack_push(false);
      }
      else {
        data_stack_push(true);
      }
    }
    p_path_executable_p() {
      // path -> true or false
      char* path = data_stack_pop();

      if (access(path, X_OK) == -1) {
        data_stack_push(false);
      }
      else {
        data_stack_push(true);
      }
    }
    p_file_print_path() {
      // file -> path
      FILE* file = data_stack_pop();

      char proc_link_path[PATH_MAX];
      char file_path[PATH_MAX];

      sprintf(proc_link_path, "/proc/self/fd/%d", fileno(file));

      ssize_t real_bytes = readlink(proc_link_path, file_path, PATH_MAX);
      if (real_bytes == -1) {
        printf("- p_file_print_path fail readlink /proc/self/fd/%d\n", fileno(file));
        perror("\n");
      }
      else {
        file_path[real_bytes] = '\0';
        printf("%s", file_path);
      }
    }
    p_repl();

    p_path_load() {
      // path -> {reading_stack}
      char* path = data_stack_pop();
      FILE* file = fopen(path, "r");
      if(file == NULL) {
        printf("- p_path_load fail : %s\n", path);
        perror("file open failed");
        return;
      }
      reading_stack_push(file);
      p_repl();
      reading_stack_pop();
      fclose(file);
    }
    k_one_include() {
      // "..."
      char* path = malloc(PATH_MAX);
      cell cursor = 0;
      while (true) {
        char c = read_byte();
        if (c == '"') {
          path[cursor] = 0;
          cursor++;
          break;
        }
        else {
          path[cursor] = c;
          cursor++;
        }
      }
      char* real_read_path = get_real_reading_path(path);
      free(path);
      data_stack_push(real_read_path);
      p_path_load();
      free(real_read_path);
    }
    k_include() {
      // (include "..." ...)
      while (true) {
        jo s = read_raw_jo();
        if (s == ROUND_KET) {
          return;
        }
        else if (s == ROUND_BAR) {
          jo_apply(read_jo());
        }
        else if (s == DOUBLE_QUOTE) {
          k_one_include();
        }
        else {
          // do nothing
        }
      }
    }
    expose_file() {
      define_prim("error-number/print", p_error_number_print);

      define_prim("path/open/read", p_path_open_read);
      define_prim("path/open/write", p_path_open_write);
      define_prim("path/open/create", p_path_open_create);
      define_prim("path/open/read-and-write", p_path_open_read_and_write);

      define_prim("file/close", p_file_close);

      define_prim("file/read", p_file_read);
      define_prim("file/write", p_file_write);

      define_prim("file/size", p_file_size);

      define_prim("file/regular-file?", p_file_regular_file_p);
      define_prim("file/directory?", p_file_directory_p);
      define_prim("file/character-device?", p_file_character_device_p);
      define_prim("file/block-device?", p_file_block_device_p);
      define_prim("file/fifo?", p_file_fifo_p);
      define_prim("file/socket?", p_file_socket_p);

      define_prim("path/exist?", p_path_exist_p);
      define_prim("path/readable?", p_path_readable_p);
      define_prim("path/writable?", p_path_writable_p);
      define_prim("path/executable?", p_path_executable_p);

      define_prim("file/print-path", p_file_print_path);

      define_prim("path/load", p_path_load);
      define_primkey("include", k_include);
    }
    p_command_run() {
      // string -> {*}
      system(data_stack_pop());
    }
    p_n_command_run() {
      // ... string n -> *
      cell n = data_stack_pop();
      cell i = 0;
      char* str = malloc(4 * 1024);
      str[0] = 0;
      while (i < n) {
        strcat(str, data_stack[data_stack_pointer - n + i]);
        i++;
      }
      data_stack_pointer = data_stack_pointer - n;
      system(str);
      free(str);
    }
    cell cmd_number;

    p_cmd_number() {
      // -> cmd_number
      data_stack_push(cmd_number);
    }
    char** cmd_string_array;

    p_index_to_cmd_string() {
      // index -> string
      cell index = data_stack_pop();
      char* cmd_string = cmd_string_array[index];
      data_stack_push(cmd_string);
    }
    p_find_env_string() {
      // string -> [env-string true] or [false]
      char* var_string = data_stack_pop();
      char* env_string = getenv(var_string);
      if (env_string == NULL) {
        data_stack_push(false);
      }
      else {
        data_stack_push(env_string);
        data_stack_push(true);
      }
    }
    expose_system() {
      define_prim("command/run", p_command_run);
      define_prim("n-command/run", p_n_command_run);
      define_prim("cmd-number", p_cmd_number);
      define_prim("index->cmd-string", p_index_to_cmd_string);
      define_prim("find-env-string", p_find_env_string);
    }
    ccall (char* function_name, void* lib) {
      primitive fun = dlsym(lib, function_name);
      if (fun == NULL) {
        printf("- ccall fail\n");
        printf("  function_name : %s\n", function_name);
        printf("  dynamic link error : %s\n", dlerror());
      };
      fun();
    }
    k_clib_one() {
      // "..."
      char* path = malloc(PATH_MAX);
      cell cursor = 0;
      while (true) {
        char c = read_byte();
        if (c == '"') {
          path[cursor] = 0;
          cursor++;
          break;
        }
        else {
          path[cursor] = c;
          cursor++;
        }
      }
      char* real_read_path = get_real_reading_path(path);
      free(path);
      void* lib = dlopen(real_read_path, RTLD_LAZY);
      if (lib == NULL) {
        printf("- k_clib_one fail to open library\n");
        printf("  real_read_path : %s\n", real_read_path);
        printf("  dynamic link error : %s\n", dlerror());
        p_debug();
        return;
      };
      free(real_read_path);
      ccall("expose", lib);
    }
    k_clib() {
      // (clib "..." ...)
      while (true) {
        jo s = read_raw_jo();
        if (s == ROUND_KET) {
          return;
        }
        else if (s == DOUBLE_QUOTE) {
          k_clib_one();
        }
        else {
          // do nothing
        }
      }
    }
    expose_cffi() {
      define_prim("clib", k_clib);
    }
    k_run();

    k_define() {
      jo name = read_jo();
      k_run();
      data_stack_push(name);
      p_bind_name();
    }
      k_declare_one() {
        jo jo = read_jo();
        jo->tag = JO_DECLARED;
        k_ignore();
      }
      k_declare() {
        while (true) {
          jo s = read_jo();
          if (s == ROUND_KET) {
            return;
          }
          else if (s == ROUND_BAR) {
            k_declare_one();
          }
          else {
            // do nothing
          }
        }
      }
    p_compile_jojo();

    k_run() {
      // (run ...)
      jo* jojo = compiling_stack_tos();
      p_compile_jojo();
      return_stack_new_point(jojo);
      eval();
    }
    bool test_flag = false;
    p_test_flag() { data_stack_push(test_flag); }
    p_test_flag_on() { test_flag = true; }
    p_test_flag_off() { test_flag = false; }

    k_test() {
      if (test_flag) {
        k_run();
      }
      else {
        k_ignore();
      }
    }
    bool repl_flag = false;
    p_repl_flag() { data_stack_push(repl_flag); }
    p_repl_flag_on() { repl_flag = true; }
    p_repl_flag_off() { repl_flag = false; }

    p_repl() {
      while (true) {
        if (!has_jo_p()) {
          return;
        }
        jo s = read_jo();
        if (s == ROUND_BAR) {
          jo_apply(read_jo());
          if (repl_flag) {
            p_print_data_stack();
          }
        }
        else {
          // loop
        }
      }
    }
    p_bare_jojo_print() {
      // jojo -> {terminal-output}
      jo* jojo = data_stack_pop();
      printf("[ ");
      while (true) {
        if (jojo[0] == 0 && jojo[1] == 0) {
          break;
        }
        else if (jojo[0] == JO_INS_INT) {
          printf("(int %ld) ", jojo[1]);
          jojo++;
          jojo++;
        }
        else if (jojo[0] == JO_INS_JO) {
          printf("(jo %s) ", jo2str(jojo[1]));
          jojo++;
          jojo++;
        }
        else if (jojo[0] == JO_INS_STRING) {
          printf("(string \"%s\") ", (char*)jojo[1]);
          jojo++;
          jojo++;
        }
        else if (jojo[0] == JO_INS_BYTE) {
          printf("(btye \"%c\") ", (char)jojo[1]);
          jojo++;
          jojo++;
        }
        else if (jojo[0] == JO_INS_BARE_JOJO) {
          printf("(bare-jojo ");
          data_stack_push(jojo + 2);
          p_bare_jojo_print();
          printf(") ");
          jojo = jojo + (cell)jojo[1];
          jojo++;
        }
        else if (jojo[0] == JO_INS_ADDRESS) {
          printf("(address %ld) ", jojo[1]);
          jojo++;
          jojo++;
        }
        else if (jojo[0] == JO_INS_JUMP_IF_FALSE) {
          printf("(jump-if-false %ld) ", jojo[1]);
          jojo++;
          jojo++;
        }
        else if (jojo[0] == JO_INS_JUMP) {
          printf("(jump %ld) ", jojo[1]);
          jojo++;
          jojo++;
        }
        else if (jojo[0] == JO_INS_LOOP) {
          printf("(loop) ");
          jojo++;
          jojo++;
        }
        else if (jojo[0] == JO_INS_RECUR) {
          printf("(recur) ");
          jojo++;
          jojo++;
        }
        else if (jojo[0] == JO_INS_TAIL_CALL) {
          printf("(tail-call %s) ", jo2str(jojo[1]));
          jojo++;
          jojo++;
        }
        else {
          printf("%s ", jo2str(jojo[0]));
          jojo++;
        }
      }
      printf("] ");
    }
    point_return_point(cell i) {
      printf("    - ");
      if (i != return_stack_pointer -1) {
        printf("{ %s } ", jo2str(*(return_stack[i].jojo - 1)));
      }
      data_stack_push(return_stack[i].jojo);
      p_bare_jojo_print();
      printf("\n");

      cell cursor = return_stack[i].local_pointer;
      cell end = return_stack[i+1].local_pointer;
      if (i = return_stack_pointer -1) {
        end = current_local_pointer;
      }

      while (end > cursor) {
        printf("      %s = %ld %s\n"
               , jo2str(local_area[cursor].name)
               , local_area[cursor].local_data
               , jo2str(local_area[cursor].local_tag));
        cursor++;
      }
    }
    p_print_return_stack() {
      cell i = return_stack_base;
      printf("  - return-stack :\n");
      while (i < return_stack_pointer) {
        point_return_point(i);
        i++;
      }
    }
    cell debug_repl_level = 0;

    p_debug_repl() {
      while (true) {
        if (!has_jo_p()) {
          return;
        }
        jo jo = read_raw_jo();
        if (jo == str2jo("help")) {
          printf("- debug-repl usage :\n");
          printf("  - available commands :\n");
          printf("    help exit bye\n");
        }
        else if (jo == str2jo("exit")) {
          return;
        }
        else if (jo == str2jo("bye")) {
          p_bye();
          return;
        }
        else if (jo == ROUND_BAR) {
          jo_apply(read_jo());
          p_print_data_stack();
          printf("debug[%ld]> ", debug_repl_level);
        }
        else {
          // loop
        }
      }
    }
    p_debug() {
      reading_stack_push(stdin);

      printf("- in debug-repl [level %ld] >_<!\n", debug_repl_level);
      p_print_return_stack();
      p_print_data_stack();
      printf("debug[%ld]> ", debug_repl_level);
      debug_repl_level++;
      p_debug_repl();
      debug_repl_level--;
      printf("- exit debug-repl [level %ld]\n", debug_repl_level);

      reading_stack_pop();
    }
    cell stepper_counter = 0;
    cell pending_steps = 0;

    exit_stepper() {
      step_flag = false;
      stepper_counter = 0;
      pending_steps = 0;
      printf("- exit stepper\n");
    }

    stepper() {
      printf("stepper> ");
      while (true) {

        if (return_stack_empty_p()) {
          printf("\n");
          printf("- the return-stack is empty\n");
          exit_stepper();
          return;
        }

        if (pending_steps > 0) {
          p_print_return_stack();
          p_print_data_stack();
          stepper_counter++;
          printf("- stepper counting : %ld\n", stepper_counter);
          pending_steps--;
          return;
        }

        jo jo = read_raw_jo();
        if (jo == str2jo("help")) {
          printf("- stepper usage :\n");
          printf("  type '.' to execute one step\n");
          printf("  type a numebr to execute the number of steps\n");
          printf("  - available commands :\n");
          printf("    help exit bye\n");
        }
        else if (jo == str2jo(".")) {
          p_print_return_stack();
          p_print_data_stack();
          stepper_counter++;
          printf("- stepper counting : %ld\n", stepper_counter);
          return;
        }
        else if (nat_string_p(jo2str(jo))) {
          p_print_return_stack();
          p_print_data_stack();
          stepper_counter++;
          printf("- stepper counting : %ld\n", stepper_counter);
          pending_steps = string_to_int(jo2str(jo)) - 1;
          return;
        }
        else if (jo == str2jo("exit")) {
          exit_stepper();
          return;
        }
        else if (jo == str2jo("bye")) {
          p_bye();
          return;
        }
        else {
          // loop
        }
      }
    }
    p_step() {
      step_flag = true;
    }
      kernel_signal_handler(int sig, siginfo_t *siginfo, void *ucontext) {
        fflush(stdin);
        fflush(stdout);
        fflush(stderr);

        printf("- kernel_signal_handler\n");
        psiginfo(siginfo, "  signal ");

        int errno_backup;
        errno_backup = errno;

        p_debug();

        errno = errno_backup;
      }
      init_kernel_signal_handler() {
        struct sigaction kernel_signal_action;

        sigemptyset(&kernel_signal_action.sa_mask);

        kernel_signal_action.sa_flags = SA_SIGINFO | SA_NODEFER | SA_RESTART;
        kernel_signal_action.sa_sigaction = kernel_signal_handler;

        int sig_array[] = { SIGSEGV, SIGBUS, SIGFPE, SIGILL,
                            SIGPIPE, SIGSYS, SIGXCPU, SIGXFSZ};
        int sig_array_length = sizeof(sig_array)/sizeof(sig_array[0]);
        cell i = 0;
        while (i < sig_array_length) {
          if (sigaction(sig_array[i], &kernel_signal_action, NULL) == -1) {
            perror("- init_kernel_signal_handler fail");
          }
          i++;
        }
      }
    expose_top_level() {
      define_primkey("define", k_define);
      define_primkey("bind-name", p_bind_name);
      define_primkey("declare", k_declare);

      define_primkey("run", k_run);

      define_primkey("test", k_test);
      define_prim("test-flag", p_test_flag);
      define_prim("test-flag/on", p_test_flag_on);
      define_prim("test-flag/off", p_test_flag_off);

      define_prim("repl", p_repl);
      define_prim("repl-flag", p_repl_flag);
      define_prim("repl-flag/on", p_repl_flag_on);
      define_prim("repl-flag/off", p_repl_flag_off);

      define_prim("bare-jojo/print", p_bare_jojo_print);
      define_prim("print-return-stack", p_print_return_stack);
      define_prim("debug", p_debug);

      define_prim("step", p_step);
    }
    k_ignore() {
      while (true) {
        jo s = read_raw_jo();
        if (s == ROUND_BAR) {
          k_ignore();
        }
        if (s == ROUND_KET) {
          break;
        }
      }
    }
    compile_until_meet_jo(jo ending_jo) {
      while (true) {
        jo jo = read_jo();
        if (jo == ROUND_BAR) {
          jo_apply(read_jo());
        }
        else if (jo == ending_jo) {
          break;
        }
        else if (used_jo_p(jo)) {
          here(jo);
        }
        else {
          // no compile before define
          printf("- compile_until_meet_jo undefined : %s\n", jo2str(jo));
          k_ignore();
          p_debug();
          return;
        }
      }
    }
    p_compile_until_meet_jo() {
      compile_until_meet_jo(data_stack_pop());
    }
    jo compile_until_meet_jo_or_jo(jo ending_jo1, jo ending_jo2) {
      while (true) {
        jo jo = read_jo();
        if (jo == ROUND_BAR) {
          jo_apply(read_jo());
        }
        else if (jo == ending_jo1 || jo == ending_jo2) {
          return jo;
        }
        else if (used_jo_p(jo)) {
          here(jo);
        }
        else {
          // no compile before define
          printf("- compile_until_meet_jo_or_jo undefined : %s\n", jo2str(jo));
          printf("- ending_jo1 : %s\n", jo2str(ending_jo1));
          printf("- ending_jo2 : %s\n", jo2str(ending_jo2));
          k_ignore();
          p_debug();
          return JO_NULL; // to fool the compiler
        }
      }
    }
    p_compile_until_round_ket() {
      compile_until_meet_jo(ROUND_KET);
    }
    i_jump() {
      // {return_stack}
      return_point rp = return_stack_tos();
      jo* jojo = rp.jojo;
      cell offset = jojo[0];
      return_point rp1 = return_stack_pop();
      return_stack_make_point(jojo + offset, rp1.local_pointer);
    }
    k_jump() {
      here(JO_INS_JUMP);
      here(string_to_int(jo2str(read_raw_jo())));
      k_ignore();
    }
    i_jump_if_false() {
      // [bool] -> {return_stack}
      return_point rp = return_stack_tos();
      return_stack_inc();
      jo* jojo = rp.jojo;
      cell offset = jojo[0];
      cell b = data_stack_pop();
      if (b == false) {
        return_point rp1 = return_stack_pop();
        return_stack_make_point(jojo + offset, rp1.local_pointer);
      }
    }
    k_jump_if_false() {
      here(JO_INS_JUMP_IF_FALSE);
      here(string_to_int(jo2str(read_raw_jo())));
      k_ignore();
    }
    // - without else
    //   (if a b p? then c d)
    //   ==>
    //     a b p?
    //     jump_if_false[:end-of-then]
    //     c d
    //   :end-of-then

    // - with else
    //   (if a b p? then c d else e f)
    //   ==>
    //     a b p?
    //     jump_if_false[:end-of-then]
    //     c d
    //     jump[:end-of-else]
    //   :end-of-then
    //     e f
    //   :end-of-else

    k_if() {
      compile_until_meet_jo(JO_THEN);
      here(JO_INS_JUMP_IF_FALSE);
      jo* end_of_then = compiling_stack_tos();
      compiling_stack_inc();
      jo ending_jo = compile_until_meet_jo_or_jo(JO_ELSE, ROUND_KET);
      if (ending_jo == ROUND_KET) {
        end_of_then[0] = compiling_stack_tos() - end_of_then;
        return;
      }
      else {
        here(JO_INS_JUMP);
        jo* end_of_else = compiling_stack_tos();
        compiling_stack_inc();
        end_of_then[0] = compiling_stack_tos() - end_of_then;
        p_compile_until_round_ket();
        end_of_else[0] = compiling_stack_tos() - end_of_else;
        return;
      }
    }
    typedef jo current_compiling_jojo_stack_t[1024];

    current_compiling_jojo_stack_t current_compiling_jojo_stack;
    cell current_compiling_jojo_stack_base = 0;
    cell current_compiling_jojo_stack_pointer = 0;

    current_compiling_jojo_stack_push(jo* value) {
      current_compiling_jojo_stack[current_compiling_jojo_stack_pointer] = value;
      current_compiling_jojo_stack_pointer++;
    }

    jo* current_compiling_jojo_stack_pop() {
      current_compiling_jojo_stack_pointer--;
      return current_compiling_jojo_stack[current_compiling_jojo_stack_pointer];
    }

    current_compiling_jojo_stack_inc() {
      current_compiling_jojo_stack[current_compiling_jojo_stack_pointer - 1] =
        current_compiling_jojo_stack[current_compiling_jojo_stack_pointer - 1] + 1;
    }


    jo* current_compiling_jojo_stack_tos() {
      return current_compiling_jojo_stack[current_compiling_jojo_stack_pointer - 1];
    }

    bool current_compiling_jojo_stack_empty_p() {
      return current_compiling_jojo_stack_pointer == current_compiling_jojo_stack_base;
    }
    p_compile_jojo() {
      jo* jojo = compiling_stack_tos();
      current_compiling_jojo_stack_push(jojo);
      compile_until_meet_jo(ROUND_KET);
      here(JO_END);
      here(0);
      here(0);
      current_compiling_jojo_stack_pop();
    }
    i_tail_call() {
      return_point rp = return_stack_pop();
      jo* jojo = rp.jojo;
      jo jo = jojo[0];
      jo_apply_with_local_pointer(jo, rp.local_pointer);
    }
    k_tail_call() {
      // no check for "no compile before define"
      here(JO_INS_TAIL_CALL);
      here(read_jo());
      k_ignore();
    }
    i_loop() {
      return_point rp = return_stack_pop();
      jo* jojo = rp.jojo;
      jo* jojo_self = jojo[0];
      return_stack_make_point(jojo_self, rp.local_pointer);
    }
    k_loop() {
      here(JO_INS_LOOP);
      here(current_compiling_jojo_stack_tos());
      k_ignore();
    }
    i_recur() {
      return_point rp = return_stack_tos();
      return_stack_inc();
      jo* jojo = rp.jojo;
      jo* jojo_self = jojo[0];
      return_stack_new_point(jojo_self);
    }
    k_recur() {
      here(JO_INS_RECUR);
      here(current_compiling_jojo_stack_tos());
      k_ignore();
    }
    p_compiling_stack_tos() {
      data_stack_push(compiling_stack_tos());
    }
    i_bare_jojo() {
      return_point rp = return_stack_pop();
      jo* jojo = rp.jojo;
      cell offset = jojo[0];
      return_point rp1 = {.jojo = jojo + offset,
                          .local_pointer = rp.local_pointer};
      return_stack_push(rp1);
      data_stack_push(jojo + 1);
    }
    k_bare_jojo() {
      // (bare-jojo ...)
      here(JO_INS_BARE_JOJO);
      jo* beginning = compiling_stack_tos();
      compiling_stack_inc();
      p_compile_jojo();
      beginning[0] = (compiling_stack_tos() - beginning);
    }
    k_jojo() {
      // (jojo ...)
      k_bare_jojo();
      here(JO_INS_JO);
      here(TAG_JOJO);
    }
    k_keyword() {
      // (keyword ...)
      k_bare_jojo();
      here(JO_INS_JO);
      here(TAG_KEYWORD);
    }
    k_data() {
      // (data ...)
      p_compile_until_round_ket();
      here(JO_INS_JO);
      here(TAG_DATA);
    }
    cell local_find(jo name) {
      // return index of local_area
      // -1 -- no found
      return_point rp = return_stack_tos();
      cell cursor = current_local_pointer - 1;
      while (cursor >= rp.local_pointer) {
        if (local_area[cursor].name == name) {
          return cursor;
        }
        else {
          cursor--;
        }
      }
      return -1;
    }
    p_local_data_in() {
      cell jo = data_stack_pop();
      cell index = local_find(jo);
      cell data = data_stack_pop();
      if (index != -1) {
        local_area[index].name = jo;
        local_area[index].local_data = data;
      }
      else {
        local_area[current_local_pointer].name = jo;
        local_area[current_local_pointer].local_data = data;
        current_local_pointer = current_local_pointer + 1;
      }
    }
    p_local_data_out() {
      cell jo = data_stack_pop();
      cell index = local_find(jo);
      if (index != -1) {
        local_point lp = local_area[index];
        data_stack_push(lp.local_data);
      }
      else {
        printf("- p_local_data_out fatal error\n");
        printf("  name is not bound\n");
        printf("  name : %s\n", jo2str(jo));
        p_debug();
      }
    }
    p_local_tag_in() {
      cell jo = data_stack_pop();
      cell index = local_find(jo);
      cell tag = data_stack_pop();
      if (index != -1) {
        local_area[index].name = jo;
        local_area[index].local_tag = tag;
      }
      else {
        local_area[current_local_pointer].name = jo;
        local_area[current_local_pointer].local_tag = tag;
        current_local_pointer = current_local_pointer + 1;
      }
    }
    p_local_tag_out() {
      cell jo = data_stack_pop();
      cell index = local_find(jo);
      if (index != -1) {
        local_point lp = local_area[index];
        data_stack_push(lp.local_tag);
      }
      else {
        printf("- p_local_tag_out fatal error\n");
        printf("  name is not bound\n");
        printf("  name : %s\n", jo2str(jo));
        p_debug();
      }
    }
    p_local_in() {
      cell jo = data_stack_pop();
      cell index = local_find(jo);
      cell tag = data_stack_pop();
      cell data = data_stack_pop();
      if (index != -1) {
        local_area[index].name = jo;
        local_area[index].local_tag = tag;
        local_area[index].local_data = data;
      }
      else {
        local_area[current_local_pointer].name = jo;
        local_area[current_local_pointer].local_tag = tag;
        local_area[current_local_pointer].local_data = data;
        current_local_pointer = current_local_pointer + 1;
      }
    }
    p_local_out() {
      cell jo = data_stack_pop();
      cell index = local_find(jo);
      if (index != -1) {
        local_point lp = local_area[index];
        data_stack_push(lp.local_data);
        data_stack_push(lp.local_tag);
      }
      else {
        printf("- p_local_out fatal error\n");
        printf("  name is not bound\n");
        printf("  name : %s\n", jo2str(jo));
        p_debug();
      }
    }
    k_local_data_in() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        k_local_data_in();

        here(JO_INS_JO);
        here(s);
        here(JO_LOCAL_DATA_IN);
      }
    }
    k_local_data_out() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        here(JO_INS_JO);
        here(s);
        here(JO_LOCAL_DATA_OUT);

        k_local_data_out();
      }
    }
    k_local_tag_in() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        k_local_data_in();

        here(JO_INS_JO);
        here(s);
        here(JO_LOCAL_TAG_IN);
      }
    }
    k_local_tag_out() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        here(JO_INS_JO);
        here(s);
        here(JO_LOCAL_TAG_OUT);

        k_local_data_out();
      }
    }
    k_local_in() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        k_local_data_in();

        here(JO_INS_JO);
        here(s);
        here(JO_LOCAL_IN);
      }
    }
    k_local_out() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        here(JO_INS_JO);
        here(s);
        here(JO_LOCAL_OUT);

        k_local_data_out();
      }
    }
    p_current_local_pointer() {
      data_stack_push(current_local_pointer);
    }
    p_last_local_pointer() {
      return_point rp = return_stack_tos();
      data_stack_push(rp.local_pointer);
    }
    expose_keyword() {
      define_primkey("ignore", k_ignore);
      define_primkey("note", k_ignore);

      define_prim("compiling-stack/tos", p_compiling_stack_tos);
      define_prim("compiling-stack/inc", compiling_stack_inc);

      define_prim("compile-until-meet-jo", p_compile_until_meet_jo);
      define_prim("compile-until-round-ket", p_compile_until_round_ket);


      define_prim("ins/jump", i_jump);
      define_primkey("jump", k_jump);

      define_prim("ins/jump-if-false", i_jump_if_false);
      define_primkey("jump-if-false", k_jump_if_false);

      define_primkey("if", k_if);
      define_primkey("else", p_compile_until_round_ket);
      define_primkey("el", p_compile_until_round_ket);

      define_prim("compile-jojo", p_compile_jojo);

      define_prim("ins/tail-call", i_tail_call);
      define_primkey("tail-call", k_tail_call);
      define_prim("ins/loop", i_loop);
      define_primkey("loop", k_loop);
      define_prim("ins/recur", i_recur);
      define_primkey("recur", k_recur);

      define_primkey("data", k_data);
      define_primkey("jojo", k_jojo);
      define_primkey("keyword", k_keyword);

      define_prim("ins/bare-jojo", i_bare_jojo);
      define_primkey("bare-jojo", k_bare_jojo);

      define_prim("local-data-in", p_local_data_in);
      define_prim("local-data-out", p_local_data_out);
      define_primkey(">", k_local_data_in);
      define_primkey("<", k_local_data_out);

      define_prim("local-tag-in", p_local_tag_in);
      define_prim("local-tag-out", p_local_tag_out);
      define_primkey("%>", k_local_tag_in);
      define_primkey("<%", k_local_tag_out);

      define_prim("local-in", p_local_in);
      define_prim("local-out", p_local_out);
      define_primkey(">>", k_local_in);
      define_primkey("<<", k_local_out);

      define_prim("current-local-pointer", p_current_local_pointer);
      define_prim("last-local-pointer", p_last_local_pointer);
    }
    p_here() {
      here(data_stack_pop());
    }
    p_address_of_here() {
     data_stack_push(compiling_stack_tos());
    }
    p_round_bar()    { data_stack_push(ROUND_BAR); }
    p_round_ket()    { data_stack_push(ROUND_KET); }
    p_square_bar()   { data_stack_push(SQUARE_BAR); }
    p_square_ket()   { data_stack_push(SQUARE_KET); }
    p_flower_bar()   { data_stack_push(FLOWER_BAR); }
    p_flower_ket()   { data_stack_push(FLOWER_KET); }
    p_double_quote() { data_stack_push(DOUBLE_QUOTE); }
    p_cell_size() {
      data_stack_push(CELL_SIZE);
    }
    p_newline() {
      printf("\n");
    }
    expose_mise() {
      define_prim("here", p_here);
      define_prim("address-of-here", p_address_of_here);

      define_prim("round-bar", p_round_bar);
      define_prim("round-ket", p_round_ket);
      define_prim("square-bar", p_square_bar);
      define_prim("square-ket", p_square_ket);
      define_prim("flower-bar", p_flower_bar);
      define_prim("flower-ket", p_flower_ket);
      define_prim("double-quote", p_double_quote);

      define_prim("cell-size", p_cell_size);

      define_prim("newline", p_newline);
    }
    expose_play() {
    }
    init_jotable() {
      bzero(jotable, jotable_size * sizeof(jotable_entry));
    }
    init_literal_jo() {
      EMPTY_JO = str2jo("");

      TAG_PRIM         = str2jo("<prim>");
      TAG_JOJO         = str2jo("<jojo>");
      TAG_PRIM_KEYWORD = str2jo("<prim-keyword>");
      TAG_KEYWORD      = str2jo("<keyword>");
      TAG_DATA         = str2jo("<data>");

      JO_DECLARED = str2jo("declared");

      ROUND_BAR    =   str2jo("(");
      ROUND_KET    =   str2jo(")");
      SQUARE_BAR   =   str2jo("[");
      SQUARE_KET   =   str2jo("]");
      FLOWER_BAR   =   str2jo("{");
      FLOWER_KET   =   str2jo("}");
      DOUBLE_QUOTE =   str2jo("\"");

      JO_INS_INT  = str2jo("ins/int");
      JO_INS_JO   = str2jo("ins/jo");
      JO_INS_STRING = str2jo("ins/string");
      JO_INS_BYTE = str2jo("ins/byte");
      JO_INS_BARE_JOJO = str2jo("ins/bare-jojo");
      JO_INS_ADDRESS = str2jo("ins/address");

      JO_INS_JUMP = str2jo("ins/jump");
      JO_INS_JUMP_IF_FALSE = str2jo("ins/jump-if-false");

      JO_INS_TAIL_CALL = str2jo("ins/tail-call");
      JO_INS_LOOP = str2jo("ins/loop");
      JO_INS_RECUR = str2jo("ins/recur");

      JO_NULL     = str2jo("null");
      JO_THEN     = str2jo("then");
      JO_ELSE     = str2jo("else");

      JO_APPLY     = str2jo("apply");
      JO_END       = str2jo("end");

      JO_LOCAL_DATA_IN = str2jo("local-data-in");
      JO_LOCAL_DATA_OUT = str2jo("local-data-out");

      JO_LOCAL_TAG_IN = str2jo("local-tag-in");
      JO_LOCAL_TAG_OUT = str2jo("local-tag-out");

      JO_LOCAL_IN = str2jo("local-in");
      JO_LOCAL_OUT = str2jo("local-out");
    }
    init_jojo() {
      init_jotable();
      init_literal_jo();

      init_compiling_stack();
      init_jo_filter_stack();
      init_kernel_signal_handler();

      p_empty_jo();
      p_drop();

      expose_bind();
      expose_apply();
      expose_stack_operation();
      expose_ending();
      expose_bool();
      expose_int();
      expose_memory();
      expose_byte();
      expose_jo();
      expose_string();
      expose_file();
      expose_keyword();
      expose_system();
      expose_cffi();
      expose_top_level();
      expose_mise();

      expose_play();
    }
    #include "core.h"

    init_core() {
      FILE* core_file = fmemopen(core_jo, core_jo_len, "r");
      reading_stack_push(core_file);
      p_repl();
      reading_stack_pop();
      fclose(core_file);
    }
    main(int argc, char** argv) {
      cmd_number = argc;
      cmd_string_array = argv;
      init_jojo();
      init_core();
      p_repl();
    }
