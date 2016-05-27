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

typedef void (*primitive)();

typedef cell name;

typedef struct {
  cell size;
  name *array;
} jojo;

typedef union {
  cell cell;
  primitive primitive;
  jojo jojo;
} bind;

typedef struct {
  cell index;
  string key;
  name type;
  bind value;
  cell orbit_length;
  cell orbiton;
} nametable_entry;

name k2n (string str);

nametable_entry
new_nametable_entry
(cell index) {
  nametable_entry e = {
    .index = index,
    .key = 0,
    .type = k2n("none"),
    .value.cell = 0,
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
  return e.type != k2n("none");
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

string n2k (cell index);

void nametable_entry_print
(nametable_entry entry) {
  printf("%s : ", n2k(entry.type));
  if (entry.type == k2n("cell")) {
    printf("%d", entry.value.cell);
  }
  else if (entry.type == k2n("primitive")) {
    printf("%d", entry.value.primitive);
  }
  else if (entry.type == k2n("jojo")) {
    printf("%d ", entry.value.jojo.size);
    printf("[ ", entry.value.jojo.size);
    cell i;
    for (i=0; i < entry.value.jojo.size; i=i+1) {
      printf("%d ", entry.value.jojo.array[i]);
    }
    printf("]", entry.value.jojo.size);
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
    if (nametable_entry_used(nametable[next_index])) {
      printf("    = ");
      nametable_entry_print(nametable[next_index]);
      printf("\n");
    }
    counter = 1 + counter;
  }
}

void
nametable_report
() {
  printf("\n");
  printf("- nametable_report\n");
  printf("  : <index> <key> // <orbit-length>\n");
  cell index = 0;
  while (index < nametable_size) {
    if (nametable_entry_occured(nametable[index]) &&
        nametable_entry_no_collision(nametable[index])) {
      printf("  - %d %s // %d\n",
             index, nametable[index].key, nametable[index].orbit_length);
      if (nametable_entry_used(nametable[index])) {
        printf("    = ");
        nametable_entry_print(nametable[index]);
        printf("\n");
      }
      nametable_report_orbit(index, 1);
    }
    index = 1 + index;
  }
  printf("  : <index> <key> // <orbit-length>\n");
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
    printf("  - %d %s %d // %d\n",
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

name
k2n
(string str) {
  return nametable_insert(str);
}

string
n2k
(cell index) {
  return nametable[index].key;
}

void
init_nametable
() {
  cell i = 0;
  while (i < nametable_size) {
    nametable[i] = new_nametable_entry(i);
    i = i + 1;
  }
}

name jojo_area[1024 * 1024];
cell jojo_area_counter = 0;

void
nametable_set_cell
(cell index, cell cell) {
  nametable[index].type = k2n("cell");
  nametable[index].value.cell = cell;
}

void
nametable_set_primitive
(cell index, primitive primitive) {
  nametable[index].type = k2n("primitive");
  nametable[index].value.primitive = primitive;
}

cell
nametable_get_cell
(cell index) {
  return nametable[index].value.cell;
}

primitive
nametable_get_primitive
(cell index) {
  return nametable[index].value.primitive;
}

jojo
nametable_get_jojo
(cell index) {
  return nametable[index].value.jojo;
}

void
nametable_test
() {
  k2n("testkey0");
  k2n("testkey1");
  k2n("testkey2");
  k2n("testkey3");
  k2n("testkey4");

  k2n("testkey0");
  k2n("testkey1");
  k2n("testkey2");
  k2n("testkey3");
  k2n("testkey4");

  k2n("testtestkey0");
  k2n("testtestkey1");
  k2n("testtestkey2");
  k2n("testtestkey3");
  k2n("testtestkey4");

  k2n("testtesttestkey0");
  k2n("testtesttestkey1");
  k2n("testtesttestkey2");
  k2n("testtesttestkey3");
  k2n("testtesttestkey4");

  k2n("testtesttesttestkey0");
  k2n("testtesttesttestkey1");
  k2n("testtesttesttestkey2");
  k2n("testtesttesttestkey3");
  k2n("testtesttesttestkey4");

  nametable_set_cell(k2n("k1"), 1);
  nametable_report();

  nametable_set_cell(k2n("k1"), 0);
  nametable_report();

  // nametable_print();
}

typedef cell argument_stack[1024 * 4];

argument_stack as;
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

typedef name* return_stack[1024 * 4];

return_stack rs;
cell rs_pointer = 0;

void
rs_push
(name* value) {
  rs[rs_pointer] = value;
  rs_pointer = rs_pointer + cell_size;
}

name*
rs_pop
() {
  rs_pointer = rs_pointer - cell_size;
  return rs[rs_pointer];
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
      name* function_body = rs_pop();
      addr name = *(cell*)function_body;
      rs_push(function_body + 1);
      cell type_name = nametable[name].type;
      if (type_name == k2n("primitive")) {
        primitive primitive = nametable_get_primitive(name);
        primitive();
      }
      else if (type_name == k2n("jojo")) {
        jojo jojo = nametable_get_jojo(name);
        rs_push(jojo.array);
      }
      else if (type_name == k2n("cell")) {
        cell cell = nametable_get_cell(name);
        as_push(cell);
      }
    }
  }
}

void
define_primitive
(string str, primitive fun) {
  name index = k2n(str);
  nametable_set_primitive(index, fun);
}

void
define_function
(string str, cell size, string *str_array) {
  name index = k2n(str);
  cell i;
  cell *array;
  array = (jojo_area + jojo_area_counter);
  jojo_area_counter = size + jojo_area_counter;
  for (i=0; i < size; i=i+1) {
    array[i] = k2n(str_array[i]);
  }
  nametable[index].type = k2n("jojo");
  nametable[index].value.jojo.size = size;
  nametable[index].value.jojo.array = array;
}

void
define_variable
(string str, cell cell) {
  name index = k2n(str);
  nametable_set_cell(index, cell);
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

  jojo first_jojo = nametable_get_jojo(k2n("first-function"));
  rs_push(first_jojo.array);

  // nametable_report();
  interpreter();

}

int
main
(int argc, string* argv) {
  vm1();
  // nametable_test();
  return 0;
}
