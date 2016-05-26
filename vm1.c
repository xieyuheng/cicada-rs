#include <stdio.h>
#include <stdint.h>
#include <setjmp.h>
#include <string.h>

typedef enum { false, true } bool;

int
max
(int a, int b) {
  if (a < b) {
    return b;
  }
  else {
    return a;
  }
}

int
min
(int a, int b) {
  if (a > b) {
    return b;
  }
  else {
    return a;
  }
}

typedef char* string;

bool
string_equal
(string s1, string s2) {
  if (strcmp(s1, s2) == 0) {
    return true;
  }
  else {
    return false;
  }
}

typedef uint8_t byte;
typedef uint32_t cell;
typedef cell addr;

#define cell_SIZE 4;
cell cell_size = 4;

cell memory_size = 1024 * 1024;
byte memory[1024 * 1024];
addr memory_cursor = 0;


byte
memory_get_byte
(cell index) {
  return (byte) memory[index];
}

void
memory_set_byte
(cell index, byte value) {
  memory[index] = value;
}

cell
memory_get
(cell index) {
    cell sum = 0;
    cell i;
    cell base = 1;
    for (i=0; i < cell_size; i=i+1) {
      sum = sum + (memory_get_byte(index + i) * base);
      base = base * 256;
    }
    return sum;
}

void
memory_set
(cell index, cell value) {
  cell i;
  for (i=0; i < cell_size; i=i+1) {
    memory_set_byte(index + i, value % 256);
    value = value >> 8;
  }
}

cell
memory_allocate
(cell size) {
  cell return_address = memory_cursor;
  memory_cursor = return_address + size;
  return return_address;
}

typedef cell stack[1024 * 4];


stack as;
cell as_pointer = 0;

void
as_push
(cell value) {
  as[as_pointer] = value;
  as_pointer = as_pointer + cell_size;
}

cell
as_pop
() {
  as_pointer = as_pointer - cell_size;
  return as[as_pointer];
}


stack rs;
cell rs_pointer = 0;

void
rs_push
(cell value) {
  rs[rs_pointer] = value;
  rs_pointer = rs_pointer + cell_size;
}

cell
rs_pop
() {
  rs_pointer = rs_pointer - cell_size;
  return rs[rs_pointer];
}

typedef struct {
  cell index;
  string key;
  cell value;
  cell orbit_length;
  cell orbiton;
} nametable_entry;

nametable_entry
new_nametable_entry
(cell index) {
  nametable_entry e = {
    .index = index,
    .key = 0,
    .value = 0,
    .orbit_length = 0,
    .orbiton = 0
  };
  return e;
}

bool
nametable_entry_occured
(nametable_entry e) {
  return e.key != 0;
}

bool
nametable_entry_used
(nametable_entry e) {
  return e.value != 0;
}

bool
nametable_entry_no_collision
(nametable_entry e) {
  return e.index == e.orbiton;
}

// prime table size
#define nametable_size 997
nametable_entry nametable[nametable_size];
cell nametable_counter = 0;

void
init_nametable
() {
  cell i = 0;
  while (i < nametable_size) {
    nametable[i] = new_nametable_entry(i);
    i = i + 1;
  }
}

cell
string_to_sum
(string str) {
  cell sum = 0;
  cell max_step = 10;
  cell i = 0;
  while (i < strlen(str)) {
    sum = sum + str[i] * (2 << min(i, max_step));
    i = 1 + i;
  }
  return sum;
}

bool
nametable_keyeq
(string k1, string k2) {
  return string_equal(k1, k2);
}

cell
nametable_hash
(string key, cell counter) {
  return (counter + string_to_sum(key)) % nametable_size;
}

int // -1 denotes the hash_table is filled
nametable_insert
(string key) {
  cell orbit_index = nametable_hash(key, 0);
  cell counter = 0;
  while (true) {
    cell index = nametable_hash(key, counter);
    if (!nametable_entry_occured(nametable[index])) {
      nametable[index].key = key;
      nametable[index].orbiton = orbit_index;
      nametable[orbit_index].orbit_length = 1 + counter;
      nametable_counter = 1 + nametable_counter;
      return index;
    }
    else if (nametable_keyeq(key, nametable[index].key)) {
      return index;
    }
    else if (counter == nametable_size) {
      return -1;
    }
    else {
      counter = 1 + counter;
    }
  }
}

int // -1 denotes key not occured
nametable_search
(string key) {
  cell counter = 0;
  while (true) {
    cell index = nametable_hash(key, counter);
    if (!nametable_entry_occured(nametable[index])) {
      return -1;
    }
    else if (nametable_keyeq(key, nametable[index].key)) {
      return index;
    }
    else if (counter == nametable_size) {
      return -1;
    }
    else {
      counter = 1 + counter;
    }
  }
}

void
nametable_report_orbit
(cell index, cell counter) {
  while (counter < nametable[index].orbit_length) {
    string key = nametable[index].key;
    cell next_index = nametable_hash(key, counter);
    if (index == nametable[next_index].orbiton) {
      printf("  - %d %s\n", next_index, nametable[next_index].key);
    }
    counter = 1 + counter;
  }
}

void
nametable_report
() {
  printf("\n");
  printf("- nametable_report\n");
  cell index = 0;
  while (index < nametable_size) {
    if (nametable_entry_occured(nametable[index]) &&
        nametable_entry_no_collision(nametable[index])) {
      printf("  - %d %s // %d\n",
             index, nametable[index].key, nametable[index].orbit_length);
      if (nametable_entry_used(nametable[index])) {
        printf("      %d\n", nametable[index].value);
      }
      nametable_report_orbit(index, 1);
    }
    index = 1 + index;
  }
  printf("\n");
  printf("- used : %d\n", nametable_counter);
  printf("- free : %d\n", nametable_size - nametable_counter);
}

void
nametable_print
() {
  printf("\n");
  printf("- nametable_print\n");
  cell index = 0;
  while (index < nametable_size) {
    printf("  - %d %d %d // %d\n",
           index,
           nametable[index].key,
           nametable[index].value,
           nametable[index].orbit_length);
    index = 1 + index;
  }
  printf("\n");
  printf("- used : %d\n", nametable_counter);
  printf("- free : %d\n", nametable_size - nametable_counter);
}

void
nametable_set
(cell index, cell value) {
  nametable[index].value = value;
}

cell
nametable_get
(cell index) {
  return nametable[index].value;
}

cell
name
(string str) {
  return nametable_insert(str);
}

string
name_to_key
(cell index) {
  return nametable[index].key;
}

void
nametable_test
() {
  printf("%d\n", name("testkey0"));
  printf("%d\n", name("testkey1"));
  printf("%d\n", name("testkey2"));
  printf("%d\n", name("testkey3"));
  printf("%d\n", name("testkey4"));

  printf("%d\n", name("testkey0"));
  printf("%d\n", name("testkey1"));
  printf("%d\n", name("testkey2"));
  printf("%d\n", name("testkey3"));
  printf("%d\n", name("testkey4"));

  printf("%d\n", name("testtestkey0"));
  printf("%d\n", name("testtestkey1"));
  printf("%d\n", name("testtestkey2"));
  printf("%d\n", name("testtestkey3"));
  printf("%d\n", name("testtestkey4"));

  printf("%d\n", name("testtesttestkey0"));
  printf("%d\n", name("testtesttestkey1"));
  printf("%d\n", name("testtesttestkey2"));
  printf("%d\n", name("testtesttestkey3"));
  printf("%d\n", name("testtesttestkey4"));

  printf("%d\n", name("testtesttesttestkey0"));
  printf("%d\n", name("testtesttesttestkey1"));
  printf("%d\n", name("testtesttesttestkey2"));
  printf("%d\n", name("testtesttesttestkey3"));
  printf("%d\n", name("testtesttesttestkey4"));

  printf("%d\n", nametable_get(name("k1")));
  nametable_set(name("k1"), 1);
  printf("%d\n", nametable_get(name("k1")));
  nametable_set(name("k1"), 0);
  printf("%d\n", nametable_get(name("k1")));
  nametable_set(name("k1"), 1);

  nametable_report();
  // nametable_print();
}

typedef void (*primitive)();

typedef primitive primitive_record_t[1024];

primitive_record_t primitive_record;
cell primitive_record_counter = 0;

primitive
primitive_record_get
(cell index) {
  return primitive_record[index];
}

void
primitive_record_set
(cell index, primitive fun) {
  primitive_record[index] = fun;
}

cell
create_primitive
(primitive fun) {
  cell return_address = primitive_record_counter;
  primitive_record_set
    (primitive_record_counter, fun);
  primitive_record_counter =
    primitive_record_counter + 1;
  return return_address;
}

cell address_after_explainer = 0;

jmp_buf jmp_buffer;

bool
exit_interpreter
() {
  longjmp(jmp_buffer, 666);
}

void
interpreter
() {
  if (666 == setjmp(jmp_buffer)) {
    return;
  }
  else {
    while (true) {
      addr function_body = rs_pop();
      addr explainer = memory_get(memory_get(function_body));
      rs_push(function_body + cell_size);
      address_after_explainer =
        memory_get(function_body) + cell_size;
      primitive explainer_function =
        primitive_record_get(explainer);
      explainer_function();
    }
  }
}

typedef struct {
  string string;
  addr address;
} tag_record_entry;

typedef tag_record_entry tag_record_t[1024];

tag_record_t tag_record;
cell tag_record_counter = 0;

addr
tag_record_get
(string string) {
  cell i;
  bool match_p;
  for (i=0; i < tag_record_counter; i=i+1) {
    match_p = (string_equal
               (string,
                (tag_record[i].string)));
    if (match_p) {
      return (tag_record[i].address);
    }
  }
}

byte tag_record_string_buffer[1024 * 1024];
cell tag_record_string_buffer_counter = 0;

void
tag_record_set
(string string, addr address) {
  strcpy((tag_record_string_buffer +
          tag_record_string_buffer_counter),
         string);
  tag_record
    [tag_record_counter]
    .string = (tag_record_string_buffer +
               tag_record_string_buffer_counter);
  tag_record_string_buffer_counter =
    tag_record_string_buffer_counter +
    strlen(string) + 1;
  tag_record
    [tag_record_counter]
    .address = address;
  tag_record_counter =
    tag_record_counter + 1;
}

void
data
(cell value) {
  memory_set(memory_cursor, value);
  memory_cursor =
    memory_cursor + cell_size;
}

void
mark
(string tag_string) {
  tag_record_set
    (tag_string,
     memory_cursor);
}

cell link = 0;

void
p_primitive_explainer
() {
  primitive primitive =
    (primitive_record_get
     (memory_get
      (address_after_explainer)));
  primitive();
}

cell primitive_explainer = 0;

void
define_primitive
(string tag_string, primitive fun) {
  cell function_index = create_primitive(fun);
  data(link);
  link = memory_cursor - cell_size;
  mark(tag_string);
  data(primitive_explainer);
  data(function_index);
}

void
p_function_explainer
() {
  rs_push(address_after_explainer);
}

cell function_explainer = 1;

void
define_function
(string tag_string,
 cell length,
 string *function_tag_string_array) {
  data(link);
  link = memory_cursor - cell_size;
  mark(tag_string);
  data(function_explainer);
  cell i;
  for (i=0; i < length; i=i+1) {
    data(tag_record_get
         (function_tag_string_array[i]));
  }
}

void
p_variable_explainer
() {
  as_push
    (memory_get(address_after_explainer));
}

cell variable_explainer = 2;

void
define_variable
(string tag_string, cell value) {
  data(link);
  link = memory_cursor - cell_size;
  mark(tag_string);
  data(variable_explainer);
  data(value);
}

void
p_end
() {
  // (rs: addr ->)
  rs_pop();
}

void
p_bye
() {
  // (-> [exit])
  printf("bye bye ^-^/\n");
  exit_interpreter();
}

void
p_dup
() {
  // (cell cell -> cell)
  cell a = as_pop();
  as_push(a);
  as_push(a);
}

void
p_mul
() {
  // (integer integer -> integer)
  cell a = as_pop();
  cell b = as_pop();
  as_push(a * b);
}

void
p_simple_wirte
() {
  // (integer -> [IO])
  printf("%d\n", as_pop());
}

void
vm1
() {

  init_nametable();

  create_primitive(p_primitive_explainer);
  create_primitive(p_function_explainer);
  create_primitive(p_variable_explainer);

  define_primitive("end", p_end);
  define_primitive("bye", p_bye);
  define_primitive("dup", p_dup);
  define_primitive("mul", p_mul);
  define_primitive("simple-wirte", p_simple_wirte);

  define_variable("little-test-number", 4);

  string p_square[] = {
    "dup",
    "mul",
    "end"
  };
  define_function("square", 3, p_square);

  string p_little_test[] = {
    "little-test-number",
    "square",
    "simple-wirte",
    "bye"
  };
  define_function ("little-test", 4, p_little_test);

  string p_first_function[] = {
    "little-test",
    "end"
  };
  define_function("first-function", 2, p_first_function);

  rs_push
    (tag_record_get("first-function") + cell_size);
  interpreter();

}

int
main
(int argc, string* argv) {
  vm1();
  nametable_test();
  return 0;
}
