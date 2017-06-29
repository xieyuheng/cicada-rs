#include <sys/types.h>  /* Type definitions used by many programs */
#include <stdio.h>      /* Standard I/O functions */
#include <stdlib.h>     /* Prototypes of commonly used library functions,
                           plus EXIT_SUCCESS and EXIT_FAILURE constants */
#include <unistd.h>     /* Prototypes for many system calls */
#include <errno.h>      /* Declares errno and defines error constants */
#include <string.h>     /* Commonly used string-handling functions */

#include <fcntl.h>

#include <sys/stat.h>
#include <ctype.h>
#include <stdint.h>
#include <setjmp.h>
#include <dlfcn.h>
#include <dirent.h>

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

typedef char* string;

bool string_equal(string s1, string s2) {
  if (strcmp(s1, s2) == 0) {
    return true;
  }
  else {
    return false;
  }
}

bool nat_string_p(string str) {
  cell i = 0;
  while (str[i] != 0) {
    if (!isdigit(str[i])) {
      return false;
      }
    i++;
  }
  return true;
}

bool int_string_p(string str) {
  if (str[0] == '-' ||
      str[0] == '+') {
    return nat_string_p(str + 1);
  }
  else {
    return nat_string_p(str);
  }
}

cell string_to_based_nat(string str, cell base) {
  cell result = 0;
  cell len = strlen(str);
  cell i = 0;
  while (i < len) {
    result = result + (char_to_nat(str[i]) * power(base, (len - i - 1)));
    i++;
  }
  return result;
}

cell string_to_based_int(string str, cell base) {
  if (str[0] == '-') {
    return - string_to_based_nat(str, base);
  }
  else {
    return string_to_based_nat(str, base);
  }
}

cell string_to_dec(string str) { return string_to_based_int(str, 10); }
cell string_to_bin(string str) { return string_to_based_int(str,  2); }
cell string_to_oct(string str) { return string_to_based_int(str,  8); }
cell string_to_hex(string str) { return string_to_based_int(str, 16); }

typedef void (*primitive)();

typedef cell jo;

typedef struct {
  cell size;
  jo* array;
} jojo;

typedef cell bind;

typedef struct {
  cell index;
  string key;
  jo tag;
  bind value;
  cell orbit_length;
  cell orbiton;
} jotable_entry;

jo str2jo (string str);

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

bool jotable_entry_occured(jotable_entry e) {
  return e.key != 0;
}

bool jotable_entry_used(jotable_entry e) {
  return e.tag != str2jo("not-used");
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

cell string_to_sum(string str) {
  cell sum = 0;
  cell max_step = 10;
  cell i = 0;
  while (i < strlen(str)) {
    sum = sum + ((byte) str[i]) * (2 << min(i, max_step));
    i++;
  }
  return sum;
}

bool jotable_keyeq(string k1, string k2) {
  return string_equal(k1, k2);
}

cell jotable_hash(string key, cell counter) {
  return (counter + string_to_sum(key)) % jotable_size;
}

char string_area[4 * 1024 * 1024];
cell string_area_counter = 0;

string copy_to_string_area(string str) {
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
cell jotable_insert(string key) {
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
cell jotable_search(string key) {
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

string jo2str (cell index);

void jotable_entry_print(jotable_entry entry) {
  printf("%s : ", jo2str(entry.tag));
  printf("%ld", entry.value);
}

void jotable_report_orbit(cell index, cell counter) {
  while (counter < jotable[index].orbit_length) {
    string key = jotable[index].key;
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

jo str2jo(string str) {
  return jotable_insert(str);
}

string jo2str(cell index) {
  return jotable[index].key;
}

void init_jotable() {
  cell i = 0;
  while (i < jotable_size) {
    jotable[i] = proto_jotable_entry(i);
    i++;
  }
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

void jotable_test() {
  str2jo("testkey0");
  str2jo("testkey1");
  str2jo("testkey2");
  str2jo("testkey3");
  str2jo("testkey4");

  str2jo("testkey0");
  str2jo("testkey1");
  str2jo("testkey2");
  str2jo("testkey3");
  str2jo("testkey4");

  str2jo("testtestkey0");
  str2jo("testtestkey1");
  str2jo("testtestkey2");
  str2jo("testtestkey3");
  str2jo("testtestkey4");

  str2jo("testtesttestkey0");
  str2jo("testtesttestkey1");
  str2jo("testtesttestkey2");
  str2jo("testtesttestkey3");
  str2jo("testtesttestkey4");

  str2jo("testtesttesttestkey0");
  str2jo("testtesttesttestkey1");
  str2jo("testtesttesttestkey2");
  str2jo("testtesttesttestkey3");
  str2jo("testtesttesttestkey4");

  jotable_set_tag_and_value(str2jo("k1"), str2jo("<data>"), 1);
  jotable_report();

  jotable_set_tag_and_value(str2jo("k1"), str2jo("<data>"), 0);
  jotable_report();

  // jotable_print();
}

typedef cell argument_stack[1024 * 4];

argument_stack as;
cell as_base = 64;
cell as_pointer = 64;

void as_push(cell value) {
  as[as_pointer] = value;
  as_pointer++;
}

cell as_pop() {
  as_pointer--;
  return as[as_pointer];
}

cell as_tos() {
  return as[as_pointer - 1];
}

typedef struct {
  jo name;
  cell local_tag;
  cell local_data;
} local_point;

local_point local_area[1024 * 1024];
cell current_local_pointer = 0;

typedef struct {
  jo* array;
  cell local_pointer;
} return_point;

typedef return_point return_stack[1024 * 4];

return_stack rs;
cell rs_base = 64;
cell rs_pointer = 64;

void rs_push(return_point value) {
  rs[rs_pointer] = value;
  rs_pointer++;
}

return_point rs_pop() {
  rs_pointer--;
  return rs[rs_pointer];
}

return_point rs_tos() {
  return rs[rs_pointer - 1];
}

void rs_make_point(jo* array, cell local_pointer) {
  return_point rp = {.array = array, .local_pointer = local_pointer};
  rs[rs_pointer] = rp;
  rs_pointer++;
}

void rs_new_point(jo* array) {
  rs_make_point(array, current_local_pointer);
}

void rs_inc() {
  return_point rp = rs_pop();
  return_point rp1 = {.array = rp.array + 1, .local_pointer = rp.local_pointer};
  rs_push(rp1);
}

jo core_name_record[64 * 1024];
cell core_name_record_counter = 0;

void p_core_name_record() {
  as_push(core_name_record);
}

void p_core_name_report() {
  printf("- p_core_name_report // counter : %ld\n", core_name_record_counter);
  cell i = 0;
  while (i < core_name_record_counter) {
    printf("  %s\n", jo2str(core_name_record[i]));
    i++;
  }
  printf("\n");
}

jo name_record[64 * 1024];
cell name_record_counter = 0;

void p_name_record() {
  as_push(name_record);
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

typedef jo binding_filter;

typedef binding_filter binding_filter_stack_t[128];
binding_filter_stack_t binding_filter_stack;

cell binding_filter_stack_base = 0;
cell binding_filter_stack_pointer = 0;

void binding_filter_stack_push(binding_filter value) {
  binding_filter_stack[binding_filter_stack_pointer] = value;
  binding_filter_stack_pointer++;
}

binding_filter binding_filter_stack_pop() {
  binding_filter_stack_pointer--;
  return binding_filter_stack[binding_filter_stack_pointer];
}

binding_filter binding_filter_stack_tos() {
  return binding_filter_stack[binding_filter_stack_pointer - 1];
}

bool binding_filter_stack_empty_p() {
  return binding_filter_stack_pointer == binding_filter_stack_base;
}

void p_binding_filter_stack_push() {
  binding_filter_stack_push(as_pop());
}

void p_binding_filter_stack_pop() {
  as_push(binding_filter_stack_pop());
}

void run_binding_filter() {
  cell i = binding_filter_stack_pointer;
  while (i > binding_filter_stack_base) {
    jo_apply_now(binding_filter_stack[i-1]);
    i--;
  }
}

typedef jo binding_hook;

typedef binding_hook binding_hook_stack_t[128];
binding_hook_stack_t binding_hook_stack;

cell binding_hook_stack_base = 0;
cell binding_hook_stack_pointer = 0;

void binding_hook_stack_push(binding_hook value) {
  binding_hook_stack[binding_hook_stack_pointer] = value;
  binding_hook_stack_pointer++;
}

binding_hook binding_hook_stack_pop() {
  binding_hook_stack_pointer--;
  return binding_hook_stack[binding_hook_stack_pointer];
}

binding_hook binding_hook_stack_tos() {
  return binding_hook_stack[binding_hook_stack_pointer - 1];
}

bool binding_hook_stack_empty_p() {
  return binding_hook_stack_pointer == binding_hook_stack_base;
}

void p_binding_hook_stack_push() {
  binding_hook_stack_push(as_pop());
}

void p_binding_hook_stack_pop() {
  as_push(binding_hook_stack_pop());
}

void run_binding_hook(cell name, jo tag, cell value) {
  cell i = binding_hook_stack_pointer;
  while (i > binding_hook_stack_base) {
    as_push(value);
    as_push(tag);
    as_push(name);
    jo_apply_now(binding_hook_stack[i-1]);
    i--;
  }
}

bool used_jo_p(jo index) {
  return
    jotable[index].tag != str2jo("not-used");
}

bool declared_jo_p(jo index) {
  return
    jotable[index].tag == str2jo("declared");
}

void p_bind_name() {
  run_binding_filter();
  jo name = as_pop();
  jo tag = as_pop();
  cell value = as_pop();
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

  core_name_record[core_name_record_counter] = name;
  core_name_record_counter++;
  core_name_record[core_name_record_counter] = 0;

  name_record[name_record_counter] = name;
  name_record_counter++;
  name_record[name_record_counter] = 0;
}

void define_prim(string str, primitive fun) {
  jo name = str2jo(str);
  as_push(fun);
  as_push(str2jo("<prim>"));
  as_push(name);
  p_bind_name();
}

void define_primkey(string str, primitive fun) {
  jo name = str2jo(str);
  as_push(fun);
  as_push(str2jo("<prim-keyword>"));
  as_push(name);
  p_bind_name();
}

void export_bind() {
  define_prim("core-name-report", p_core_name_report);
  define_prim("core-name-record", p_core_name_record);

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

void jo_apply(jo jo) {
  if (!jotable_entry_used(jotable[jo])) {
    printf("undefined jo : %s\n", jo2str(jo));
    return;
  }
  cell tag = jotable[jo].tag;

  if (tag == str2jo("<prim>")) {
    primitive primitive = jotable_get_value(jo);
    primitive();
  }
  else if (tag == str2jo("<jojo>")) {
    cell jojo = jotable_get_value(jo);
    rs_new_point(jojo);
  }

  else if (tag == str2jo("<prim-keyword>")) {
    keyword_stack_push(alias_stack_pointer);
    primitive primitive = jotable_get_value(jo);
    primitive();
    alias_stack_pointer = keyword_stack_pop();
  }
  else if (tag == str2jo("<keyword>")) {
    // keywords are always evaled
    keyword_stack_push(alias_stack_pointer);
    cell jojo = jotable_get_value(jo);
    rs_new_point(jojo);
    eval();
    alias_stack_pointer = keyword_stack_pop();
  }

  else if (tag == str2jo("<data>")) {
    cell cell = jotable_get_value(jo);
    as_push(cell);
  }
  else {
    cell cell = jotable_get_value(jo);
    as_push(cell);
    as_push(tag);
  }
}

void jo_apply_now(jo jo) {
  cell tag = jotable[jo].tag;
  if (tag == str2jo("<jojo>")) {
    cell jojo = jotable_get_value(jo);
    rs_new_point(jojo);
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
  if (tag == str2jo("<jojo>")) {
    cell jojo = jotable_get_value(jo);
    rs_make_point(jojo, local_pointer);
    return;
  }
  else {
    jo_apply(jo);
    return;
  }
}

jmp_buf eval_jmp_buffer;

bool exit_eval() {
  longjmp(eval_jmp_buffer, 666);
}

void eval() {
  if (666 == setjmp(eval_jmp_buffer)) {
    return;
  }
  else {
    cell rs_base = rs_pointer;
    while (rs_pointer >= rs_base) {
      return_point rp = rs_tos();
      rs_inc();
      cell jo = *(cell*)rp.array;
      jo_apply(jo);
    }
  }
}

void p_apply() {
  rs_new_point(as_pop());
}

void p_apply_with_local_pointer() {
  jo* jojo = as_pop();
  cell local_pointer = as_pop();
  rs_make_point(jojo, local_pointer);
}

void p_replacing_apply_with_last_local_pointer() {
  jo jojo = as_pop();
  return_point rp = rs_pop();
  rs_make_point(jojo, rp.local_pointer);
}

void p_jo_apply() {
  jo_apply(as_pop());
}

void p_jo_apply_with_local_pointer() {
  jo jo = as_pop();
  cell local_pointer = as_pop();
  jo_apply_with_local_pointer(jo, local_pointer);
}

void p_jo_replacing_apply_with_last_local_pointer() {
  jo jo = as_pop();
  return_point rp = rs_pop();
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
  as_pop();
}

void p_2drop() {
  as_pop();
  as_pop();
}

void p_dup() {
  // (a a -> a)
  cell a = as_pop();
  as_push(a);
  as_push(a);
}

void p_2dup() {
  // (b a -> b a b a)
  cell a = as_pop();
  cell b = as_pop();
  as_push(b);
  as_push(a);
  as_push(b);
  as_push(a);
}

void p_over() {
  // (b a -> b a b)
  cell a = as_pop();
  cell b = as_pop();
  as_push(b);
  as_push(a);
  as_push(b);
}

void p_2over() {
  // (d c  b a -> d c  b a  d c)
  cell a = as_pop();
  cell b = as_pop();
  cell c = as_pop();
  cell d = as_pop();
  as_push(d);
  as_push(c);
  as_push(b);
  as_push(a);
  as_push(d);
  as_push(c);
}

void p_tuck() {
  // (b a -> a b a)
  cell a = as_pop();
  cell b = as_pop();
  as_push(a);
  as_push(b);
  as_push(a);
}

void p_2tuck() {
  // (d c  b a -> b a  d c  b a)
  cell a = as_pop();
  cell b = as_pop();
  cell c = as_pop();
  cell d = as_pop();
  as_push(b);
  as_push(a);
  as_push(d);
  as_push(c);
  as_push(b);
  as_push(a);
}

void p_swap() {
  // (b a -> a b)
  cell a = as_pop();
  cell b = as_pop();
  as_push(a);
  as_push(b);
}

void p_2swap() {
  // (d c  b a -> b a  d c)
  cell a = as_pop();
  cell b = as_pop();
  cell c = as_pop();
  cell d = as_pop();
  as_push(b);
  as_push(a);
  as_push(d);
  as_push(c);
}

void p_xy_swap() {
  // (xxx yyy x y -> yyy xxx)
  cell y = as_pop();
  cell x = as_pop();
  cell* yp = calloc(y, cell_size);
  cell* xp = calloc(x, cell_size);
  cell_copy(y, (as + (as_pointer - y)), yp);
  cell_copy(x, (as + (as_pointer - y - x)), xp);
  cell_copy(y, yp, (as + (as_pointer - y - x)));
  cell_copy(x, xp, (as + (as_pointer - x)));
  free(yp);
  free(xp);
}

void p_as_print() {
  // ([io] ->)
  printf("\n");
  if (as_pointer < as_base) {
    printf("  * %ld *  ", (as_pointer - as_base));
    printf("-- below the stack --\n");
  }
  else {
    printf("  * %ld *  ", (as_pointer - as_base));
    printf("-- ");
    cell i = as_base;
    while (i < as_pointer) {
      printf("%ld ", as[i]);
      i++;
    }
    printf("--\n");
  }
}

void p_stack_base() {
  as_push(as + as_base);
}

void p_stack_pointer() {
  as_push(as + as_pointer);
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
  define_prim("as/print", p_as_print);
  define_prim("stack-pointer", p_stack_pointer);
  define_prim("stack-base", p_stack_base);
}

void p_end() {
  // (rs: addr ->)
  return_point rp = rs_pop();
  current_local_pointer = rp.local_pointer;
}

void p_bye() {
  // (-> [exit])
  printf("bye bye ^-^/\n");
  exit(0);
}

void export_ending() {
  define_prim("end", p_end);
  define_prim("bye", p_bye);
}

void i_lit() {
  // ([rs] -> int)
  return_point rp = rs_tos();
  rs_inc();
  cell jo = *(cell*)rp.array;
  as_push(jo);
}

void i_jump_if_false() {
  // (bool [rs] -> [rs])
  return_point rp = rs_tos();
  rs_inc();
  jo* a = *(cell*)rp.array;
  cell b = as_pop();
  if (b == 0) {
    return_point rp1 = rs_pop();
    rs_make_point(a, rp1.local_pointer);
  }
}

void i_jump() {
  // ([rs] -> [rs])
  return_point rp = rs_tos();
  jo* a = *(cell*)rp.array;
  return_point rp1 = rs_pop();
  rs_make_point(a, rp1.local_pointer);
}

void export_control() {
  define_prim("ins/lit", i_lit);
  define_prim("ins/jump-if-false", i_jump_if_false);
  define_prim("ins/jump", i_jump);
}

void p_true() {
  as_push(1);
}

void p_false() {
  as_push(0);
}

void p_not() {
  // (bool -> bool)
  cell a = as_pop();
  as_push(!a);
}

void p_and() {
  // (bool bool -> bool)
  cell a = as_pop();
  cell b = as_pop();
  as_push(a&&b);
}

void p_or() {
  // (bool bool -> bool)
  cell a = as_pop();
  cell b = as_pop();
  as_push(a||b);
}

void export_bool() {
  define_prim("true", p_true);
  define_prim("false", p_false);
  define_prim("not", p_not);
  define_prim("and", p_and);
  define_prim("or", p_or);
}

void p_true_bit() {
  // (-> cell)
  cell i = -1;
  as_push(i);
}

void p_false_bit() {
  // (-> cell)
  as_push(0);
}

void p_bit_and() {
  // (cell cell -> cell)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a&b);
}

void p_bit_or() {
  // (cell cell -> cell)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a|b);
}

void p_bit_xor() {
  // (cell cell -> cell)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a^b);
}

void p_bit_not() {
  // (cell -> cell)
  cell a = as_pop();
  as_push(~a);
}

void p_bit_shift_left() {
  // (cell step -> cell)
  cell s = as_pop();
  cell a = as_pop();
  as_push(a<<s);
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
  cell a = as_pop();
  as_push(a + 1);
}

void p_dec() {
  cell a = as_pop();
  as_push(a - 1);
}

void p_neg() {
  cell a = as_pop();
  as_push(- a);
}

void p_add() {
  // (cell cell -> int)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a + b);
}

void p_sub() {
  // (cell cell -> int)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a - b);
}

void p_mul() {
  // (cell cell -> int)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a * b);
}

void p_div() {
  // (cell cell -> int)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a / b);
}

void p_mod() {
  // (cell cell -> int)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a % b);
}

void p_n_eq_p() {
  // (a ... b ... n -> bool)
  cell n = as_pop();
  cell old_n = n;
  cell* cursor1 = (as + as_pointer - n);
  cell* cursor2 = (as + as_pointer - n - n);
  while (n > 0) {
    if (cursor1[n-1] != cursor2[n-1]) {
      as_pointer = as_pointer - old_n - old_n;
      as_push(false);
      return;
    }
    n--;
  }
  as_pointer = as_pointer - old_n - old_n;
  as_push(true);
}

void p_eq_p() {
  // (cell cell -> bool)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a == b);
}

void p_gt_p() {
  // (cell cell -> bool)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a > b);
}

void p_lt_p() {
  // (cell cell -> bool)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a < b);
}

void p_gteq_p() {
  // (cell cell -> bool)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a >= b);
}

void p_lteq_p() {
  // (cell cell -> bool)
  cell b = as_pop();
  cell a = as_pop();
  as_push(a <= b);
}

jo read_jo();

void k_integer() {
  // ([io] -> [compile])
  while (true) {
    jo s = read_jo();
    if (s == str2jo(")")) {
      break;
    }
    else {
      here(str2jo("ins/lit"));
      here(string_to_dec(jo2str(s)));
    }
  }
}

void p_integer_print() { printf("%ld", as_pop()); }

void p_dot() { printf("%ld ", as_pop()); }
void p_integer_dot() { printf("%ld ", as_pop()); }

void export_integer() {
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

  define_primkey("integer", k_integer);

  define_prim("integer/print", p_integer_print);

  define_prim("dot", p_dot);
  define_prim("integer/dot", p_integer_dot);
}

void p_allocate () {
  // (size -> addr)
  as_push(calloc(as_pop(), 1));
}

void p_free () {
  // (addr ->)
  free(as_pop());
}

void k_address() {
  // ([io] -> [compile])
  here(str2jo("ins/lit"));
  jo index = read_jo();
  here(&(jotable[index].value));
  k_ignore();
}

void p_jo_as_var() {
  jo jo = as_pop();
  as_push(&(jotable[jo].value));
}

void p_set_cell() {
  // (cell address ->)
  cell* address = as_pop();
  cell value = as_pop();
  address[0] = value;
}

void p_get_cell() {
  // (address -> cell)
  cell* address = as_pop();
  as_push(address[0]);
}

void p_set_byte() {
  // (byte address ->)
  char* address = as_pop();
  cell value = as_pop();
  address[0] = value;
}

void p_get_byte() {
  // (address -> byte)
  char* address = as_pop();
  as_push(address[0]);
}

void export_memory() {
  define_prim("allocate", p_allocate);
  define_prim("free", p_free);
  define_primkey("&", k_address);
  define_prim("jo-as-var", p_jo_as_var);
  define_prim("set-cell", p_set_cell);
  define_prim("get-cell", p_get_cell);
  define_prim("set-byte", p_set_byte);
  define_prim("get-byte", p_get_byte);
}

typedef struct {
  FILE* file_handle;
  string file;
  string dir;
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

void real_reading_path(string path, char* buffer) {
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

byte read_byte() {
  if (reading_stack_empty_p()) {
    return fgetc(stdin);
  }
  else {
    char c = fgetc(reading_stack_tos().file_handle);
    if (c == EOF) {
      reading_point rp = reading_stack_pop();
      fclose(rp.file_handle);
      free(rp.file);
      free(rp.dir);
      return read_byte();
    }
    else {
      return c;
    }
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
  // (-> byte)
  as_push(read_byte());
}

void p_byte_unread() {
  // (byte -> [reading_stack])
  byte_unread(as_pop());
}

void p_byte_print() {
  // (byte ->)
  printf("%c", as_pop());
}

void export_byte() {
  define_prim("read/byte", p_read_byte);
  define_prim("byte/unread", p_byte_unread);
  define_prim("byte/print", p_byte_print);
}

void p_alias_push() {
  jo name = as_pop();
  jo nick = as_pop();
  alias a = {.nick = nick, .name = name};
  alias_stack_push(a);
}

void p_alias_filter() {
  jo nick = as_pop();
  cell base = keyword_stack_tos();
  cell i = alias_stack_pointer;
  while (i >= base) {
    if (alias_stack[i].nick == nick) {
      as_push(alias_stack[i].name);
      return;
    }
    else {
      i--;
    }
  }
  as_push(nick);
}

void p_read_raw_jo() {
  // ([io] -> jo)
  byte buf[1024];
  cell cur = 0;
  cell collecting = false;
  byte c;
  byte go = true;
  while (go) {
    c = read_byte();
    if (!collecting) {
      if (isspace(c)) {
        // do nothing
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
  as_push(str2jo(buf));
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
  jo_filter_stack_push(as_pop());
}

void p_jo_filter_stack_pop() {
  as_push(jo_filter_stack_pop());
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
  // ([io] -> jo)
  p_read_jo();
  return as_pop();
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
  jo jo2 = as_pop();
  jo jo1 = as_pop();
  as_push(cat_2_jo(jo1, jo2));
}

void p_empty_jo() {
  as_push(str2jo(""));
}

void p_jo_used_p() {
  // (jo -> bool)
  jo jo = as_pop();
  as_push(jotable_entry_used(jotable[jo]));
}

void p_jo_to_string() {
  // (jo -> string)
  jo jo = as_pop();
  as_push(jo2str(jo));
}

void p_string_length_to_jo() {
  // (string length -> jo)
  cell len = as_pop();
  cell str = as_pop();
  char buffer[2 * 1024];
  strncpy(buffer, str, len);
  buffer[len] = 0;
  as_push(str2jo(buffer));
}

void p_string_to_jo() {
  // (string -> jo)
  string str = as_pop();
  as_push(str2jo(str));
}

void p_null() {
  as_push(str2jo("null"));
}

void k_jo() {
  // ([io] -> [compile])
  while (true) {
    jo s = read_jo();
    if (s == str2jo("(")) {
      jo_apply(read_jo());
    }
    else if (s == str2jo(")")) {
      break;
    }
    else {
      here(str2jo("ins/lit"));
      here(s);
    }
  }
}

void p_jo_print() {
  // (jo -> [io])
  printf("%s", jo2str(as_pop()));
}

void p_jo_dot() {
  // (jo -> [io])
  printf("%s ", jo2str(as_pop()));
}

cell p_generate_jo_counter = 0;
void p_generate_jo() {
  string s = as_pop();
  char buffer [1024];
  sprintf(buffer, "%s:generated-jo#%ld", jo2str(s), p_generate_jo_counter);
  p_generate_jo_counter++;
  as_push(str2jo(buffer));
}

void export_jo() {
  define_prim("null", p_null);

  define_prim("jo-filter-stack-push", p_jo_filter_stack_push);
  define_prim("jo-filter-stack-pop", p_jo_filter_stack_pop);

  define_prim("alias-push", p_alias_push);
  define_prim("alias-filter", p_alias_filter);

  define_prim("read/jo", p_read_jo);
  define_prim("read/raw-jo", p_read_raw_jo);

  define_prim("jo/used?", p_jo_used_p);
  define_prim("jo/append", p_jo_append);
  define_prim("empty-jo", p_empty_jo);
  define_prim("jo->string", p_jo_to_string);
  define_prim("string->jo", p_string_to_jo);
  define_prim("string/length->jo", p_string_length_to_jo);
  define_primkey("jo", k_jo);
  define_prim("jo/print", p_jo_print);
  define_prim("jo/dot", p_jo_dot);
  define_prim("generate-jo", p_generate_jo);
}

void k_one_string() {
  // ([io] -> [compile])
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
  string str = malloc(cursor);
  strcpy(str, buffer);
  here(str2jo("ins/lit"));
  here(str);
}

void k_string() {
  // ([io] -> [compile])
  while (true) {
    jo s = read_jo();
    if (s == str2jo(")")) {
      return;
    }
    else if (s == str2jo("\"")) {
      k_one_string();
    }
    else {
      // do nothing
    }
  }
}

void p_string_length() {
  // (string -> length)
  as_push(strlen(as_pop()));
}

void p_string_print() {
  // (string -> [io])
  printf("%s", as_pop());
}

void p_string_dot() {
  // (string -> [io])
  printf("\"%s \"", as_pop());
}

void p_string_append_to_buffer() {
  // (buffer, string -> buffer)
  string str = as_pop();
  string buffer = as_tos();
  strcat(buffer, str);
}

void p_string_last_char() {
  string s = as_pop();
  cell i = 0;
  while (s[i+1] != 0) {
    i++;
  }
  as_push(s[i]);
}

void export_string() {
  define_primkey("string", k_string);
  define_primkey("one-string", k_one_string);
  define_prim("string/print", p_string_print);
  define_prim("string/dot", p_string_dot);
  define_prim("string/length", p_string_length);
  define_prim("string/append-to-buffer", p_string_append_to_buffer);
  define_prim("string/last-char", p_string_last_char);
}

void p_open_for_reading() {
  string pathname = as_pop();
  FILE* fd = open(pathname, O_RDONLY);
  if (fd == -1) {
    perror("- p_open_for_reading fail\n");
  }
  as_push(fd);
}

bool file_readable_p(string path) {
  FILE* fp = fopen(path, "r");
  if (!fp) {
    return false;
  }
  else {
    fclose(fp);
    return true;
  }
}

void p_file_readable_p() {
  // (file -> bool)
  as_push(file_readable_p(as_pop()));
}

bool dir_ok_p(string path) {
  DIR* dir = opendir(path);
  if (!dir) {
    return false;
  }
  else {
    closedir(dir);
    return true;
  }
}

void p_dir_ok_p() {
  // (dir -> bool)
  as_push(dir_ok_p(as_pop()));
}

cell file_size(string file_name) {
  struct stat st;
  stat(file_name, &st);
  return st.st_size;
}

void p_file_size() {
  as_push(file_size(as_pop()));
}

void p_file_copy_to_buffer() {
  // (file-name addr -> number)
  cell buffer = as_pop();
  cell path = as_pop();
  cell limit = file_size(path);
  FILE* fp = fopen(path, "r");
  if(!fp) {
    printf("- p_file_copy_to_buffer file to open file : %s\n", path);
    perror("  ");
    as_push(0);
    return;
  }
  cell read_counter = fread(buffer, 1, limit, fp);
  fclose(fp);
  as_push(read_counter);
}

void export_file() {
  define_prim("open-for-reading", p_open_for_reading);
  define_prim("file/readable?", p_file_readable_p);
  define_prim("dir/ok?", p_dir_ok_p);
  define_prim("file/size", p_file_size);
  define_prim("file/copy-to-buffer", p_file_copy_to_buffer);
}

void p_current_dir() {
  // (-> string)
  char buf[1024];
  as_push(getcwd(buf, 1024));
}

void p_command_run() {
  // (string -> *)
  system(as_pop());
}

void p_n_command_run() {
  // (..., string, n -> *)
  cell n = as_pop();
  cell i = 0;
  string str = malloc(4 * 1024);
  str[0] = 0;
  while (i < n) {
    strcat(str, as[as_pointer - n + i]);
    i++;
  }
  as_pointer = as_pointer - n;
  system(str);
  free(str);
}

cell argument_counter;

void p_argument_counter() {
  // (-> argument_counter)
  as_push(argument_counter);
}

string* argument_string_array;

void p_index_to_argument_string() {
  // (index -> string)
  cell index = as_pop();
  string argument_string = argument_string_array[index];
  as_push(argument_string);
}

void p_var_string_to_env_string() {
  // (string -> string)
  string var_string = as_pop();
  string env_string = getenv(var_string);
  as_push(env_string);
}

void export_system() {
  define_prim("current-dir", p_current_dir);
  define_prim("command/run", p_command_run);
  define_prim("n-command/run", p_n_command_run);
  define_prim("argument-counter", p_argument_counter);
  define_prim("index->argument-string", p_index_to_argument_string);
  define_prim("var-string->env-string", p_var_string_to_env_string);
}

void ccall (string str, void* lib) {
  primitive fun = dlsym(lib, str);
  if (fun == NULL) {
    printf("can not find %s function lib : %s\n",
           str, dlerror());
  };
  fun();
}

void* get_clib(string rel_path) {
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
  // ([io] -> [compile])
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
  // ([io] -> [compile])
  while (true) {
    jo s = read_jo();
    if (s == str2jo(")")) {
      return;
    }
    else if (s == str2jo("\"")) {
      k_clib_one();
    }
    else {
      // do nothing
    }
  }
}

void export_ffi() {
  define_prim("clib", k_clib);
}

void k_define() {
  jo name = read_jo();
  k_run();
  as_push(name);
  p_bind_name();
}

void k_declare_one() {
  jo index = read_jo();
  jotable[index].tag = str2jo("declared");
  k_ignore();
}

void k_declare() {
  while (true) {
    jo s = read_jo();
    if (s == str2jo(")")) {
      return;
    }
    else if (s == str2jo("(")) {
      k_declare_one();
    }
    else {
      // do nothing
    }
  }
}

void p_compile_jojo();

void k_run() {
  // ([io] -> *)
  jo* jojo = compiling_stack_tos();
  p_compile_jojo();
  rs_new_point(jojo);
  eval();
}

bool testing_flag = false;
void p_testing_flag() { as_push(testing_flag); }
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

void p_as_print_by_flag() {
  if (top_repl_printing_flag) {
    p_as_print();
  }
  else {
    // do nothing
  }
}

void p_top_repl() {
  // ([io] -> *)
  while (true) {
    jo s = read_jo();
    if (s == str2jo("(")) {
      jo_apply(read_jo());
      p_as_print_by_flag();
    }
    else {
      // do nothing
    }
  }
}

void p_top_repl_printing_flag() { as_push(top_repl_printing_flag); }
void p_top_repl_printing_flag_on() { top_repl_printing_flag = true; }
void p_top_repl_printing_flag_off() { top_repl_printing_flag = false; }

void export_top_level() {
  define_primkey("define", k_define);
  define_primkey("bind-name", p_bind_name);
  define_primkey("declare", k_declare);

  define_primkey("run", k_run);

  define_primkey("test", k_test);
  define_prim("testing-flag", p_testing_flag);
  define_prim("testing-flag/on", p_testing_flag_on);
  define_prim("testing-flag/off", p_testing_flag_off);


  define_prim("as/print-by-flag", p_as_print_by_flag);
  define_prim("top-repl", p_top_repl);
  define_prim("top-repl/printing-flag", p_top_repl_printing_flag);
  define_prim("top-repl/printing-flag/on", p_top_repl_printing_flag_on);
  define_prim("top-repl/printing-flag/off", p_top_repl_printing_flag_off);
}

void k_ignore() {
  // ([io] ->)
  while (true) {
    jo s = read_jo();
    if (s == str2jo("(")) {
      k_ignore();
    }
    if (s == str2jo(")")) {
      break;
    }
  }
}

void compile_until_meet_jo(jo ending_jo) {
  // ([io] -> [compile])
  while (true) {
    jo s = read_jo();
    if (s == str2jo("(")) {
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
      return;
    }
  }
}

void p_compile_until_meet_jo() {
  // (jo -> [compile])
  compile_until_meet_jo(as_pop());
}

jo compile_until_meet_jo_or_jo(jo ending_jo1, jo ending_jo2) {
  while (true) {
    jo s = read_jo();
    if (s == str2jo("(")) {
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
      return;
    }
  }
}

void p_compile_until_round_ket() {
  // ([io] -> [compile])
  compile_until_meet_jo(str2jo(")"));
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
  // ([io] -> [compile])
  compile_until_meet_jo(str2jo("then"));
  here(str2jo("ins/jump-if-false"));
  cell* end_of_then = compiling_stack_tos();
  compiling_stack_inc();
  jo ending_jo = compile_until_meet_jo_or_jo(str2jo("else"), str2jo(")"));
  if (ending_jo == str2jo(")")) {
    end_of_then[0] = compiling_stack_tos();
    return;
  }
  else {
    here(str2jo("ins/jump"));
    cell* end_of_else = compiling_stack_tos();
    compiling_stack_inc();
    end_of_then[0] = compiling_stack_tos();
    p_compile_until_round_ket();
    end_of_else[0] = compiling_stack_tos();
    return;
  }
}

void k_tail_call() {
  // ([io] -> [compile])
  // no check for "no compile before define"
  here(str2jo("ins/lit"));
  here(read_jo());
  here(str2jo("jo/replacing-apply-with-last-local-pointer"));
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
  // ([io] -> [compile])
  jo* jojo = compiling_stack_tos();
  current_compiling_jojo_stack_push(jojo);
  compile_until_meet_jo(str2jo(")"));
  here(str2jo("end"));
  current_compiling_jojo_stack_pop();
}

void k_loop() {
  here(str2jo("ins/lit"));
  here(current_compiling_jojo_stack_tos());
  here(str2jo("replacing-apply-with-last-local-pointer"));
  k_ignore();
}

void k_recur() {
  here(str2jo("ins/lit"));
  here(current_compiling_jojo_stack_tos());
  here(str2jo("apply"));
  k_ignore();
}

void p_compiling_stack_tos() {
  as_push(compiling_stack_tos());
}

void k_bare_jojo() {
  // ([io] -> [compile])
  here(str2jo("ins/jump"));
  cell* offset_place = compiling_stack_tos();
  compiling_stack_inc();
  p_compile_jojo();
  offset_place[0] = compiling_stack_tos();
  here(str2jo("ins/lit"));
  here(offset_place + 1);
}

void k_jojo() {
  // ([io] -> [compile])
  here(str2jo("ins/jump"));
  cell* offset_place = compiling_stack_tos();
  compiling_stack_inc();
  p_compile_jojo();
  offset_place[0] = compiling_stack_tos();
  here(str2jo("ins/lit"));
  here(offset_place + 1);
  here(str2jo("ins/lit"));
  here(str2jo("<jojo>"));
}

void k_keyword() {
  // ([io] -> [compile])
  here(str2jo("ins/jump"));
  cell* offset_place = compiling_stack_tos();
  compiling_stack_inc();
  p_compile_jojo();
  offset_place[0] = compiling_stack_tos();
  here(str2jo("ins/lit"));
  here(offset_place + 1);
  here(str2jo("ins/lit"));
  here(str2jo("<keyword>"));
}

void k_data() {
  // ([io] -> [compile])
  p_compile_until_round_ket();
  here(str2jo("ins/lit"));
  here(str2jo("<data>"));
}

cell local_find(jo name) {
  // return index of local_area
  // -1 -- no found
  return_point rp = rs_tos();
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
  cell jo = as_pop();
  cell index = local_find(jo);
  cell data = as_pop();
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
  cell jo = as_pop();
  cell index = local_find(jo);
  if (index != -1) {
    local_point lp = local_area[index];
    as_push(lp.local_data);
  }
  else {
    printf("- p_local_data_out fatal error\n");
    printf("  name is not bound\n");
    printf("  name : %s\n", jo2str(jo));
  }
}

void p_local_tag_in() {
  cell jo = as_pop();
  cell index = local_find(jo);
  cell tag = as_pop();
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
  cell jo = as_pop();
  cell index = local_find(jo);
  if (index != -1) {
    local_point lp = local_area[index];
    as_push(lp.local_tag);
  }
  else {
    printf("- p_local_tag_out fatal error\n");
    printf("  name is not bound\n");
    printf("  name : %s\n", jo2str(jo));
  }
}

void p_local_in() {
  cell jo = as_pop();
  cell index = local_find(jo);
  cell tag = as_pop();
  cell data = as_pop();
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
  cell jo = as_pop();
  cell index = local_find(jo);
  if (index != -1) {
    local_point lp = local_area[index];
    as_push(lp.local_data);
    as_push(lp.local_tag);
  }
  else {
    printf("- p_local_out fatal error\n");
    printf("  name is not bound\n");
    printf("  name : %s\n", jo2str(jo));
  }
}

void k_local_data_in() {
  jo s = read_jo();
  if (s == str2jo("(")) {
    jo_apply(read_jo());
    k_local_data_in();
  }
  else if (s == str2jo(")")) {
    return;
  }
  else {
    k_local_data_in();
    here(str2jo("ins/lit"));
    here(s);
    here(str2jo("local-data-in"));
  }
}

void k_local_data_out() {
  jo s = read_jo();
  if (s == str2jo("(")) {
    jo_apply(read_jo());
    k_local_data_out();
  }
  else if (s == str2jo(")")) {
    return;
  }
  else {
    k_local_data_out();
    here(str2jo("ins/lit"));
    here(s);
    here(str2jo("local-data-out"));
  }
}

void k_local_tag_in() {
  jo s = read_jo();
  if (s == str2jo("(")) {
    jo_apply(read_jo());
    k_local_data_in();
  }
  else if (s == str2jo(")")) {
    return;
  }
  else {
    k_local_data_in();
    here(str2jo("ins/lit"));
    here(s);
    here(str2jo("local-tag-in"));
  }
}

void k_local_tag_out() {
  jo s = read_jo();
  if (s == str2jo("(")) {
    jo_apply(read_jo());
    k_local_data_out();
  }
  else if (s == str2jo(")")) {
    return;
  }
  else {
    k_local_data_out();
    here(str2jo("ins/lit"));
    here(s);
    here(str2jo("local-tag-out"));
  }
}

void k_local_in() {
  jo s = read_jo();
  if (s == str2jo("(")) {
    jo_apply(read_jo());
    k_local_data_in();
  }
  else if (s == str2jo(")")) {
    return;
  }
  else {
    k_local_data_in();
    here(str2jo("ins/lit"));
    here(s);
    here(str2jo("local-in"));
  }
}

void k_local_out() {
  jo s = read_jo();
  if (s == str2jo("(")) {
    jo_apply(read_jo());
    k_local_data_out();
  }
  else if (s == str2jo(")")) {
    return;
  }
  else {
    k_local_data_out();
    here(str2jo("ins/lit"));
    here(s);
    here(str2jo("local-out"));
  }
}

void p_current_local_pointer() {
  as_push(current_local_pointer);
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
  here(as_pop());
}

void p_address_of_here() {
 as_push(compiling_stack_tos());
}

void p_round_bar() { as_push(str2jo("(")); }
void p_round_ket() { as_push(str2jo(")")); }
void p_square_bar() { as_push(str2jo("[")); }
void p_square_ket() { as_push(str2jo("]")); }
void p_flower_bar() { as_push(str2jo("{")); }
void p_flower_ket() { as_push(str2jo("}")); }
void p_double_quote() { as_push(str2jo("\"")); }

void p_cell_size() {
  // (-> cell)
  as_push(cell_size);
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

void p1() {
  printf("- p1\n");
  printf("  %ld %ld %ld\n", sizeof(void*), cell_size, sizeof(unsigned));
  printf("  %ld %ld\n", sizeof((cell)-1), sizeof(-1));
  printf("  %x %x\n", 1<<cell_size, 32>>6);
  printf("  %x %x %x\n", -1>>2, (cell)-1>>2, (unsigned)-1>>2);
  printf("  %ld %ld\n", string_to_bin("1000"), string_to_hex("ffff"));
  printf("  %ld %ld %ld %ld\n", '0', '1', 'A', 'a');

  printf("  %ld\n", EOF);
  printf("  %ld\n", PATH_MAX);

  struct stat st;
  stat("READM", &st);
  printf("  file-size of README : %ld\n", st.st_size);
  printf("  sizeof &st : %ld\n", sizeof(&st));
  printf("  sizeof st : %ld\n", sizeof(st));
}

void p2() {
  printf("- p2\n");
  printf("  sizeof local_point : %ld\n", sizeof(local_point));
  printf("  sizeof local_area : %ld\n", sizeof(local_area));
}

cell string_to_sum_test(string str) {
  cell sum = 0;
  cell max_step = 10;
  cell i = 0;
  while (i < strlen(str)) {
    sum = sum + ((unsigned char) str[i]) * (2 << min(i, max_step));
    printf("| char: %ld | unsigned char: %ld | sum: %ld |\n", str[i], (unsigned char) str[i], sum);
    i++;
  }
  return sum;
}

void p3() {
  printf("- p3\n");
  printf("  %ld\n", string_to_sum_test("abcabcabc"));
  printf("  %ld\n", string_to_sum_test("中中"));
  printf("  %ld\n", string_to_sum_test("中中中"));
}

void export_play() {
  define_prim("p1", p1);
  define_prim("p2", p2);
  define_prim("p3", p3);
}

void load_file(string path) {
  // [reading_stack]
  FILE* fp = fopen(path, "r");
  if(!fp) {
    perror("File opening failed");
    printf("load_file fail : %s\n", path);
    return;
  }
  char* file_buffer = malloc(PATH_MAX);
  char* dir_buffer = malloc(PATH_MAX);
  realpath(path, file_buffer);
  realpath(path, dir_buffer);
  char* dir_addr = dirname(dir_buffer);
  reading_point rp = {
    .file_handle = fp,
    .file = file_buffer,
    .dir = dir_addr
  };
  // { printf("- load_file\n");
  //   printf("  fp: %d\n", fp);
  //   printf("  file: %s\n", file_buffer);
  //   printf("  dir_buffer: %s #%ld\n", dir_buffer, dir_buffer);
  //   printf("  dir_addr: %s #%ld\n", dir_addr, dir_addr);
  // }
  reading_stack_push(rp);
}

void init_top_repl() {
  init_jotable();
  init_compiling_stack();
  init_jo_filter_stack();

  p_empty_jo();
  p_drop();

  export_bind();
  export_apply();
  export_stack_operation();
  export_ending();
  export_control();
  export_bool();
  export_bit();
  export_integer();
  export_memory();
  export_byte();
  export_jo();
  export_string();
  export_file();
  export_keyword();
  export_system();
  // export_module();
  // export_ffi();
  export_top_level();
  export_mise();
  export_play();
}

int main(int argc, string* argv) {

  argument_counter = argc;
  argument_string_array = argv;

  init_top_repl();

  if (argc != 1) {
    if (file_readable_p(argv[1])) {
      load_file(argv[1]);
    }
    else {
      printf("- jojo can not load file: %s\n", argv[1]);
      printf("  it is not readable\n");
      return 69;
    }
  }

  p_top_repl();
}
