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
    typedef enum { false, true } bool;
    typedef intptr_t cell;
    #define cell_size (sizeof(cell))
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
    typedef unsigned char byte;
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
      cell string_to_dec(char* str) { return string_to_based_int(str, 10); }
      cell string_to_bin(char* str) { return string_to_based_int(str,  2); }
      cell string_to_oct(char* str) { return string_to_based_int(str,  8); }
      cell string_to_hex(char* str) { return string_to_based_int(str, 16); }
    typedef void (*primitive)();
    typedef cell jo;
    typedef cell bind;

    typedef struct {
      cell index;
      char* key;
      jo tag;
      bind value;
      cell orbit_length;
      cell orbiton;
    } jotable_entry;
    jo JO_NOT_USED;

    bool jotable_entry_occured(jotable_entry e) {
      return e.key != 0;
    }

    bool jotable_entry_used(jotable_entry e) {
      return e.tag != JO_NOT_USED;
    }

    bool jotable_entry_no_collision(jotable_entry e) {
      return e.index == e.orbiton;
    }
    // prime table size
    //   1000003   about 976 k
    //   1000033
    //   1000333
    //   100003    about 97 k
    //   100333
    //   997
    #define jotable_size 100003
    jotable_entry jotable[jotable_size];
    cell jotable_counter = 0;
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
    bool jotable_keyeq(char* k1, char* k2) {
      return string_equal(k1, k2);
    }
    cell jotable_hash(char* key, cell counter) {
      return (counter + string_to_sum(key)) % jotable_size;
    }
    char string_area[4 * 1024 * 1024];
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
    // -1 denotes the hash_table is filled
    cell jotable_insert(char* key) {
      cell orbit_index = jotable_hash(key, 0);
      cell counter = 0;
      while (true) {
        cell index = jotable_hash(key, counter);
        if (!jotable_entry_occured(jotable[index])) {
          key = copy_to_string_area(key);
          jotable[index].key = key;
          jotable[index].orbiton = orbit_index;
          jotable[orbit_index].orbit_length = 1 + counter;
          jotable_counter = 1 + jotable_counter;
          return index;
        }
        else if (jotable_keyeq(key, jotable[index].key)) {
          return index;
        }
        else if (counter == jotable_size) {
          return -1;
        }
        else {
          counter = 1 + counter;
        }
      }
    }
    // -1 denotes key not occured
    cell jotable_search(char* key) {
      cell counter = 0;
      while (true) {
        cell index = jotable_hash(key, counter);
        if (!jotable_entry_occured(jotable[index])) {
          return -1;
        }
        else if (jotable_keyeq(key, jotable[index].key)) {
          return index;
        }
        else if (counter == jotable_size) {
          return -1;
        }
        else {
          counter = 1 + counter;
        }
      }
    }
    char* jo2str (cell index);

    void jotable_entry_print(jotable_entry entry) {
      printf("%s : ", jo2str(entry.tag));
      printf("%ld", entry.value);
    }
    void jotable_report_orbit(cell index, cell counter) {
      while (counter < jotable[index].orbit_length) {
        char* key = jotable[index].key;
        cell next_index = jotable_hash(key, counter);
        if (index == jotable[next_index].orbiton) {
          printf("  | %ld %s\n", next_index, jotable[next_index].key);
        }
        if (jotable_entry_used(jotable[next_index])) {
          printf("    = ");
          jotable_entry_print(jotable[next_index]);
          printf("\n");
        }
        counter = 1 + counter;
      }
    }
    void jotable_report() {
      printf("\n");
      printf("- jotable_report\n");
      printf("  : <index> <key> // <orbit-length>\n");
      cell index = 0;
      while (index < jotable_size) {
        if (jotable_entry_occured(jotable[index]) &&
            jotable_entry_no_collision(jotable[index])) {
          printf("  - %ld %s // %ld\n",
                 index, jotable[index].key, jotable[index].orbit_length);
          if (jotable_entry_used(jotable[index])) {
            printf("    = ");
            jotable_entry_print(jotable[index]);
            printf("\n");
          }
          jotable_report_orbit(index, 1);
        }
        index = 1 + index;
      }
      printf("  : <index> <key> // <orbit-length>\n");
      printf("\n");
      printf("- used : %ld\n", jotable_counter);
      printf("- free : %ld\n", jotable_size - jotable_counter);
    }
    void jotable_print() {
      printf("\n");
      printf("- jotable_print\n");
      cell index = 0;
      while (index < jotable_size) {
        printf("  - %ld %s %ld // %ld\n",
               index,
               jotable[index].key,
               jotable[index].value,
               jotable[index].orbit_length);
        index = 1 + index;
      }
      printf("\n");
      printf("- used : %ld\n", jotable_counter);
      printf("- free : %ld\n", jotable_size - jotable_counter);
    }
    jo str2jo(char* str) {
      return jotable_insert(str);
    }

    char* jo2str(cell index) {
      return jotable[index].key;
    }
    jo jojo_area[1024 * 1024];
      typedef jo* compiling_stack_t[1024];

      compiling_stack_t compiling_stack;
      cell compiling_stack_base = 0;
      cell compiling_stack_pointer = 0;

      void compiling_stack_push(jo* value) {
        compiling_stack[compiling_stack_pointer] = value;
        compiling_stack_pointer++;
      }

      jo* compiling_stack_pop() {
        compiling_stack_pointer--;
        return compiling_stack[compiling_stack_pointer];
      }

      void compiling_stack_inc() {
        compiling_stack[compiling_stack_pointer - 1] =
          compiling_stack[compiling_stack_pointer - 1] + 1;
      }


      jo* compiling_stack_tos() {
        return compiling_stack[compiling_stack_pointer - 1];
      }

      bool compiling_stack_empty_p() {
        return compiling_stack_pointer == compiling_stack_base;
      }
      void init_compiling_stack() {
        compiling_stack_push(jojo_area);
      }
      void here(cell n) {
        jo* pointer = compiling_stack_pop();
        pointer[0] = n;
        compiling_stack_push(pointer + 1);
      }
    void jotable_set_tag_and_value(cell index, jo tag, cell value) {
      jotable[index].tag = tag;
      jotable[index].value = value;
    }
    cell jotable_get_value(cell index) {
      return jotable[index].value;
    }
    jo EMPTY_JO;
    jo TAG_PRIM;
    jo TAG_JOJO;
    jo TAG_PRIM_KEYWORD;
    jo TAG_KEYWORD;
    jo TAG_DATA;

    jo JO_NOT_USED;
    jo JO_DECLARED;

    jo ROUND_BAR    ;
    jo ROUND_KET    ;
    jo SQUARE_BAR   ;
    jo SQUARE_KET   ;
    jo FLOWER_BAR   ;
    jo FLOWER_KET   ;
    jo DOUBLE_QUOTE ;

    jo JO_INS_INT;
    jo JO_INS_JUMP;
    jo JO_INS_JUMP_IF_FALSE;

    jo JO_NULL;
    jo JO_THEN;
    jo JO_ELSE;

    jo JO_APPLY;
    jo JO_END;

    jo JO_JO_REPLACING_APPLY_WITH_LAST_LOCAL_POINTER;
    jo JO_REPLACING_APPLY_WITH_LAST_LOCAL_POINTER;

    jo JO_LOCAL_DATA_IN;
    jo JO_LOCAL_DATA_OUT;

    jo JO_LOCAL_TAG_IN;
    jo JO_LOCAL_TAG_OUT;

    jo JO_LOCAL_IN;
    jo JO_LOCAL_OUT;
    typedef cell data_stack_t[1024 * 4];

    data_stack_t data_stack;
    cell data_stack_base = 64;
    cell data_stack_pointer = 64;

    void data_stack_push(cell value) {
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

    void return_stack_push(return_point value) {
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

    void return_stack_make_point(jo* jojo, cell local_pointer) {
      return_point rp = {.jojo = jojo, .local_pointer = local_pointer};
      return_stack[return_stack_pointer] = rp;
      return_stack_pointer++;
    }

    void return_stack_new_point(jo* jojo) {
      return_stack_make_point(jojo, current_local_pointer);
    }

    void return_stack_inc() {
      return_point rp = return_stack_pop();
      return_point rp1 = {.jojo = rp.jojo + 1, .local_pointer = rp.local_pointer};
      return_stack_push(rp1);
    }
    jo name_record[64 * 1024];
    cell name_record_counter = 0;
    void p_name_record() {
      data_stack_push(name_record);
    }
    void p_name_report() {
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

      void binding_filter_stack_push(jo value) {
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
      void p_binding_filter_stack_push() {
        binding_filter_stack_push(data_stack_pop());
      }
      void p_binding_filter_stack_pop() {
        data_stack_push(binding_filter_stack_pop());
      }
      void jo_apply_now(jo jo);

      void run_binding_filter() {
        cell i = binding_filter_stack_pointer;
        while (i > binding_filter_stack_base) {
          jo_apply_now(binding_filter_stack[i-1]);
          i--;
        }
      }
      typedef jo binding_hook_stack_t[128];
      binding_hook_stack_t binding_hook_stack;

      cell binding_hook_stack_base = 0;
      cell binding_hook_stack_pointer = 0;

      void binding_hook_stack_push(jo value) {
        binding_hook_stack[binding_hook_stack_pointer] = value;
        binding_hook_stack_pointer++;
      }

      jo binding_hook_stack_pop() {
        binding_hook_stack_pointer--;
        return binding_hook_stack[binding_hook_stack_pointer];
      }

      jo binding_hook_stack_tos() {
        return binding_hook_stack[binding_hook_stack_pointer - 1];
      }

      bool binding_hook_stack_empty_p() {
        return binding_hook_stack_pointer == binding_hook_stack_base;
      }
      void p_binding_hook_stack_push() {
        binding_hook_stack_push(data_stack_pop());
      }
      void p_binding_hook_stack_pop() {
        data_stack_push(binding_hook_stack_pop());
      }
      void run_binding_hook(cell name, jo tag, cell value) {
        cell i = binding_hook_stack_pointer;
        while (i > binding_hook_stack_base) {
          data_stack_push(value);
          data_stack_push(tag);
          data_stack_push(name);
          jo_apply_now(binding_hook_stack[i-1]);
          i--;
        }
      }
    bool used_jo_p(jo index) {
      return
        jotable[index].tag != JO_NOT_USED;
    }

    bool declared_jo_p(jo index) {
      return
        jotable[index].tag == JO_DECLARED;
    }

    void p_bind_name() {
      run_binding_filter();
      jo name = data_stack_pop();
      jo tag = data_stack_pop();
      cell value = data_stack_pop();
      if (used_jo_p(name) && !declared_jo_p(name)) {
        printf("- p_bind_name can not rebind\n");
        printf("  name : %s\n", jo2str(name));
        printf("  tag : %s\n", jo2str(tag));
        printf("  value : %ld\n", value);
        printf("  it has been bound as a %s\n", jo2str(jotable[name].tag));
        return;
      }
      jotable_set_tag_and_value(name, tag, value);

      run_binding_hook(name, tag, value);

      name_record[name_record_counter] = name;
      name_record_counter++;
      name_record[name_record_counter] = 0;
    }
    void define_prim(char* str, primitive fun) {
      jo name = str2jo(str);
      data_stack_push(fun);
      data_stack_push(TAG_PRIM);
      data_stack_push(name);
      p_bind_name();
    }
    void define_primkey(char* str, primitive fun) {
      jo name = str2jo(str);
      data_stack_push(fun);
      data_stack_push(TAG_PRIM_KEYWORD);
      data_stack_push(name);
      p_bind_name();
    }
    void export_bind() {
      define_prim("name-report", p_name_report);
      define_prim("name-record", p_name_record);

      define_prim("binding-filter-stack-push", p_binding_filter_stack_push);
      define_prim("binding-filter-stack-pop", p_binding_filter_stack_pop);

      define_prim("binding-hook-stack-push", p_binding_hook_stack_push);
      define_prim("binding-hook-stack-pop", p_binding_hook_stack_pop);
    }
    typedef cell keyword;
    typedef keyword keyword_stack_t[128];
    keyword_stack_t keyword_stack;

    cell keyword_stack_base = 0;
    cell keyword_stack_pointer = 0;

    void keyword_stack_push(keyword value) {
      keyword_stack[keyword_stack_pointer] = value;
      keyword_stack_pointer++;
    }

    keyword keyword_stack_pop() {
      keyword_stack_pointer--;
      return keyword_stack[keyword_stack_pointer];
    }

    keyword keyword_stack_tos() {
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

    void alias_stack_push(alias value) {
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
    void eval();
    void p_debug();

    void jo_apply(jo jo) {
      if (!jotable_entry_used(jotable[jo])) {
        printf("- jo_apply meet undefined jo : %s\n", jo2str(jo));
        p_debug();
      }

      cell tag = jotable[jo].tag;

      if (tag == TAG_PRIM) {
        primitive primitive = jotable_get_value(jo);
        primitive();
      }
      else if (tag == TAG_JOJO) {
        cell jojo = jotable_get_value(jo);
        return_stack_new_point(jojo);
      }

      else if (tag == TAG_PRIM_KEYWORD) {
        keyword_stack_push(alias_stack_pointer);
        primitive primitive = jotable_get_value(jo);
        primitive();
        alias_stack_pointer = keyword_stack_pop();
      }
      else if (tag == TAG_KEYWORD) {
        // keywords are always evaled
        keyword_stack_push(alias_stack_pointer);
        cell jojo = jotable_get_value(jo);
        return_stack_new_point(jojo);
        eval();
        alias_stack_pointer = keyword_stack_pop();
      }

      else if (tag == TAG_DATA) {
        cell cell = jotable_get_value(jo);
        data_stack_push(cell);
      }
      else {
        cell cell = jotable_get_value(jo);
        data_stack_push(cell);
        data_stack_push(tag);
      }
    }
    void jo_apply_now(jo jo) {
      cell tag = jotable[jo].tag;
      if (tag == TAG_JOJO) {
        cell jojo = jotable_get_value(jo);
        return_stack_new_point(jojo);
        eval();
        return;
      }
      else {
        jo_apply(jo);
        return;
      }
    }
    void jo_apply_with_local_pointer(jo jo, cell local_pointer) {
      cell tag = jotable[jo].tag;
      if (tag == TAG_JOJO) {
        cell jojo = jotable_get_value(jo);
        return_stack_make_point(jojo, local_pointer);
        return;
      }
      else {
        jo_apply(jo);
        return;
      }
    }
    bool step_flag = false;

    void stepper();

    void eval() {
      cell return_stack_base = return_stack_pointer;
      while (return_stack_pointer >= return_stack_base) {
        return_point rp = return_stack_tos();
        return_stack_inc();
        cell jo = *(cell*)rp.jojo;
        jo_apply(jo);
        if (step_flag == true) {
          stepper();
        }
      }
    }
    void p_apply() {
      return_stack_new_point(data_stack_pop());
    }
    void p_apply_with_local_pointer() {
      jo* jojo = data_stack_pop();
      cell local_pointer = data_stack_pop();
      return_stack_make_point(jojo, local_pointer);
    }
    void p_replacing_apply_with_last_local_pointer() {
      jo jojo = data_stack_pop();
      return_point rp = return_stack_pop();
      return_stack_make_point(jojo, rp.local_pointer);
    }
    void p_jo_apply() {
      jo_apply(data_stack_pop());
    }
    void p_jo_apply_with_local_pointer() {
      jo jo = data_stack_pop();
      cell local_pointer = data_stack_pop();
      jo_apply_with_local_pointer(jo, local_pointer);
    }
    void p_jo_replacing_apply_with_last_local_pointer() {
      jo jo = data_stack_pop();
      return_point rp = return_stack_pop();
      jo_apply_with_local_pointer(jo, rp.local_pointer);
    }
    void export_apply() {
      define_prim("apply", p_apply);
      define_prim("apply-with-local-pointer", p_apply_with_local_pointer);
      define_prim("replacing-apply-with-last-local-pointer", p_replacing_apply_with_last_local_pointer);

      define_prim("jo/apply", p_jo_apply);
      define_prim("jo/apply-with-local-pointer", p_jo_apply_with_local_pointer);
      define_prim("jo/replacing-apply-with-last-local-pointer", p_jo_replacing_apply_with_last_local_pointer);
    }
    void cell_copy(cell length, cell* from, cell* to) {
      cell i = 0;
      while (i < length) {
        to[i] = from[i];
        i++;
      }
    }
    void p_drop() {
      data_stack_pop();
    }
    void p_2drop() {
      data_stack_pop();
      data_stack_pop();
    }
    void p_dup() {
      // a a -> a
      cell a = data_stack_pop();
      data_stack_push(a);
      data_stack_push(a);
    }
    void p_2dup() {
      // b a -> b a b a
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(b);
      data_stack_push(a);
      data_stack_push(b);
      data_stack_push(a);
    }
    void p_over() {
      // b a -> b a b
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(b);
      data_stack_push(a);
      data_stack_push(b);
    }
    void p_2over() {
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
    void p_tuck() {
      // b a -> a b a
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(a);
      data_stack_push(b);
      data_stack_push(a);
    }
    void p_2tuck() {
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
    void p_swap() {
      // b a -> a b
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(a);
      data_stack_push(b);
    }
    void p_2swap() {
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
    void p_xy_swap() {
      // xxx yyy x y -> yyy xxx
      cell y = data_stack_pop();
      cell x = data_stack_pop();
      cell* yp = calloc(y, cell_size);
      cell* xp = calloc(x, cell_size);
      cell_copy(y, (data_stack + (data_stack_pointer - y)), yp);
      cell_copy(x, (data_stack + (data_stack_pointer - y - x)), xp);
      cell_copy(y, yp, (data_stack + (data_stack_pointer - y - x)));
      cell_copy(x, xp, (data_stack + (data_stack_pointer - x)));
      free(yp);
      free(xp);
    }
    void p_print_data_stack() {
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
    void p_stack_base() {
      data_stack_push(data_stack + data_stack_base);
    }
    void p_stack_pointer() {
      data_stack_push(data_stack + data_stack_pointer);
    }
    void export_stack_operation() {
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
      define_prim("xy-swap", p_xy_swap);
      define_prim("print-data-stack", p_print_data_stack);
      define_prim("stack-pointer", p_stack_pointer);
      define_prim("stack-base", p_stack_base);
    }
    void p_end() {
      return_point rp = return_stack_pop();
      current_local_pointer = rp.local_pointer;
    }
    void p_bye() {
      printf("bye bye ^-^/\n");
      exit(0);
    }
    void export_ending() {
      define_prim("end", p_end);
      define_prim("bye", p_bye);
    }
    void i_lit() {
      // [] -> [cell] {return_stack}
      return_point rp = return_stack_tos();
      return_stack_inc();
      cell jo = *(cell*)rp.jojo;
      data_stack_push(jo);
    }
    void i_jump_if_false() {
      // [bool] -> {return_stack}
      return_point rp = return_stack_tos();
      return_stack_inc();
      jo* a = *(cell*)rp.jojo;
      cell b = data_stack_pop();
      if (b == 0) {
        return_point rp1 = return_stack_pop();
        return_stack_make_point(a, rp1.local_pointer);
      }
    }
    void i_jump() {
      // {return_stack}
      return_point rp = return_stack_tos();
      jo* a = *(cell*)rp.jojo;
      return_point rp1 = return_stack_pop();
      return_stack_make_point(a, rp1.local_pointer);
    }
    void export_control() {
      define_prim("ins/lit", i_lit);
      define_prim("ins/jump-if-false", i_jump_if_false);
      define_prim("ins/jump", i_jump);
    }
    void p_true() {
      data_stack_push(1);
    }
    void p_false() {
      data_stack_push(0);
    }
    void p_not() {
      // bool -> bool
      cell a = data_stack_pop();
      data_stack_push(!a);
    }
    void p_and() {
      // bool bool -> bool
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(a&&b);
    }
    void p_or() {
      // bool bool -> bool
      cell a = data_stack_pop();
      cell b = data_stack_pop();
      data_stack_push(a||b);
    }
    void export_bool() {
      define_prim("true", p_true);
      define_prim("false", p_false);
      define_prim("not", p_not);
      define_prim("and", p_and);
      define_prim("or", p_or);
    }
    void p_true_bit() {
      // -> cell
      cell i = -1;
      data_stack_push(i);
    }
    void p_false_bit() {
      // -> cell
      data_stack_push(0);
    }
    void p_bit_and() {
      // cell cell -> cell
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a&b);
    }
    void p_bit_or() {
      // cell cell -> cell
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a|b);
    }
    void p_bit_xor() {
      // cell cell -> cell
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a^b);
    }
    void p_bit_not() {
      // cell -> cell
      cell a = data_stack_pop();
      data_stack_push(~a);
    }
    void p_bit_shift_left() {
      // cell step -> cell
      cell s = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a<<s);
    }
    void export_bit() {
      define_prim("true/bit", p_true_bit);
      define_prim("false/bit", p_false_bit);
      define_prim("bit/not", p_bit_not);
      define_prim("bit/and", p_bit_and);
      define_prim("bit/xor", p_bit_xor);
      define_prim("bit/or", p_bit_or);
      define_prim("bit/shift-left", p_bit_shift_left);
      // define_prim("bit/shift-right", p_bit_shift_right);
      // define_prim("bit/arithmetic-shift-right", p_bit_arithmetic_shift_right);
    }
    void p_inc() {
      cell a = data_stack_pop();
      data_stack_push(a + 1);
    }
    void p_dec() {
      cell a = data_stack_pop();
      data_stack_push(a - 1);
    }
    void p_neg() {
      cell a = data_stack_pop();
      data_stack_push(- a);
    }
    void p_add() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a + b);
    }
    void p_sub() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a - b);
    }
    void p_mul() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a * b);
    }
    void p_div() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a / b);
    }
    void p_mod() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a % b);
    }
    void p_n_eq_p() {
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
    void p_eq_p() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a == b);
    }
    void p_gt_p() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a > b);
    }
    void p_lt_p() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a < b);
    }
    void p_gteq_p() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a >= b);
    }
    void p_lteq_p() {
      cell b = data_stack_pop();
      cell a = data_stack_pop();
      data_stack_push(a <= b);
    }
    jo read_raw_jo();

    void k_int() {
      // (int ...)
      while (true) {
        jo s = read_raw_jo();
        if (s == ROUND_KET) {
          break;
        }
        else {
          here(JO_INS_INT);
          here(string_to_dec(jo2str(s)));
        }
      }
    }
    void p_int_print() { printf("%ld", data_stack_pop()); }
    void p_dot() { printf("%ld ", data_stack_pop()); }
    void p_int_dot() { printf("%ld ", data_stack_pop()); }
    void export_int() {
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

      define_primkey("int", k_int);

      define_prim("int/print", p_int_print);

      define_prim("dot", p_dot);
      define_prim("int/dot", p_int_dot);
    }
    void p_allocate () {
      // size -> addr
      data_stack_push(calloc(data_stack_pop(), 1));
    }
    void p_free () {
      // addr ->
      free(data_stack_pop());
    }
    void k_ignore();

    void k_address() {
      // (address ...)
      here(JO_INS_INT);
      jo index = read_raw_jo();
      here(&(jotable[index].value));
      k_ignore();
    }
    void p_jo_as_var() {
      jo jo = data_stack_pop();
      data_stack_push(&(jotable[jo].value));
    }
    void p_set_cell() {
      // cell address ->
      cell* address = data_stack_pop();
      cell value = data_stack_pop();
      address[0] = value;
    }
    void p_get_cell() {
      // address -> cell
      cell* address = data_stack_pop();
      data_stack_push(address[0]);
    }
    void p_set_byte() {
      // byte address ->
      char* address = data_stack_pop();
      cell value = data_stack_pop();
      address[0] = value;
    }
    void p_get_byte() {
      // address -> byte
      char* address = data_stack_pop();
      data_stack_push(address[0]);
    }
    void export_memory() {
      define_prim("allocate", p_allocate);
      define_prim("free", p_free);
      define_primkey("address", k_address);
      define_prim("jo-as-var", p_jo_as_var);
      define_prim("set-cell", p_set_cell);
      define_prim("get-cell", p_get_cell);
      define_prim("set-byte", p_set_byte);
      define_prim("get-byte", p_get_byte);
    }
    typedef struct {
      FILE* file_handle;
      char* dir;
    } reading_point;

    typedef reading_point reading_stack_t[64];

    reading_stack_t reading_stack;
    cell reading_stack_base = 0;
    cell reading_stack_pointer = 0;

    void reading_stack_push(reading_point value) {
      reading_stack[reading_stack_pointer] = value;
      reading_stack_pointer++;
    }

    reading_point reading_stack_pop() {
      reading_stack_pointer--;
      return reading_stack[reading_stack_pointer];
    }

    reading_point reading_stack_tos() {
      return reading_stack[reading_stack_pointer - 1];
    }

    bool reading_stack_empty_p() {
      return reading_stack_pointer == reading_stack_base;
    }
    void real_reading_path(char* path, char* buffer) {
      if (path[0] == '/') {
        realpath(path, buffer);
        return;
      }
      else if (reading_stack_empty_p()) {
        realpath(path, buffer);
        return;
      }
      else {
        buffer[0] = 0;
        strcat(buffer, reading_stack_tos().dir);
        strcat(buffer, "/");
        strcat(buffer, path);
        return;
      }
    }
    bool has_byte_p() {
    FILE* file;
      if (reading_stack_empty_p()) {
        file = stdin;
      }
      else {
        file = reading_stack_tos().file_handle;
      }

      if (feof(file) == 0) {
        return true;
      }
      else {
        return false;
      }
    }
    void p_has_byte_p() {
      data_stack_push(has_byte_p());
    }
    byte read_byte() {
      if (reading_stack_empty_p()) {
        return fgetc(stdin);
      }
      else {
        return fgetc(reading_stack_tos().file_handle);
      }
    }
    void byte_unread(byte c) {
      if (reading_stack_empty_p()) {
        ungetc(c, stdin);
      }
      else {
        ungetc(c, reading_stack_tos().file_handle);
      }
    }
    void p_read_byte() {
      // -> byte
      data_stack_push(read_byte());
    }
    void p_byte_unread() {
      // byte -> {reading_stack}
      byte_unread(data_stack_pop());
    }
    void p_byte_print() {
      // byte ->
      printf("%c", data_stack_pop());
    }
    void k_byte() {
      // (byte ...)

    }
    void export_byte() {
      define_prim("has-byte?", p_has_byte_p);
      define_prim("read/byte", p_read_byte);
      define_prim("byte/unread", p_byte_unread);
      define_prim("byte/print", p_byte_print);
    }
    void k_one_string() {
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
      here(JO_INS_INT);
      here(str);
    }
    void k_string() {
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
    void p_string_length() {
      // string -> length
      data_stack_push(strlen(data_stack_pop()));
    }
    void p_string_print() {
      // string -> {terminal-output}
      printf("%s", data_stack_pop());
    }
    void p_string_dot() {
      // string -> {terminal-output}
      printf("\"%s \"", data_stack_pop());
    }
    void p_string_append_to_buffer() {
      // buffer string -> buffer
      char* str = data_stack_pop();
      char* buffer = data_stack_tos();
      strcat(buffer, str);
    }
    void p_string_first_byte() {
      char* s = data_stack_pop();
      data_stack_push(s[0]);
    }
    void p_string_last_byte() {
      char* s = data_stack_pop();
      cell i = 0;
      while (s[i+1] != 0) {
        i++;
      }
      data_stack_push(s[i]);
    }
    void p_string_member_p() {
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
    void p_string_find_byte() {
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
    void export_string() {
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
    }
    void p_alias_push() {
      jo name = data_stack_pop();
      jo nick = data_stack_pop();
      alias a = {.nick = nick, .name = name};
      alias_stack_push(a);
    }
    void p_alias_filter() {
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
    void p_has_jo_p() {
      data_stack_push(has_jo_p());
    }
    void p_read_raw_jo() {
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

      void jo_filter_stack_push(reading_filter value) {
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
      void p_jo_filter_stack_push() {
        jo_filter_stack_push(data_stack_pop());
      }
      void p_jo_filter_stack_pop() {
        data_stack_push(jo_filter_stack_pop());
      }
      void run_jo_filter() {
        cell i = jo_filter_stack_pointer;
        while (i > jo_filter_stack_base) {
          jo_apply_now(jo_filter_stack[i-1]);
          i--;
        }
      }
      void init_jo_filter_stack() {
        jo_filter_stack_push(str2jo("alias-filter"));
      }
    void p_read_jo() {
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
    void p_jo_append() {
      jo jo2 = data_stack_pop();
      jo jo1 = data_stack_pop();
      data_stack_push(cat_2_jo(jo1, jo2));
    }
    void p_empty_jo() {
      data_stack_push(EMPTY_JO);
    }
    void p_jo_used_p() {
      // jo -> bool
      jo jo = data_stack_pop();
      data_stack_push(jotable_entry_used(jotable[jo]));
    }
    void p_jo_to_string() {
      // jo -> string
      jo jo = data_stack_pop();
      data_stack_push(jo2str(jo));
    }
    void p_string_length_to_jo() {
      // string length -> jo
      cell len = data_stack_pop();
      cell str = data_stack_pop();
      char buffer[2 * 1024];
      strncpy(buffer, str, len);
      buffer[len] = 0;
      data_stack_push(str2jo(buffer));
    }
    void p_string_to_jo() {
      // string -> jo
      char* str = data_stack_pop();
      data_stack_push(str2jo(str));
    }
    void p_null() {
      data_stack_push(JO_NULL);
    }
    void k_raw_jo() {
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
          here(JO_INS_INT);
          here(s);
        }
      }
    }
    void k_jo() {
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
          here(JO_INS_INT);
          here(s);
        }
      }
    }
    void p_jo_print() {
      // jo -> {terminal-output}
      printf("%s", jo2str(data_stack_pop()));
    }
    void p_jo_dot() {
      // jo -> {terminal-output}
      printf("%s ", jo2str(data_stack_pop()));
    }
    cell p_generate_jo_counter = 0;
    void p_generate_jo() {
      char* s = data_stack_pop();
      char buffer [1024];
      sprintf(buffer, "%s:generated-jo#%ld", jo2str(s), p_generate_jo_counter);
      p_generate_jo_counter++;
      data_stack_push(str2jo(buffer));
    }
    void p_jo_find_byte() {
      // byte jo -> [index true] or [false]
      p_jo_to_string();
      p_string_find_byte();
    }
    void p_jo_right_part() {
      // index jo -> jo
      jo jo = data_stack_pop();
      cell index = data_stack_pop();
      char* s = jo2str(jo);
      data_stack_push(str2jo(s + index));
    }
    void p_jo_left_part() {
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
    void p_jo_part() {
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
    void export_jo() {
      define_prim("null", p_null);

      define_prim("jo-filter-stack-push", p_jo_filter_stack_push);
      define_prim("jo-filter-stack-pop", p_jo_filter_stack_pop);

      define_prim("alias-push", p_alias_push);
      define_prim("alias-filter", p_alias_filter);

      define_prim("has-jo?", p_has_jo_p);

      define_prim("read/raw-jo", p_read_raw_jo);
      define_prim("read/jo", p_read_jo);
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
    void p_error_number_print() {
      // errno -> {terminal-output}
      int no = data_stack_pop();
      printf("%s", strerror(no));
    }
    void p_path_open_read() {
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
    void p_path_open_write() {
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
    void p_path_open_read_and_write() {
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
    void p_path_open_create() {
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
    void p_file_close() {
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
    void p_file_end_p() {
      // file -> true or false
      FILE* file = data_stack_pop();

      if (feof(file)) {
        data_stack_push(true);
      }
      else {
        data_stack_push(false);
      }
    }
    void p_file_read() {
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
    void p_file_write() {
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
    void p_file_size() {
      // file -> int
      FILE* file = data_stack_pop();
      struct stat file_state;
      fstat(fileno(file), &file_state);
      data_stack_push(file_state.st_size);
    }
    void p_file_regular_file_p() {
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
    void p_file_directory_p() {
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
    void p_file_character_device_p() {
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
    void p_file_block_device_p() {
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
    void p_file_fifo_p() {
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
    void p_file_socket_p() {
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
    void p_path_exist_p() {
      // path -> true or false
      char* path = data_stack_pop();

      if (access(path, F_OK) == -1) {
        data_stack_push(false);
      }
      else {
        data_stack_push(true);
      }
    }
    void p_path_readable_p() {
      // path -> true or false
      char* path = data_stack_pop();

      if (access(path, R_OK) == -1) {
        data_stack_push(false);
      }
      else {
        data_stack_push(true);
      }
    }
    void p_path_writable_p() {
      // path -> true or false
      char* path = data_stack_pop();

      if (access(path, W_OK) == -1) {
        data_stack_push(false);
      }
      else {
        data_stack_push(true);
      }
    }
    void p_path_executable_p() {
      // path -> true or false
      char* path = data_stack_pop();

      if (access(path, X_OK) == -1) {
        data_stack_push(false);
      }
      else {
        data_stack_push(true);
      }
    }
    void p_file_print_path() {
      // file -> path
      FILE* file = data_stack_pop();

      char proclnk[PATH_MAX];
      char filename[PATH_MAX];

      sprintf(proclnk, "/proc/self/fd/%d", fileno(file));

      ssize_t real_bytes = readlink(proclnk, filename, PATH_MAX);
      if (real_bytes == -1) {
        printf("- p_file_print_path fail readlink /proc/self/fd/%d\n", fileno(file));
        perror("\n");
      }
      else {
        filename[real_bytes] = '\0';
        printf("%s", filename);
      }
    }
    void p_repl();

    void p_path_load() {
      // path -> {reading_stack}
      char* path = data_stack_pop();
      FILE* file = fopen(path, "r");
      if(file == NULL) {
        printf("- p_path_load fail : %s\n", path);
        perror("File opening failed");
        return;
      }

      char* dir_buffer = malloc(PATH_MAX);
      realpath(path, dir_buffer);
      char* dir_addr = dirname(dir_buffer);

      reading_point rp = {
        .file_handle = file,
        .dir = dir_addr
      };
      reading_stack_push(rp);

      p_repl();

      reading_stack_pop();
      fclose(rp.file_handle);
      free(rp.dir);
    }
    void k_one_include() {
      // "..."
      char buffer[PATH_MAX];
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
      char buffer1[PATH_MAX];
      real_reading_path(buffer, buffer1);
      data_stack_push(buffer1);
      p_path_load();
    }
    void k_include() {
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
    void export_file() {
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
    void p_command_run() {
      // string -> {*}
      system(data_stack_pop());
    }
    void p_n_command_run() {
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
    cell argument_counter;

    void p_argument_counter() {
      // -> argument_counter
      data_stack_push(argument_counter);
    }
    char** argument_string_array;

    void p_index_to_argument_string() {
      // index -> string
      cell index = data_stack_pop();
      char* argument_string = argument_string_array[index];
      data_stack_push(argument_string);
    }
    void p_get_env_string() {
      // string -> string
      char* var_string = data_stack_pop();
      char* env_string = getenv(var_string);
      data_stack_push(env_string);
    }
    void export_system() {
      define_prim("command/run", p_command_run);
      define_prim("n-command/run", p_n_command_run);
      define_prim("argument-counter", p_argument_counter);
      define_prim("index->argument-string", p_index_to_argument_string);
      define_prim("get-env-string", p_get_env_string);
    }
    void ccall (char* str, void* lib) {
      primitive fun = dlsym(lib, str);
      if (fun == NULL) {
        printf("can not find %s function lib : %s\n",
               str, dlerror());
      };
      fun();
    }
    void* get_clib(char* rel_path) {
      char path[PATH_MAX];
      real_reading_path(rel_path, path);
      void* lib = dlopen(path, RTLD_LAZY);
      if (lib == NULL) {
        printf("fail to open library : %s : %s\n",
               path, dlerror());
      };
      return lib;
    }
    void k_clib_one() {
      // "..."
      char buffer[PATH_MAX];
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
      ccall("export", get_clib(buffer));
    }
    void k_clib() {
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
    void export_cffi() {
      define_prim("clib", k_clib);
    }
    void k_run();

    void k_define() {
      jo name = read_jo();
      k_run();
      data_stack_push(name);
      p_bind_name();
    }
      void k_declare_one() {
        jo index = read_jo();
        jotable[index].tag = JO_DECLARED;
        k_ignore();
      }
      void k_declare() {
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
    void p_compile_jojo();

    void k_run() {
      // (run ...)
      jo* jojo = compiling_stack_tos();
      p_compile_jojo();
      return_stack_new_point(jojo);
      eval();
    }
    bool testing_flag = false;
    void p_testing_flag() { data_stack_push(testing_flag); }
    void p_testing_flag_on() { testing_flag = true; }
    void p_testing_flag_off() { testing_flag = false; }
    void k_test() {
      if (testing_flag) {
        k_run();
      }
      else {
        k_ignore();
      }
    }
    bool top_repl_printing_flag = false;

    void p_print_data_stack_by_flag() {
      if (top_repl_printing_flag) {
        p_print_data_stack();
      }
    }

    void p_repl() {
      while (true) {
        if (!has_jo_p()) {
          return;
        }
        jo s = read_jo();
        if (s == ROUND_BAR) {
          jo_apply(read_jo());
          p_print_data_stack_by_flag();
        }
        else {
          // loop
        }
      }
    }
    void p_repl_printing_flag() { data_stack_push(top_repl_printing_flag); }
    void p_repl_printing_flag_on() { top_repl_printing_flag = true; }
    void p_repl_printing_flag_off() { top_repl_printing_flag = false; }
    void p_bare_jojo_print() {
      // jojo -> {terminal-output}
      jo* jojo = data_stack_pop();
      printf("[ ");
      while (true) {
        if (jojo[0] == 0 && jojo[1] == 0) {
          break;
        }
        if (jojo[0] == JO_INS_INT ||
            jojo[0] == JO_INS_JUMP ||
            jojo[0] == JO_INS_JUMP_IF_FALSE) {
          printf("%s ", jo2str(jojo[0]));
          printf("%ld ", jojo[1]);
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
    void point_local(cell begin, cell end) {
      cell i = begin;
      while (i < end) {
        printf("{%s = %ld %s} "
               , jo2str(local_area[i].name)
               , local_area[i].local_data
               , jo2str(local_area[i].local_tag));
        i++;
      }
    }

    void p_print_return_stack() {
      cell i = return_stack_base;
      printf("  - return-stack :\n");

      while (i < return_stack_pointer -1) {

        printf("    - ");
        data_stack_push(return_stack[i].jojo);
        p_bare_jojo_print();
        printf("\n");

        if (return_stack[i].local_pointer ==
            return_stack[i+1].local_pointer) {
        }
        else {
          printf("      ");
          point_local(return_stack[i].local_pointer,
                      return_stack[i+1].local_pointer);
          printf("\n");
        }

        i++;
      }

      printf("    - ");
      data_stack_push(return_stack[i].jojo);
      p_bare_jojo_print();
      printf("\n");

      if (return_stack[i].local_pointer ==
          current_local_pointer) {
      }
      else {
        printf("      ");
        point_local(return_stack[i].local_pointer,
                    current_local_pointer);
        printf("\n");
      }
    }
    cell debug_repl_level = 0;

    void p_debug_repl() {
      while (true) {
        if (!has_jo_p()) {
          return;
        }
        jo s = read_jo();
        if (s == str2jo("exit")) {
          return;
        }
        if (s == str2jo("bye")) {
          p_bye();
          return;
        }
        else if (s == ROUND_BAR) {
          jo_apply(read_jo());
          p_print_data_stack();
          printf("debug[%ld]> ", debug_repl_level);
        }
        else {
          // loop
        }
      }
    }
    void p_debug() {
      char current_dir[PATH_MAX];
      getcwd(current_dir, PATH_MAX);

      reading_point rp = {
        .file_handle = stdin,
        .dir = current_dir
      };
      reading_stack_push(rp);

      printf("- in debug-repl [level %ld] >_<!\n", debug_repl_level);
      printf("  available commends : exit bye\n");
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

    void exit_stepper() {
      step_flag = false;
      stepper_counter = 0;
      pending_steps = 0;
      printf("- exit stepper\n");
    }

    void stepper() {
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
          printf("  - available commands in stepper :\n");
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
          pending_steps = string_to_dec(jo2str(jo)) - 1;
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
    void p_step() {
      step_flag = true;
    }
      void kernel_signal_handler(int sig, siginfo_t *siginfo, void *ucontext) {
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
      void init_kernel_signal_handler() {
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
    void export_top_level() {
      define_primkey("define", k_define);
      define_primkey("bind-name", p_bind_name);
      define_primkey("declare", k_declare);

      define_primkey("run", k_run);

      define_primkey("test", k_test);
      define_prim("testing-flag", p_testing_flag);
      define_prim("testing-flag/on", p_testing_flag_on);
      define_prim("testing-flag/off", p_testing_flag_off);

      define_prim("print-data-stack/by-flag", p_print_data_stack_by_flag);
      define_prim("repl", p_repl);
      define_prim("repl/printing-flag", p_repl_printing_flag);
      define_prim("repl/printing-flag/on", p_repl_printing_flag_on);
      define_prim("repl/printing-flag/off", p_repl_printing_flag_off);

      define_prim("bare-jojo/print", p_bare_jojo_print);
      define_prim("print-return-stack", p_print_return_stack);
      define_prim("debug", p_debug);

      define_prim("step", p_step);
    }
    void k_ignore() {
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
    void compile_until_meet_jo(jo ending_jo) {
      while (true) {
        jo s = read_jo();
        if (s == ROUND_BAR) {
          jo_apply(read_jo());
        }
        else if (s == ending_jo) {
          break;
        }
        else if (jotable_entry_used(jotable[s])) {
          here(s);
        }
        else {
          // no compile before define
          printf("- compile_until_meet_jo undefined : %s\n", jo2str(s));
          k_ignore();
          p_debug();
          return;
        }
      }
    }
    void p_compile_until_meet_jo() {
      compile_until_meet_jo(data_stack_pop());
    }
    jo compile_until_meet_jo_or_jo(jo ending_jo1, jo ending_jo2) {
      while (true) {
        jo s = read_jo();
        if (s == ROUND_BAR) {
          jo_apply(read_jo());
        }
        else if (s == ending_jo1 || s == ending_jo2) {
          return s;
        }
        else if (jotable_entry_used(jotable[s])) {
          here(s);
        }
        else {
          // no compile before define
          printf("- compile_until_meet_jo_or_jo undefined : %s\n", jo2str(s));
          printf("- ending_jo1 : %s\n", jo2str(ending_jo1));
          printf("- ending_jo2 : %s\n", jo2str(ending_jo2));
          k_ignore();
          p_debug();
          return JO_NULL; // this is to fool the compiler
        }
      }
    }
    void p_compile_until_round_ket() {
      compile_until_meet_jo(ROUND_KET);
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

    void k_if() {
      compile_until_meet_jo(JO_THEN);
      here(JO_INS_JUMP_IF_FALSE);
      cell* end_of_then = compiling_stack_tos();
      compiling_stack_inc();
      jo ending_jo = compile_until_meet_jo_or_jo(JO_ELSE, ROUND_KET);
      if (ending_jo == ROUND_KET) {
        end_of_then[0] = compiling_stack_tos();
        return;
      }
      else {
        here(JO_INS_JUMP);
        cell* end_of_else = compiling_stack_tos();
        compiling_stack_inc();
        end_of_then[0] = compiling_stack_tos();
        p_compile_until_round_ket();
        end_of_else[0] = compiling_stack_tos();
        return;
      }
    }
    void k_tail_call() {
      // no check for "no compile before define"
      here(JO_INS_INT);
      here(read_jo());
      here(JO_JO_REPLACING_APPLY_WITH_LAST_LOCAL_POINTER);
      k_ignore();
    }
    typedef jo current_compiling_jojo_stack_t[1024];

    current_compiling_jojo_stack_t current_compiling_jojo_stack;
    cell current_compiling_jojo_stack_base = 0;
    cell current_compiling_jojo_stack_pointer = 0;

    void current_compiling_jojo_stack_push(jo* value) {
      current_compiling_jojo_stack[current_compiling_jojo_stack_pointer] = value;
      current_compiling_jojo_stack_pointer++;
    }

    jo* current_compiling_jojo_stack_pop() {
      current_compiling_jojo_stack_pointer--;
      return current_compiling_jojo_stack[current_compiling_jojo_stack_pointer];
    }

    void current_compiling_jojo_stack_inc() {
      current_compiling_jojo_stack[current_compiling_jojo_stack_pointer - 1] =
        current_compiling_jojo_stack[current_compiling_jojo_stack_pointer - 1] + 1;
    }


    jo* current_compiling_jojo_stack_tos() {
      return current_compiling_jojo_stack[current_compiling_jojo_stack_pointer - 1];
    }

    bool current_compiling_jojo_stack_empty_p() {
      return current_compiling_jojo_stack_pointer == current_compiling_jojo_stack_base;
    }
    void p_compile_jojo() {
      jo* jojo = compiling_stack_tos();
      current_compiling_jojo_stack_push(jojo);
      compile_until_meet_jo(ROUND_KET);
      here(JO_END);
      here(0);
      here(0);
      current_compiling_jojo_stack_pop();
    }
    void k_loop() {
      here(JO_INS_INT);
      here(current_compiling_jojo_stack_tos());
      here(JO_REPLACING_APPLY_WITH_LAST_LOCAL_POINTER);
      k_ignore();
    }
    void k_recur() {
      here(JO_INS_INT);
      here(current_compiling_jojo_stack_tos());
      here(JO_APPLY);
      k_ignore();
    }
    void p_compiling_stack_tos() {
      data_stack_push(compiling_stack_tos());
    }
    void k_bare_jojo() {
      // (bare-jojo ...)
      here(JO_INS_JUMP);
      cell* offset_place = compiling_stack_tos();
      compiling_stack_inc();
      p_compile_jojo();
      offset_place[0] = compiling_stack_tos();
      here(JO_INS_INT);
      here(offset_place + 1);
    }
    void k_jojo() {
      // (jojo ...)
      here(JO_INS_JUMP);
      cell* offset_place = compiling_stack_tos();
      compiling_stack_inc();
      p_compile_jojo();
      offset_place[0] = compiling_stack_tos();
      here(JO_INS_INT);
      here(offset_place + 1);
      here(JO_INS_INT);
      here(TAG_JOJO);
    }
    void k_keyword() {
      // (keyword ...)
      here(JO_INS_JUMP);
      cell* offset_place = compiling_stack_tos();
      compiling_stack_inc();
      p_compile_jojo();
      offset_place[0] = compiling_stack_tos();
      here(JO_INS_INT);
      here(offset_place + 1);
      here(JO_INS_INT);
      here(TAG_KEYWORD);
    }
    void k_data() {
      // (data ...)
      p_compile_until_round_ket();
      here(JO_INS_INT);
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
    void p_local_data_in() {
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
    void p_local_data_out() {
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
      }
    }
    void p_local_tag_in() {
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
    void p_local_tag_out() {
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
      }
    }
    void p_local_in() {
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
    void p_local_out() {
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
      }
    }
    void k_local_data_in() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        k_local_data_in();

        here(JO_INS_INT);
        here(s);
        here(JO_LOCAL_DATA_IN);
      }
    }
    void k_local_data_out() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        here(JO_INS_INT);
        here(s);
        here(JO_LOCAL_DATA_OUT);

        k_local_data_out();
      }
    }
    void k_local_tag_in() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        k_local_data_in();

        here(JO_INS_INT);
        here(s);
        here(JO_LOCAL_TAG_IN);
      }
    }
    void k_local_tag_out() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        here(JO_INS_INT);
        here(s);
        here(JO_LOCAL_TAG_OUT);

        k_local_data_out();
      }
    }
    void k_local_in() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        k_local_data_in();

        here(JO_INS_INT);
        here(s);
        here(JO_LOCAL_IN);
      }
    }
    void k_local_out() {
      jo s = read_raw_jo();
      if (s == ROUND_KET) {
        return;
      }
      else {
        here(JO_INS_INT);
        here(s);
        here(JO_LOCAL_OUT);

        k_local_data_out();
      }
    }
    void p_current_local_pointer() {
      data_stack_push(current_local_pointer);
    }
    void export_keyword() {
      define_primkey("ignore", k_ignore);
      define_primkey("note", k_ignore);

      define_prim("compiling-stack/tos", p_compiling_stack_tos);
      define_prim("compiling-stack/inc", compiling_stack_inc);

      define_primkey("if", k_if);
      define_prim("compile-until-meet-jo", p_compile_until_meet_jo);
      define_prim("compile-until-round-ket", p_compile_until_round_ket);

      define_primkey("else", p_compile_until_round_ket);
      define_primkey("el", p_compile_until_round_ket);

      define_prim("compile-jojo", p_compile_jojo);

      define_primkey("tail-call", k_tail_call);
      define_primkey("loop", k_loop);
      define_primkey("recur", k_recur);

      define_primkey("data", k_data);
      define_primkey("jojo", k_jojo);
      define_primkey("keyword", k_keyword);

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
    }
    void do_nothing() {
    }
    void p_here() {
      here(data_stack_pop());
    }
    void p_address_of_here() {
     data_stack_push(compiling_stack_tos());
    }
    void p_round_bar()    { data_stack_push(ROUND_BAR); }
    void p_round_ket()    { data_stack_push(ROUND_KET); }
    void p_square_bar()   { data_stack_push(SQUARE_BAR); }
    void p_square_ket()   { data_stack_push(SQUARE_KET); }
    void p_flower_bar()   { data_stack_push(FLOWER_BAR); }
    void p_flower_ket()   { data_stack_push(FLOWER_KET); }
    void p_double_quote() { data_stack_push(DOUBLE_QUOTE); }
    void p_cell_size() {
      // -> cell
      data_stack_push(cell_size);
    }
    void p_newline() {
      printf("\n");
    }
    void export_mise() {
      define_prim("here", p_here);
      define_prim("address-of-here", p_address_of_here);

      define_prim("jotable/report", jotable_report);

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
    jotable_entry proto_jotable_entry(cell index) {
      jotable_entry e = {
        .index = index,
        .key = 0,
        .tag = str2jo("not-used"),
        .value = 0,
        .orbit_length = 0,
        .orbiton = 0
      };
      return e;
    }

    void init_jotable() {
      cell i = 0;
      while (i < jotable_size) {
        jotable[i] = proto_jotable_entry(i);
        i++;
      }
    }
    void init_literal_jo() {
      EMPTY_JO = str2jo("");

      TAG_PRIM         = str2jo("<prim>");
      TAG_JOJO         = str2jo("<jojo>");
      TAG_PRIM_KEYWORD = str2jo("<prim-keyword>");
      TAG_KEYWORD      = str2jo("<keyword>");
      TAG_DATA         = str2jo("<data>");

      JO_NOT_USED = str2jo("not-used");
      JO_DECLARED = str2jo("declared");

      ROUND_BAR    =   str2jo("(");
      ROUND_KET    =   str2jo(")");
      SQUARE_BAR   =   str2jo("[");
      SQUARE_KET   =   str2jo("]");
      FLOWER_BAR   =   str2jo("{");
      FLOWER_KET   =   str2jo("}");
      DOUBLE_QUOTE =   str2jo("\"");

      JO_INS_INT  = str2jo("ins/lit");
      JO_INS_JUMP = str2jo("ins/jump");
      JO_INS_JUMP_IF_FALSE = str2jo("ins/jump-if-false");

      JO_NULL     = str2jo("null");
      JO_THEN     = str2jo("then");
      JO_ELSE     = str2jo("else");

      JO_APPLY     = str2jo("apply");
      JO_END       = str2jo("end");

      JO_JO_REPLACING_APPLY_WITH_LAST_LOCAL_POINTER = str2jo("jo/replacing-apply-with-last-local-pointer");
      JO_REPLACING_APPLY_WITH_LAST_LOCAL_POINTER = str2jo("replacing-apply-with-last-local-pointer");

      JO_LOCAL_DATA_IN = str2jo("local-data-in");
      JO_LOCAL_DATA_OUT = str2jo("local-data-out");

      JO_LOCAL_TAG_IN = str2jo("local-tag-in");
      JO_LOCAL_TAG_OUT = str2jo("local-tag-out");

      JO_LOCAL_IN = str2jo("local-in");
      JO_LOCAL_OUT = str2jo("local-out");
    }
    void init_jojo() {

      init_jotable();
      init_literal_jo();

      init_compiling_stack();
      init_jo_filter_stack();
      init_kernel_signal_handler();

      p_empty_jo();
      p_drop();

      export_bind();
      export_apply();
      export_stack_operation();
      export_ending();
      export_control();
      export_bool();
      export_bit();
      export_int();
      export_memory();
      export_byte();
      export_jo();
      export_string();
      export_file();
      export_keyword();
      export_system();
      export_cffi();
      export_top_level();
      export_mise();
    }
    #include "core.h";

    void init_core() {
      FILE* core_file = fmemopen(core_jo, core_jo_len, "r");

      char current_dir[PATH_MAX];
      getcwd(current_dir, PATH_MAX);

      reading_point rp = {
        .file_handle = core_file,
        .dir = current_dir
      };
      reading_stack_push(rp);

      p_repl();

      reading_stack_pop();
      fclose(rp.file_handle);
    }
    int main(int argc, char** argv) {
      argument_counter = argc;
      argument_string_array = argv;
      init_jojo();
      init_core();
      p_repl();
    }
