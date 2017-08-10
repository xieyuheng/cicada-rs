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
  #include <stdarg.h>
  #include <sys/socket.h>
  #include <netdb.h>
  #include <arpa/inet.h>
  typedef enum { false, true } bool;
  // typedef intptr_t cell;
  typedef intmax_t cell;
  typedef void (* primitive_t)();
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
    void report(char* format, ...) {
      va_list arg_list;
      va_start(arg_list, format);
      vdprintf(STDERR_FILENO, format, arg_list);
      va_end(arg_list);
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
      cell string_count_member(char* s, char b) {
        cell sum = 0;
        cell i = 0;
        while (s[i] != '\0') {
          if (s[i] == b) {
            sum++;
          }
          i++;
        }
        return sum;
      }
      bool string_member_p(char* s, char b) {
        cell i = 0;
        while (s[i] != '\0') {
          if (s[i] == b) {
            return true;
          }
          i++;
        }
        return false;
      }
      char string_last_byte(char* s) {
        cell i = 0;
        while (s[i+1] != 0) {
          i++;
        }
        return s[i];
      }
      // caller free
      char* substring(char* str, cell begin, cell end) {
        cell len = strlen(str);
        char* buf = strdup(str);
        buf[end] = '\0';
        if (begin == 0) {
          return buf;
        }
        else {
          char* s = strdup(buf+begin);
          free(buf);
          return s;
        }
      }
  void p_debug() {
    exit(233);
  }
    struct jotable_entry {
      char *key;
      struct jotable_entry *tag;
      cell data;
    };

    typedef struct jotable_entry* jo_t;

    // prime table size
    //   1000003   about 976 k
    //   1000033
    //   1000333
    //   100003    about 97 k
    //   100333
    //   997
    #define JOTABLE_SIZE 100003
    struct jotable_entry jotable[JOTABLE_SIZE];

    // thus (jotable + index) is jo
    bool used_jo_p(jo_t jo) {
      return jo->tag != 0;
    }
    cell string_to_sum(char* str) {
      cell sum = 0;
      cell max_step = 10;
      cell i = 0;
      while (i < strlen(str)) {
        sum = sum + ((char) str[i]) * (2 << min(i, max_step));
        i++;
      }
      return sum;
    }
    // a hash an index into jotable
    cell jotable_hash(cell sum, cell counter) {
      return (counter + sum) % JOTABLE_SIZE;
    }
    void p_debug();

    jo_t jotable_insert(char* key) {
      // in C : [string] -> [jo]
      cell sum = string_to_sum(key);
      cell counter = 0;
      while (true) {
        cell index = jotable_hash(sum, counter);
        jo_t jo = (jotable + index);
        if (jo->key == 0) {
          key = strdup(key);
          jo->key = key;
          return jo;
        }
        else if (string_equal(key, jo->key)) {
          return jo;
        }
        else if (counter == JOTABLE_SIZE) {
          report("- jotable_insert fail\n");
          report("  the hash_table is filled\n");
          p_debug();
          return NULL;
        }
        else {
          counter++;
        }
      }
    }
    jo_t str2jo(char* str) {
      return jotable_insert(str);
    }
    char* jo2str(jo_t jo) {
      return jo->key;
    }
    jo_t EMPTY_JO;
    jo_t TAG_PRIM;
    jo_t TAG_JOJO;
    jo_t TAG_PRIM_KEYWORD;
    jo_t TAG_KEYWORD;
    jo_t TAG_CLASS;
    jo_t TAG_CLOSURE;

    jo_t TAG_BOOL;
    jo_t TAG_INT;
    jo_t TAG_BYTE;
    jo_t TAG_STRING;
    jo_t TAG_JO;

    jo_t TAG_UNINITIALISED_FIELD_PLACE_HOLDER;

    jo_t JO_DECLARED;

    jo_t ROUND_BAR;
    jo_t ROUND_KET;
    jo_t SQUARE_BAR;
    jo_t SQUARE_KET;
    jo_t FLOWER_BAR;
    jo_t FLOWER_KET;
    jo_t DOUBLE_QUOTE;

    jo_t JO_INS_LIT;
    jo_t JO_INS_GET_LOCAL;
    jo_t JO_INS_SET_LOCAL;
    jo_t JO_INS_GET_FIELD;
    jo_t JO_INS_SET_FIELD;

    jo_t JO_INS_JMP;
    jo_t JO_INS_JZ;

    jo_t JO_INS_TAIL_CALL;
    jo_t JO_INS_LOOP;
    jo_t JO_INS_RECUR;

    jo_t JO_NULL;
    jo_t JO_THEN;
    jo_t JO_ELSE;

    jo_t JO_APPLY;
    jo_t JO_END;

    jo_t JO_LOCAL_DATA_IN;
    jo_t JO_LOCAL_DATA_OUT;

    jo_t JO_LOCAL_TAG_IN;
    jo_t JO_LOCAL_TAG_OUT;

    jo_t JO_LOCAL_IN;
    jo_t JO_LOCAL_OUT;
    jo_t name_record[16 * 1024];
    cell name_record_counter = 0;
    void report_name_record() {
      report("- name_record :\n");
      cell i = 0;
      while (i < name_record_counter) {
        report("  %s\n", jo2str(name_record[i]));
        i++;
      }
    }
    bool name_can_bind_p(jo_t name) {
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
    void bind_name(jo_t name,
                   jo_t tag,
                   cell data) {
      if (!name_can_bind_p(name)) {
        report("- bind_name can not rebind\n");
        report("  name : %s\n", jo2str(name));
        report("  tag : %s\n", jo2str(tag));
        report("  data : %ld\n", data);
        report("  it has been bound as a %s\n", jo2str(name->tag));
        return;
      }

      name_record[name_record_counter] = name;
      name_record_counter++;
      name_record[name_record_counter] = 0;

      name->tag = tag;
      name->data = data;
    }
    void rebind_name(name, tag, data)
      jo_t name;
      jo_t tag;
      cell data;
    {
      name->tag = tag;
      name->data = data;
    }
    struct stack_link {
      cell* stack;
      struct stack_link* link;
    };
    struct stack {
      char* name;
      cell pointer;
      cell* stack;
      struct stack_link* link;
    };

    #define STACK_BLOCK_SIZE 1024
    // #define STACK_BLOCK_SIZE 1 // for testing
    struct stack* new_stack(char* name) {
      struct stack* stack = (struct stack*)
        malloc(sizeof(struct stack));
      stack->name = name;
      stack->pointer = 0;
      stack->stack = (cell*)malloc(sizeof(cell) * STACK_BLOCK_SIZE);
      stack->link = NULL;
      return stack;
    }
    void stack_free_link(struct stack_link* link) {
      if (link == NULL) {
        return;
      }
      else {
        stack_free_link(link->link);
        free(link->stack);
        free(link);
      }
    }

    // ><><><
    // stack->name is not freed
    void stack_free(struct stack* stack) {
      stack_free_link(stack->link);
      free(stack->stack);
      free(stack);
    }
    // can not pop
    // for stack->pointer can not decrease under 0

    void stack_block_underflow_check(struct stack* stack) {
      if (stack->pointer > 0) {
        return;
      }
      else if (stack->link != NULL) {
        free(stack->stack);
        stack->stack = stack->link->stack;
        struct stack_link* old_link = stack->link;
        stack->link = stack->link->link;
        free(old_link);
        stack->pointer = STACK_BLOCK_SIZE;
        return;
      }
      else {
        report("- stack_block_underflow_check fail\n");
        report("  %s underflow\n", stack->name);
        p_debug();
      }
    }
    // can not push
    // for stack->pointer can not increase over STACK_BLOCK_SIZE
    void stack_block_overflow_check(struct stack* stack) {
      if (stack->pointer < STACK_BLOCK_SIZE) {
        return;
      }
      else {
        struct stack_link* new_link = (struct stack_link*)
          malloc(sizeof(struct stack_link));
        new_link->stack = stack->stack;
        new_link->link = stack->link;
        stack->link = new_link;
        stack->stack = (cell*)malloc(sizeof(cell) * STACK_BLOCK_SIZE);
        stack->pointer = 0;
      }
    }
    bool stack_empty_p(struct stack* stack) {
      return
        stack->pointer == 0 &&
        stack->link == NULL;
    }
    cell stack_length_link(cell sum, struct stack_link* link) {
      if (link == NULL) {
        return sum;
      }
      else {
        return stack_length_link(sum + STACK_BLOCK_SIZE, link->link);
      }
    }

    cell stack_length(struct stack* stack) {
      return stack_length_link(stack->pointer, stack->link);
    }
    cell pop(struct stack* stack) {
      stack_block_underflow_check(stack);
      stack->pointer--;
      return stack->stack[stack->pointer];
    }
    cell tos(struct stack* stack) {
      stack_block_underflow_check(stack);
      return stack->stack[stack->pointer - 1];
    }
    void drop(struct stack* stack) {
      stack_block_underflow_check(stack);
      stack->pointer--;
    }
    void push(struct stack* stack, cell data) {
      stack_block_overflow_check(stack);
      stack->stack[stack->pointer] = data;
      stack->pointer++;
    }
    cell stack_peek_link(struct stack_link* link, cell index) {
      if (index < STACK_BLOCK_SIZE) {
        return link->stack[STACK_BLOCK_SIZE - index];
      }
      else {
        return stack_peek_link(link->link, index - STACK_BLOCK_SIZE);
      }
    }

    cell stack_peek(struct stack* stack, cell index) {
      if (index <= stack->pointer) {
        return stack->stack[stack->pointer - index];
      }
      else {
        return stack_peek_link(stack->link, index - stack->pointer);
      }
    }
    cell stack_ref(struct stack* stack, cell index) {
      return stack_peek(stack, stack_length(stack) - index);
    }
    typedef enum {
      INPUT_STACK_REGULAR_FILE,
      INPUT_STACK_STRING,
      INPUT_STACK_TERMINAL,
    } input_stack_type;
    struct input_stack_link {
      char* stack;
      cell end_pointer;
      struct input_stack_link* link;
    };
    struct input_stack {
      cell pointer;
      cell end_pointer;
      char* stack;
      struct input_stack_link* link;
      input_stack_type type;
      union {
        int   file;
        char* string;
        // int   terminal;
      };
      cell string_pointer;
    };

    #define INPUT_STACK_BLOCK_SIZE (4 * 1024)
    // #define INPUT_STACK_BLOCK_SIZE 1 // for testing
    struct input_stack* input_stack_new(input_stack_type input_stack_type) {
      struct input_stack* input_stack = (struct input_stack*)
        malloc(sizeof(struct input_stack));
      input_stack->pointer = INPUT_STACK_BLOCK_SIZE;
      input_stack->end_pointer = INPUT_STACK_BLOCK_SIZE;
      input_stack->stack = (char*)malloc(INPUT_STACK_BLOCK_SIZE);
      input_stack->link = NULL;
      input_stack->type = input_stack_type;
      return input_stack;
    }
    struct input_stack* input_stack_file(int file) {
      int file_flag = fcntl(file, F_GETFL);
      int access_mode = file_flag & O_ACCMODE;
      if (file_flag == -1) {
        report("- input_stack_file fail\n");
        perror("  fcntl error ");
        p_debug();
        return NULL;
      }
      struct input_stack* input_stack = input_stack_new(INPUT_STACK_REGULAR_FILE);
      input_stack->file = file;
      return input_stack;
    }
    struct input_stack* input_stack_string(char* string) {
      struct input_stack* input_stack = input_stack_new(INPUT_STACK_STRING);
      input_stack->string = string;
      input_stack->string_pointer = 0;
      return input_stack;
    }
    struct input_stack* input_stack_terminal() {
      struct input_stack* input_stack = input_stack_new(INPUT_STACK_TERMINAL);
      return input_stack;
    }
    void input_stack_free_link(struct input_stack_link* link) {
      if (link == NULL) {
        return;
      }
      else {
        input_stack_free_link(link->link);
        free(link->stack);
        free(link);
      }
    }

    void input_stack_free(struct input_stack* input_stack) {
      input_stack_free_link(input_stack->link);
      free(input_stack->stack);
      free(input_stack);
    }
    // can not pop
    // for input_stack->pointer can not increase over input_stack->end_pointer
    void input_stack_block_underflow_check(struct input_stack* input_stack) {
      if (input_stack->pointer < input_stack->end_pointer) {
        return;
      }

      else if (input_stack->link != NULL) {
        free(input_stack->stack);
        input_stack->stack = input_stack->link->stack;
        input_stack->end_pointer = input_stack->link->end_pointer;
        struct input_stack_link* old_link = input_stack->link;
        input_stack->link = input_stack->link->link;
        free(old_link);
        input_stack->pointer = 0;
        return;
      }

      else if (input_stack->type == INPUT_STACK_REGULAR_FILE) {
        ssize_t real_bytes = read(input_stack->file,
                                  input_stack->stack,
                                  INPUT_STACK_BLOCK_SIZE);
        if (real_bytes == 0) {
          report("- input_stack_block_underflow_check fail\n");
          report("  input_stack underflow\n");
          report("  meet end-of-file when reading a regular_file\n");
          report("  file descriptor : %ld\n", input_stack->file);
          p_debug();
        }
        else {
          input_stack->pointer = 0;
          input_stack->end_pointer = real_bytes;
          return;
        }
      }

      else if (input_stack->type == INPUT_STACK_STRING) {
        char byte = input_stack->string[input_stack->string_pointer];
        if (byte == '\0') {
          report("- input_stack_block_underflow_check fail\n");
          report("  input_stack underflow\n");
          report("  meet end-of-string when reading a string\n");
          p_debug();
        }
        input_stack->string_pointer++;
        input_stack->end_pointer = INPUT_STACK_BLOCK_SIZE;
        input_stack->pointer = INPUT_STACK_BLOCK_SIZE - 1;
        input_stack->stack[input_stack->pointer] = byte;
        return;
      }

      else if (input_stack->type == INPUT_STACK_TERMINAL) {
        ssize_t real_bytes = read(STDIN_FILENO,
                                  input_stack->stack,
                                  INPUT_STACK_BLOCK_SIZE);
        if (real_bytes == 0) {
          report("- input_stack_block_underflow_check fail\n");
          report("  input_stack underflow\n");
          report("  meet end-of-file when reading from terminal\n");
          p_debug();
        }
        else {
          input_stack->pointer = 0;
          input_stack->end_pointer = real_bytes;
          return;
        }
      }

      else {
        report("- input_stack_block_underflow_check fail\n");
        report("  meet unknow stack type\n");
        report("  stack type number : %ld\n", input_stack->type);
        p_debug();
      }
    }
    // can not push
    // for input_stack->pointer can not decrease under 0
    void input_stack_block_overflow_check(struct input_stack* input_stack) {
      if (input_stack->pointer > 0) {
        return;
      }
      else {
        struct input_stack_link* new_link = (struct input_stack_link*)
          malloc(sizeof(struct input_stack_link));
        new_link->stack = input_stack->stack;
        new_link->link = input_stack->link;
        new_link->end_pointer = input_stack->end_pointer;
        input_stack->link = new_link;
        input_stack->stack = (char*)malloc(INPUT_STACK_BLOCK_SIZE);
        input_stack->pointer = INPUT_STACK_BLOCK_SIZE;
        input_stack->end_pointer = INPUT_STACK_BLOCK_SIZE;
      }
    }
    bool input_stack_empty_p(struct input_stack* input_stack) {
      if (input_stack->pointer != input_stack->end_pointer ||
          input_stack->link != NULL) {
        return false;
      }
      if (input_stack->type == INPUT_STACK_REGULAR_FILE) {
        ssize_t real_bytes = read(input_stack->file,
                                  input_stack->stack,
                                  INPUT_STACK_BLOCK_SIZE);
        if (real_bytes == 0) {
          return true;
        }
        else {
          input_stack->pointer = 0;
          input_stack->end_pointer = real_bytes;
          return false;
        }
      }
      else if (input_stack->type == INPUT_STACK_STRING) {
        return input_stack->string[input_stack->string_pointer] == '\0';
      }
      // else if (input_stack->type == INPUT_STACK_TERMINAL)
      else {
        ssize_t real_bytes = read(STDIN_FILENO,
                                  input_stack->stack,
                                  INPUT_STACK_BLOCK_SIZE);
        if (real_bytes == 0) {
          return true;
        }
        else {
          input_stack->pointer = 0;
          input_stack->end_pointer = real_bytes;
          return false;
        }
      }
    }
    char input_stack_pop(struct input_stack* input_stack) {
      input_stack_block_underflow_check(input_stack);
      char byte = input_stack->stack[input_stack->pointer];
      input_stack->pointer++;
      return byte;
    }
    char input_stack_tos(struct input_stack* input_stack) {
      input_stack_block_underflow_check(input_stack);
      char byte = input_stack->stack[input_stack->pointer];
      return byte;
    }
    void input_stack_drop(struct input_stack* input_stack) {
      input_stack_block_underflow_check(input_stack);
      input_stack->pointer++;
    }
    void input_stack_push(struct input_stack* input_stack, char byte) {
      input_stack_block_overflow_check(input_stack);
      input_stack->pointer--;
      input_stack->stack[input_stack->pointer] = byte;
    }
    typedef enum {
      OUTPUT_STACK_REGULAR_FILE,
      OUTPUT_STACK_STRING,
      OUTPUT_STACK_TERMINAL,
    } output_stack_type;
    struct output_stack_link {
      char* stack;
      struct output_stack_link* link;
    };
    struct output_stack {
      cell pointer;
      char* stack;
      struct output_stack_link* link;
      output_stack_type type;
      union {
        int   file; // with cache
        // char* string;
        //   generate string
        //   instead of output to pre-allocated buffer
        // int   terminal; // no cache
      };
      cell string_pointer;
    };

    #define OUTPUT_STACK_BLOCK_SIZE (4 * 1024)
    // #define OUTPUT_STACK_BLOCK_SIZE 1 // for testing
    struct output_stack* output_stack_new(output_stack_type output_stack_type) {
      struct output_stack* output_stack = (struct output_stack*)
        malloc(sizeof(struct output_stack));
      output_stack->pointer = 0;
      output_stack->stack = (char*)malloc(OUTPUT_STACK_BLOCK_SIZE);
      output_stack->link = NULL;
      output_stack->type = output_stack_type;
      return output_stack;
    }
    struct output_stack* output_stack_file(int file) {
      int file_flag = fcntl(file, F_GETFL);
      int access_mode = file_flag & O_ACCMODE;
      if (file_flag == -1) {
        report("- output_stack_file fail\n");
        perror("  fcntl error ");
        p_debug();
        return NULL;
      }
      else if (access_mode == O_WRONLY || access_mode == O_RDWR) {
        struct output_stack* output_stack = output_stack_new(OUTPUT_STACK_REGULAR_FILE);
        output_stack->file = file;
        return output_stack;
      }
      else {
        report("- output_stack_file fail\n");
        report("  output_stack_file fail\n");
        p_debug();
        return NULL;
      }
    }
    struct output_stack* output_stack_string() {
      struct output_stack* output_stack = output_stack_new(OUTPUT_STACK_STRING);
      return output_stack;
    }
    struct output_stack* output_stack_terminal() {
      struct output_stack* output_stack = output_stack_new(OUTPUT_STACK_TERMINAL);
      return output_stack;
    }
    void output_stack_free_link(struct output_stack_link* link) {
      if (link == NULL) {
        return;
      }
      else {
        output_stack_free_link(link->link);
        free(link->stack);
        free(link);
      }
    }

    void output_stack_free(struct output_stack* output_stack) {
      output_stack_free_link(output_stack->link);
      free(output_stack->stack);
      free(output_stack);
    }
    void output_stack_file_flush_link(int file, struct output_stack_link* link) {
      if (link == NULL) {
        return;
      }
      else {
        output_stack_file_flush_link(file, link->link);
        ssize_t real_bytes = write(file,
                                   link->stack,
                                   OUTPUT_STACK_BLOCK_SIZE);
        if (real_bytes != OUTPUT_STACK_BLOCK_SIZE) {
          report("- output_stack_file_flush_link fail\n");
          report("  file descriptor : %ld\n", file);
          perror("  write error : ");
          p_debug();
        }
      }
    }

    void output_stack_file_flush(struct output_stack* output_stack) {
      output_stack_file_flush_link(output_stack->file,
                                   output_stack->link);
      ssize_t real_bytes = write(output_stack->file,
                                 output_stack->stack,
                                 output_stack->pointer);
      if (real_bytes != output_stack->pointer) {
        report("- output_stack_file_flush fail\n");
        report("  file descriptor : %ld\n", output_stack->file);
        perror("  write error : ");
        p_debug();
      }
      else {
        output_stack_free_link(output_stack->link);
        output_stack->link = NULL;
        output_stack->pointer = 0;
      }
    }
    cell output_stack_string_length_link(cell sum, struct output_stack_link* link) {
      if (link == NULL) {
        return sum;
      }
      else {
        return
          OUTPUT_STACK_BLOCK_SIZE +
          output_stack_string_length_link(sum, link->link);
      }
    }

    cell output_stack_string_length(struct output_stack* output_stack) {
      cell sum = strlen(output_stack->stack);
      return output_stack_string_length_link(sum, output_stack->link);
    }


    char* output_stack_string_collect_link(char* buffer, struct output_stack_link* link) {
      if (link == NULL) {
        return buffer;
      }
      else {
        buffer = output_stack_string_collect_link(buffer, link->link);
        memcpy(buffer, link->stack, OUTPUT_STACK_BLOCK_SIZE);
        return buffer + OUTPUT_STACK_BLOCK_SIZE;
      }
    }

    char* output_stack_string_collect(struct output_stack* output_stack) {
      char* string = (char*)malloc(1 + output_stack_string_length(output_stack));
      char* buffer = string;
      buffer = output_stack_string_collect_link(buffer, output_stack->link);
      memcpy(buffer, output_stack->stack, output_stack->pointer);
      buffer[output_stack->pointer] = '\0';
      return string;
    }
    // can not pop
    // for output_stack->pointer can not decrease under 0
    void output_stack_block_underflow_check(struct output_stack* output_stack) {
      if (output_stack->pointer > 0) {
        return;
      }

      else if (output_stack->link != NULL) {
        free(output_stack->stack);
        output_stack->stack = output_stack->link->stack;
        struct output_stack_link* old_link = output_stack->link;
        output_stack->link = output_stack->link->link;
        free(old_link);
        output_stack->pointer = OUTPUT_STACK_BLOCK_SIZE;
        return;
      }

      else if (output_stack->type == OUTPUT_STACK_REGULAR_FILE) {
        report("- output_stack_block_underflow_check fail\n");
        report("  output_stack underflow\n");
        report("  when writing a regular_file\n");
        report("  file descriptor : %ld\n", output_stack->file);
        p_debug();
      }

      else if (output_stack->type == OUTPUT_STACK_STRING) {
        report("- output_stack_block_underflow_check fail\n");
        report("  output_stack underflow\n");
        report("  when writing a string\n");
        p_debug();
      }

      else if (output_stack->type == OUTPUT_STACK_TERMINAL) {
        report("- output_stack_block_underflow_check fail\n");
        report("  output_stack underflow\n");
        report("  when writing to terminal\n");
        p_debug();
      }

      else {
        report("- output_stack_block_underflow_check fail\n");
        report("  meet unknow stack type\n");
        report("  stack type number : %ld\n", output_stack->type);
        p_debug();
      }
    }
    // can not push
    // for output_stack->pointer can not increase over OUTPUT_STACK_BLOCK_SIZE
    void output_stack_block_overflow_check(struct output_stack* output_stack) {
      if (output_stack->pointer < OUTPUT_STACK_BLOCK_SIZE) {
        return;
      }
      else {
        struct output_stack_link* new_link = (struct output_stack_link*)
          malloc(sizeof(struct output_stack_link));
        new_link->stack = output_stack->stack;
        new_link->link = output_stack->link;
        output_stack->link = new_link;
        output_stack->stack = (char*)malloc(OUTPUT_STACK_BLOCK_SIZE);
        output_stack->pointer = 0;
      }
    }
    bool output_stack_empty_p(struct output_stack* output_stack) {
      if (output_stack->pointer != 0 ||
          output_stack->link != NULL) {
        return false;
      }
      if (output_stack->type == OUTPUT_STACK_REGULAR_FILE) {
        return true;
      }
      else if (output_stack->type == OUTPUT_STACK_STRING) {
        return true;
      }
      // else if (output_stack->type == OUTPUT_STACK_TERMINAL)
      else {
        return true;
      }
    }
    char output_stack_pop(struct output_stack* output_stack) {
      output_stack_block_underflow_check(output_stack);
      output_stack->pointer--;
      return output_stack->stack[output_stack->pointer];
    }
    char output_stack_tos(struct output_stack* output_stack) {
      output_stack_block_underflow_check(output_stack);
      return output_stack->stack[output_stack->pointer - 1];
    }
    void output_stack_drop(struct output_stack* output_stack) {
      output_stack_block_underflow_check(output_stack);
      output_stack->pointer--;
    }
    void output_stack_push(struct output_stack* output_stack, char b) {
      if (output_stack->type == OUTPUT_STACK_TERMINAL) {
        char buffer[1];
        buffer[0] = b;
        ssize_t real_bytes = write(STDOUT_FILENO, buffer, 1);
        if (real_bytes != 1) {
          report("- output_stack_push fail\n");
          perror("  write error : ");
          p_debug();
        }
      }
      else {
        output_stack_block_overflow_check(output_stack);
        output_stack->stack[output_stack->pointer] = b;
        output_stack->pointer++;
      }
    }
    struct obj {
      jo_t tag;
      cell data;
    };

    struct stack* object_stack;

    struct obj object_stack_pop() {
      struct obj p;
      p.tag = pop(object_stack);
      p.data = pop(object_stack);
      return p;
    }

    struct obj object_stack_tos() {
      struct obj p;
      p.tag = pop(object_stack);
      p.data = pop(object_stack);
      push(object_stack, p.data);
      push(object_stack, p.tag);
      return p;
    }

    bool object_stack_empty_p() {
      return stack_empty_p(object_stack);
    }

    void object_stack_push(jo_t tag, cell data) {
      push(object_stack, data);
      push(object_stack, tag);
    }

    jo_t object_stack_peek_tag(cell index) {
      return stack_peek(object_stack, (index*2) - 1);
    }

    cell object_stack_peek_data(cell index) {
      return stack_peek(object_stack, (index*2));
    }
    struct local {
      jo_t name;
      cell local_tag;
      cell local_data;
    };

    #define LOCAL_RECORD_SIZE (32 * 1024)
    struct local local_record[LOCAL_RECORD_SIZE];
    cell current_local_counter = 0;
    struct ret {
      jo_t* jojo;
      cell local_counter;
    };

    struct stack* return_stack;

    struct ret return_stack_pop() {
      struct ret p;
      p.jojo = pop(return_stack);
      p.local_counter = pop(return_stack);
      return p;
    }

    struct ret return_stack_tos() {
      struct ret p;
      p.jojo = pop(return_stack);
      p.local_counter = pop(return_stack);
      push(return_stack, p.local_counter);
      push(return_stack, p.jojo);
      return p;
    }

    bool return_stack_empty_p() {
      return stack_empty_p(return_stack);
    }

    void return_stack_push(jo_t* jojo, cell local_counter) {
      push(return_stack, local_counter);
      push(return_stack, jojo);
    }

    void return_stack_push_new(jo_t* jojo) {
      return_stack_push(jojo, current_local_counter);
    }

    void return_stack_inc() {
      jo_t* jojo = pop(return_stack);
      push(return_stack, jojo + 1);
    }
    typedef enum {
      GC_STATE_MARKING,
      GC_STATE_SWEEPING,
    } gc_state_t;
    typedef void (* gc_actor_t)(gc_state_t, cell);
    typedef enum {
      GC_MARK_FREE,
      GC_MARK_USING,
    } gc_mark_t;

    struct object_entry {
      gc_mark_t mark;
      gc_actor_t gc_actor;
      cell fields_number;
      cell pointer;
    };

    #define OBJECT_RECORD_SIZE 64 * 1024
    // #define OBJECT_RECORD_SIZE 3 // for testing

    struct object_entry object_record[OBJECT_RECORD_SIZE];

    struct object_entry* object_record_pointer = object_record;
    bool object_record_end_p() {
      return object_record_pointer >= (object_record + OBJECT_RECORD_SIZE);
    }
    void init_object_record() {
      bzero(object_record,
            OBJECT_RECORD_SIZE *
            sizeof(struct object_entry));
    }
    typedef void (* executer_t)(cell);

    struct class {
      jo_t class_name;
      gc_actor_t gc_actor;
      bool executable;
      executer_t executer;
      cell fields_number;
      jo_t* fields;
    };
      jo_t get_field_tag(cell* fields, cell field_index) {
        return fields[field_index*2+1];
      }

      void set_field_tag(cell* fields, cell field_index, jo_t tag) {
        fields[field_index*2+1] = tag;
      }

      cell get_field_data(cell* fields, cell field_index) {
        return fields[field_index*2];
      }

      void set_field_data(cell* fields, cell field_index, cell data) {
        fields[field_index*2] = data;
      }
      // assume exist
      jo_t class_index_to_field_name(struct class* class, cell index) {
        return class->fields[index];
      }
      // assume exist
      cell class_field_name_to_index(struct class* class, jo_t field_name) {
        cell i = 0;
        while (i < class->fields_number) {
          if (class->fields[i] == field_name) { return i; }
          i++;
        }
        report("- class_field_name_to_index fail\n");
        report("  field_name : %s\n", jo2str(field_name));
        report("  class_name : %s\n", jo2str(class->class_name));
        p_debug();
      }
      jo_t get_object_field_tag(object_entry, field_index)
        struct object_entry* object_entry;
        cell field_index;
      {
        cell* fields = object_entry->pointer;
        return get_field_tag(fields, field_index);
      }

      void set_object_field_tag(object_entry, field_index, tag)
        struct object_entry* object_entry;
        cell field_index;
        jo_t tag;
      {
        cell* fields = object_entry->pointer;
        set_field_tag(fields, field_index, tag);
      }

      cell get_object_field_data(object_entry, field_index)
        struct object_entry* object_entry;
        cell field_index;
      {
        cell* fields = object_entry->pointer;
        return get_field_data(fields, field_index);
      }

      void set_object_field_data(object_entry, field_index, data)
        struct object_entry* object_entry;
        cell field_index;
        cell data;
      {
        cell* fields = object_entry->pointer;
        set_field_data(fields, field_index, data);
      }
    struct obj get_field(jo_t class_tag, struct object_entry* object_entry, jo_t name) {
      struct class* class = class_tag->data;
      cell index = class_field_name_to_index(class, name);
      jo_t tag = get_object_field_tag(object_entry, index);
      cell data = get_object_field_data(object_entry, index);

      if (tag == TAG_UNINITIALISED_FIELD_PLACE_HOLDER) {
        object_stack_push(class_tag, object_entry);
        report("- get_field fail\n");
        report("  field is uninitialised\n");
        report("  field_name : %s\n", jo2str(name));
        report("  class_name : %s\n", jo2str(class->class_name));
        report("  see top of object_stack for the object\n");
        p_debug();
      }
      else {
        struct obj a;
        a.tag = tag;
        a.data = data;
        return a;
      }
    }
    void ins_get_field() {
      struct ret rp = return_stack_tos();
      return_stack_inc();
      jo_t* jojo = rp.jojo;
      jo_t name = jojo[0];

      struct obj a = object_stack_pop();
      struct class* class = a.tag->data;

      cell index = class_field_name_to_index(class, name);

      jo_t tag = get_object_field_tag(a.data, index);
      cell data = get_object_field_data(a.data, index);
      if (tag == TAG_UNINITIALISED_FIELD_PLACE_HOLDER) {
        object_stack_push(a.tag, a.data);
        report("- ins_get_field fail\n");
        report("  field is uninitialised\n");
        report("  field_name : %s\n", jo2str(name));
        report("  class_name : %s\n", jo2str(class->class_name));
        report("  see top of object_stack for the object\n");
        p_debug();
      }
      else {
        object_stack_push(tag, data);
      }
    }
    void ins_set_field() {
      struct ret rp = return_stack_tos();
      return_stack_inc();
      jo_t* jojo = rp.jojo;
      jo_t name = jojo[0];

      struct obj a = object_stack_pop();
      struct class* class = a.tag->data;

      cell index = class_field_name_to_index(class, name);

      struct obj b = object_stack_pop();
      set_object_field_tag(a.data, index, b.tag);
      set_object_field_data(a.data, index, b.data);
    }
    void mark_one(jo_t tag, cell data) {
      struct class* class = tag->data;
      class->gc_actor(GC_STATE_MARKING, data);
    }
    void mark_object_record() {
      // prepare
      cell i = 0;
      while (i < OBJECT_RECORD_SIZE) {
        object_record[i].mark = GC_MARK_FREE;
        i++;
      }
      // name_record as root
      i = 0;
      while (i < name_record_counter) {
        jo_t name = name_record[i];
        mark_one(name->tag, name->data);
        i++;
      }
      // object_stack as root
      i = 0;
      while (i < stack_length(object_stack)) {
        mark_one(stack_ref(object_stack, i+1),
                 stack_ref(object_stack, i));
        i++;
        i++;
      }
    }
    void sweep_one(struct object_entry* object_entry) {
      if (object_entry->mark == GC_MARK_USING) {
        return;
      }
      else {
        object_entry->gc_actor(GC_STATE_SWEEPING, object_entry);
      }
    }
    void sweep_object_record() {
      cell i = 0;
      while (i < OBJECT_RECORD_SIZE) {
        sweep_one(object_record + i);
        i++;
      }
    }
      void gc_ignore(gc_state_t gc_state, cell data) {
        if (gc_state == GC_STATE_MARKING) {
        }
        else if (gc_state == GC_STATE_SWEEPING) {
        }
      }
      void gc_free(gc_state_t gc_state, struct object_entry* object_entry) {
        if (gc_state == GC_STATE_MARKING) {
          object_entry->mark = GC_MARK_USING;
        }
        else if (gc_state == GC_STATE_SWEEPING) {
          free(object_entry->pointer);
        }
      }
      void gc_recur(gc_state_t gc_state, struct object_entry* object_entry) {
        if (gc_state == GC_STATE_MARKING) {
          if (object_entry->mark == GC_MARK_USING) { return; }
          object_entry->mark = GC_MARK_USING;
          cell fields_number = object_entry->fields_number;
          cell* fields = object_entry->pointer;
          cell i = 0;
          while (i < fields_number) {
            mark_one(get_object_field_tag(object_entry, i),
                     get_object_field_data(object_entry, i));
            i++;
          }
        }
        else if (gc_state == GC_STATE_SWEEPING) {
          free(object_entry->pointer);
        }
      }
    void run_gc() {
      mark_object_record();
      sweep_object_record();
    }

    // run_gc() {
    //   report("- run_gc()\n");
    //   mark_object_record();
    //   report("- after mark_object_record()\n");
    //   sweep_object_record();
    //   report("- after sweep_object_record()\n");
    // }
    void next_free_record_object_entry() {
      while (!object_record_end_p() &&
             object_record_pointer->mark != GC_MARK_FREE) {
        object_record_pointer++;
      }
    }

    struct object_entry* new_record_object_entry() {
      next_free_record_object_entry();
      if (!object_record_end_p()) {
        return object_record_pointer++;
      }
      else {
        run_gc();
        object_record_pointer = object_record;
        if (!object_record_end_p()) {
          return object_record_pointer++;
        }
        else {
          report("- new_record_object_entry fail\n");
          report("  after gc, the object_record is still filled\n");
          report("  OBJECT_RECORD_SIZE : %ld\n", OBJECT_RECORD_SIZE);
          return NULL;
        }
      }
    }
    struct object_entry* new_static_object_entry() {
      struct object_entry* object_entry = (struct object_entry*)
        malloc(sizeof(struct object_entry));
      return object_entry;
    }
    struct object_entry* new(struct class* class) {
      cell* fields = (cell*)malloc(class->fields_number*2*sizeof(cell));

      cell i = 0;
      while (i < class->fields_number) {
        set_field_tag(fields, i, str2jo("<uninitialised-field-place-holder>"));
        i++;
      }

      struct object_entry* object_entry = new_record_object_entry();
      object_entry->gc_actor = gc_recur;
      object_entry->pointer = fields;
      object_entry->fields_number = class->fields_number;

      return object_entry;
    }
      void add_atom_data_exe(class_name, gc_actor, executer)
        char* class_name;
        gc_actor_t gc_actor;
        executer_t executer;
      {
        struct class* class = (struct class*)
          malloc(sizeof(struct class));
        class->class_name = str2jo(class_name);
        class->gc_actor = gc_actor;
        if (executer == NULL) {
          class->executable = false;
        }
        else {
          class->executable = true;
          class->executer = executer;
        }

        jo_t name = str2jo(class_name);
        bind_name(name, str2jo("<class>"), class);
      }
      void add_atom_data(class_name, gc_actor)
        char* class_name;
        gc_actor_t gc_actor;
      {
        add_atom_data_exe(class_name, gc_actor, NULL);
      }
      void add_data_exe(class_name, executer, fields)
        char* class_name;
        executer_t executer;
        jo_t* fields[];
      {
        struct class* class = (struct class*)
          malloc(sizeof(struct class));
        jo_t name = str2jo(class_name);
        class->class_name = name;
        class->gc_actor = gc_recur;
        if (executer == NULL) {
          class->executable = false;
        }
        else {
          class->executable = true;
          class->executer = executer;
        }

        cell i = 0;
        while (fields[i] != NULL) {
          i++;
        }

        class->fields_number = i;
        class->fields = fields;

        bind_name(name, str2jo("<class>"), class);

        char* tmp = substring(class_name, 1, strlen(class_name) -1);
        jo_t data_constructor_name = str2jo(tmp);
        free(tmp);
        bind_name(data_constructor_name, str2jo("<data-constructor>"), class);

        char* tmp2 = malloc(strlen(jo2str(data_constructor_name) + 1 + 1));
        tmp2[0] = '\0';
        strcat(tmp2, jo2str(data_constructor_name));
        strcat(tmp2, "?");
        jo_t data_predicate_name = str2jo(tmp2);
        free(tmp2);
        bind_name(data_predicate_name, str2jo("<data-predicate>"), class);
      }
      void add_data(class_name, fields)
        char* class_name;
        jo_t* fields[];
      {
        add_data_exe(class_name, NULL, fields);
      }
      void _add_data(name, fields)
        jo_t name;
        jo_t fields[];
      {
        add_data(jo2str(name), fields);
      }
      void add_prim_general(tag, function_name, fun)
        jo_t tag;
        char* function_name;
        primitive_t fun;
      {
        jo_t name = str2jo(function_name);
        bind_name(name, tag, fun);
      }
      void add_prim(function_name, fun)
        char* function_name;
        primitive_t fun;
      {
        add_prim_general(TAG_PRIM, function_name, fun);
      }
      void add_prim_keyword(function_name, fun)
        char* function_name;
        primitive_t fun;
      {
        add_prim_general(TAG_PRIM_KEYWORD, function_name, fun);
      }
    struct stack* keyword_stack; // of alias_pointer
    struct alias {
      jo_t nick;
      jo_t name;
    };

    struct alias alias_record[1024];
    cell current_alias_pointer = 0;
      void exe_prim(primitive_t primitive) {
        primitive();
      }
      void exe_prim_keyword(primitive_t primitive) {
        push(keyword_stack, current_alias_pointer);
        primitive();
        current_alias_pointer = pop(keyword_stack);
      }
      void exe_jojo(jo_t* jojo) {
        return_stack_push_new(jojo);
      }
      void eval();

      void exe_keyword(jo_t* jojo) {
        // keywords are always evaled
        push(keyword_stack, current_alias_pointer);
        return_stack_push_new(jojo);
        eval();
        current_alias_pointer = pop(keyword_stack);
      }
      void exe_set_global_variable(jo_t name) {
        struct obj a = object_stack_pop();
        rebind_name(name, a.tag, a.data);
      }
      void exe_data_constructor(struct class* class) {
        cell* fields = (cell*)malloc(class->fields_number*2*sizeof(cell));

        cell i = 0;
        while (i < class->fields_number) {
          struct obj a = object_stack_pop();
          set_field_tag(fields, (class->fields_number - (i+1)), a.tag);
          set_field_data(fields, (class->fields_number - (i+1)), a.data);
          i++;
        }

        struct object_entry* object_entry = new_record_object_entry();
        object_entry->gc_actor = gc_recur;
        object_entry->pointer = fields;
        object_entry->fields_number = class->fields_number;

        object_stack_push(class->class_name, object_entry);
      }
      void exe_data_predicate(struct class* class) {
        struct obj a = object_stack_pop();
        object_stack_push(TAG_BOOL, (class->class_name == a.tag));
      }
      // caller free
      jo_t* generate_jo_array(char*ss[]) {
        cell len = 0;
        while (ss[len] != NULL) {
          len++;
        }
        jo_t* js = (jo_t*)malloc(len * sizeof(jo_t) + 1);
        cell i = 0;
        while (i < len) {
          js[i] = str2jo(ss[i]);
          i++;
        }
        js[i] = NULL;
        return js;
      }
      #define J0 (char*[]){NULL}
      #define J(...) generate_jo_array((char*[]){__VA_ARGS__, NULL})
    void p_tag() {
      struct obj a = object_stack_pop();
      object_stack_push(TAG_JO, a.tag);
    }
    void p_eq_p() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_BOOL, (b.tag == a.tag) && (b.data == a.data));
    }
    void expose_object() {
      init_object_record();

      add_prim("ins/get-field", ins_get_field);
      add_prim("ins/set-field", ins_set_field);

      add_atom_data("<byte>", gc_ignore);
      add_atom_data("<int>", gc_ignore);
      add_atom_data("<jo>", gc_ignore);
      add_atom_data("<string>", gc_free);
      add_atom_data("<class>", gc_ignore);
      add_atom_data("<uninitialised-field-place-holder>", gc_ignore);

      add_atom_data_exe("<prim>", gc_ignore, exe_prim);
      add_atom_data_exe("<prim-keyword>", gc_ignore, exe_prim_keyword);
      add_atom_data_exe("<jojo>", gc_ignore, exe_jojo);
      add_atom_data_exe("<keyword>", gc_ignore, exe_keyword);
      add_atom_data_exe("<set-global-variable>", gc_ignore, exe_set_global_variable);
      add_atom_data_exe("<data-constructor>", gc_ignore, exe_data_constructor);
      add_atom_data_exe("<data-predicate>", gc_ignore, exe_data_predicate);


      add_prim("tag", p_tag);
      add_prim("eq?", p_eq_p);
    }
    void exe(jo_t tag, cell data) {
      struct class* class = tag->data;
      class->executer(data);
    }
    void p_debug();

    void jo_apply(jo_t jo) {
      if (!used_jo_p(jo)) {
        report("- jo_apply meet undefined jo : %s\n", jo2str(jo));
        p_debug();
        return;
      }
      struct class* class = jo->tag->data;
      if (class->executable) {
        exe(jo->tag, jo->data);
      }
      else {
        push(object_stack, jo->data);
        push(object_stack, jo->tag);
      }
    }
    void eval() {
      cell base = return_stack->pointer;
      while (return_stack->pointer >= base) {
        struct ret rp = return_stack_tos();
        return_stack_inc();
        jo_t* jojo = rp.jojo;
        jo_t jo = jojo[0];
        jo_apply(jo);
      }
    }
    void p_drop() {
      object_stack_pop();
    }
    void p_dup() {
      struct obj a = object_stack_pop();
      object_stack_push(a.tag, a.data);
      object_stack_push(a.tag, a.data);
    }
    void p_over() {
      // b a -> b a b
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(b.tag, b.data);
      object_stack_push(a.tag, a.data);
      object_stack_push(b.tag, b.data);
    }
    void p_tuck() {
      // b a -> a b a
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(a.tag, a.data);
      object_stack_push(b.tag, b.data);
      object_stack_push(a.tag, a.data);
    }
    void p_swap() {
      // b a -> a b
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(a.tag, a.data);
      object_stack_push(b.tag, b.data);
    }
    void expose_stack() {
      add_prim("drop", p_drop);
      add_prim("dup",  p_dup);
      add_prim("over", p_over);
      add_prim("tuck", p_tuck);
      add_prim("swap", p_swap);
    }
    void p_end() {
      struct ret rp = return_stack_pop();
      current_local_counter = rp.local_counter;
    }
    void p_bye() {
      report("bye bye ^-^/\n");
      exit(0);
    }
    void expose_ending() {
      add_prim("end", p_end);
      add_prim("bye", p_bye);
    }
    struct stack* reading_stack; // of input_stack
    struct stack* writing_stack; // of output_stack
    bool has_byte_p() {
      return !input_stack_empty_p(tos(reading_stack));
    }
    char read_byte() {
      return input_stack_pop(tos(reading_stack));
    }
    void byte_unread(char b) {
      input_stack_push(tos(reading_stack), b);
    }
    void write_byte(char b) {
      output_stack_push(tos(writing_stack), b);
    }
    bool has_jo_p() {
      char c;
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
    jo_t read_raw_jo() {
      char buf[1024];
      cell cur = 0;
      cell collecting = false;
      char c;
      char go = true;

      while (go) {

        if (!has_byte_p()) {
          if (!collecting) {
            report("- p_read_raw_jo meet end-of-file\n");
            p_debug();
            return NULL;
          }
          else {
            break;
          }
        }

        c = read_byte(); // report("- read_byte() : %c\n", c);

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
      return str2jo(buf);
    }
    jo_t read_jo() {
      return read_raw_jo();
    }
    void string_unread(char* str) {
      if (str[0] == '\0') {
        return;
      }
      else {
        string_unread(str+1);
        byte_unread(str[0]);
      }
    }
    void jo_unread(jo_t jo) {
      char* str = jo2str(jo);
      byte_unread(' ');
      string_unread(str);
      byte_unread(' ');
    }
    void p_newline() {
      output_stack_push(tos(writing_stack), '\n');
    }
    void p_space() {
      output_stack_push(tos(writing_stack), ' ');
    }
    void expose_rw() {
      add_prim("newline", p_newline);
      add_prim("space", p_space);
    }
    cell local_find(jo_t name) {
      // return index of local_record
      // -1 -- no found
      struct ret rp = return_stack_tos();
      cell cursor = current_local_counter - 1;
      while (cursor >= rp.local_counter) {
        if (local_record[cursor].name == name) {
          return cursor;
        }
        else {
          cursor--;
        }
      }
      return -1;
    }
    void set_local(jo_t name, jo_t tag, cell data) {
      cell index = local_find(name);
      if (index != -1) {
        local_record[index].name = name;
        local_record[index].local_tag = tag;
        local_record[index].local_data = data;
      }
      else if (current_local_counter < LOCAL_RECORD_SIZE) {
        local_record[current_local_counter].name = name;
        local_record[current_local_counter].local_tag = tag;
        local_record[current_local_counter].local_data = data;
        current_local_counter++;
      }
      else {
        report("- set_local fail\n");
        report("  local_record is filled\n");
        report("  LOCAL_RECORD_SIZE : %ld\n", LOCAL_RECORD_SIZE);
        report("  name : %s\n", jo2str(name));
        report("  tag : %s\n", jo2str(tag));
        report("  data : %ld\n", data);
        p_debug();
      }
    }
    void ins_set_local() {
      struct ret rp = return_stack_tos();
      return_stack_inc();
      jo_t* jojo = rp.jojo;
      jo_t name = jojo[0];

      struct obj a = object_stack_pop();
      set_local(name, a.tag, a.data);
    }
    void ins_get_local() {
      struct ret rp = return_stack_tos();
      return_stack_inc();
      jo_t* jojo = rp.jojo;
      jo_t name = jojo[0];

      cell index = local_find(name);

      if (index != -1) {
        struct local lp = local_record[index];
        object_stack_push(lp.local_tag, lp.local_data);
      }
      else {
        report("- ins_get_local fatal error\n");
        report("  name is not bound\n");
        report("  name : %s\n", jo2str(name));
        p_debug();
      }
    }
    void expose_local() {
      add_prim("ins/get-local", ins_get_local);
      add_prim("ins/set-local", ins_set_local);
    }
    struct stack* compiling_stack; // of jojo

    void p_compiling_stack_inc() {
      jo_t* jojo = pop(compiling_stack);
      push(compiling_stack, jojo + 1);
    }
    void here(cell n) {
      jo_t* jojo = pop(compiling_stack);
      jojo[0] = n;
      push(compiling_stack, jojo + 1);
    }
      // :local
      bool get_local_string_p(char* str) {
        if (str[0] != ':') {
          return false;
        }
        else if (string_last_byte(str) == '!') {
          return false;
        }
        else if (string_member_p(str, '.')) {
          return false;
        }
        else {
          return true;
        }
      }
      // :local!
      bool set_local_string_p(char* str) {
        if (str[0] != ':') {
          return false;
        }
        else if (string_last_byte(str) != '!') {
          return false;
        }
        else if (string_member_p(str, '.')) {
          return false;
        }
        else {
          return true;
        }
      }
      // :local.field
      bool get_local_field_string_p(char* str) {
        if (str[0] != ':') {
          return false;
        }
        else if (string_last_byte(str) == '!') {
          return false;
        }
        else if (string_count_member(str, '.') != 1) {
          return false;
        }
        else {
          return true;
        }
      }
      // :local.field!
      bool set_local_field_string_p(char* str) {
        if (str[0] != ':') {
          return false;
        }
        else if (string_last_byte(str) != '!') {
          return false;
        }
        else if (string_count_member(str, '.') != 1) {
          return false;
        }
        else {
          return true;
        }
      }
      // .field
      bool get_field_string_p(char* str) {
        if (str[0] != '.') {
          return false;
        }
        else if (string_last_byte(str) == '!') {
          return false;
        }
        else if (string_count_member(str, '.') != 1) {
          return false;
        }
        else {
          return true;
        }
      }
      // .field!
      bool set_field_string_p(char* str) {
        if (str[0] != '.') {
          return false;
        }
        else if (string_last_byte(str) != '!') {
          return false;
        }
        else if (string_count_member(str, '.') != 1) {
          return false;
        }
        else {
          return true;
        }
      }
    void compile_string() {
      // "..."
      char buffer[1024 * 1024];
      cell cursor = 0;
      while (true) {
        char c = read_byte();
        if (c == '"') {
          buffer[cursor] = '\0';
          cursor++;
          break;
        }
        else {
          buffer[cursor] = c;
          cursor++;
        }
      }
      char* str = strdup(buffer);
      struct object_entry* object_entry = new_static_object_entry();
      object_entry->gc_actor = gc_ignore;
      object_entry->pointer = str;

      here(JO_INS_LIT);
      here(TAG_STRING);
      here(object_entry);
    }
    bool compile_jo(jo_t jo) {
      if (jo == ROUND_BAR) {
        jo_apply(read_jo());
        return true;
      }

      char* str = jo2str(jo);
      // number
      if (int_string_p(str)) {
        here(JO_INS_LIT);
        here(TAG_INT);
        here(string_to_int(str));
        return true;
      }
      // "string"
      else if (jo == DOUBLE_QUOTE) {
        compile_string();
        return true;
      }
      // :local
      else if (get_local_string_p(str)) {
        here(JO_INS_GET_LOCAL);
        here(jo);
        return true;
      }
      // :local!
      else if (set_local_string_p(str)) {
        here(JO_INS_SET_LOCAL);
        char* tmp = substring(str, 0, strlen(str) -1);
        here(str2jo(tmp));
        free(tmp);
        return true;
      }
      // .field
      else if (get_field_string_p(str)) {
        here(JO_INS_GET_FIELD);
        here(jo);
        return true;
      }
      // .field!
      else if (set_field_string_p(str)) {
        here(JO_INS_SET_FIELD);
        char* tmp = substring(str, 0, strlen(str) -1);
        here(str2jo(tmp));
        free(tmp);
        return true;
      }
      // 'jo
      else if (str[0] == '\'' && strlen(str) != 1) {
        here(JO_INS_LIT);
        here(TAG_JO);
        char* tmp = substring(str, 1, strlen(str));
        here(str2jo(tmp));
        free(tmp);
        return true;
      }
      else if (used_jo_p(jo)) {
        here(jo);
        return true;
      }
      else {
        // no compile before define
        report("- compile_jo meet undefined jo : %s\n", jo2str(jo));
        return false;
      }
    }
    bool compile_until_meet_jo(jo_t ending_jo) {
      while (true) {
        jo_t jo = read_jo();
        if (jo == ending_jo) {
          return true;
        }
        if (!compile_jo(jo)) {
          report("- compile_until_meet_jo fail\n");
          // report("  the rest of the ...\n");
          // p_dump();
          p_debug();
          return false;
        }
      }
    }
    jo_t compile_until_meet_jo_or_jo(jo_t ending_jo1, jo_t ending_jo2) {
      while (true) {
        jo_t jo = read_jo();
        if (jo == ending_jo1 || jo == ending_jo2) {
          return jo;
        }
        else {
          compile_jo(jo);
        }
      }
    }
    void p_compile_until_round_ket() {
      compile_until_meet_jo(ROUND_KET);
    }
    struct stack* current_compiling_exe_stack;
    // of data and tag
    void expose_compiler() {

    }
    void k_ignore() {
      while (true) {
        jo_t s = read_raw_jo();
        if (s == ROUND_BAR) {
          k_ignore();
        }
        if (s == ROUND_KET) {
          break;
        }
      }
    }
    void ins_lit() {
      struct ret rp = return_stack_tos();
      return_stack_inc();
      return_stack_inc();
      jo_t* jojo = rp.jojo;
      jo_t tag = jojo[0];
      cell data = jojo[1];
      object_stack_push(tag, data);
    }
    void ins_jmp() {
      struct ret rp = return_stack_pop();
      jo_t* jojo = rp.jojo;
      cell offset = jojo[0];
      return_stack_push(jojo + offset, rp.local_counter);
    }
    void ins_jz() {
      struct ret rp = return_stack_tos();
      return_stack_inc();
      jo_t* jojo = rp.jojo;
      cell offset = jojo[0];
      struct obj a = object_stack_pop();
      if (a.tag == TAG_BOOL && a.data == false) {
        struct ret rp1 = return_stack_pop();
        return_stack_push(jojo + offset, rp1.local_counter);
      }
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
      here(JO_INS_JZ);
      jo_t* end_of_then = tos(compiling_stack);
      p_compiling_stack_inc();
      jo_t ending_jo = compile_until_meet_jo_or_jo(JO_ELSE, ROUND_KET);
      if (ending_jo == ROUND_KET) {
        end_of_then[0] = (jo_t*)tos(compiling_stack) - end_of_then;
        return;
      }
      else {
        here(JO_INS_JMP);
        jo_t* end_of_else = tos(compiling_stack);
        p_compiling_stack_inc();
        end_of_then[0] = (jo_t*)tos(compiling_stack) - end_of_then;
        p_compile_until_round_ket();
        end_of_else[0] = (jo_t*)tos(compiling_stack) - end_of_else;
        return;
      }
    }
    void compile_maybe_square() {
      jo_t first_jo = read_jo();
      if (first_jo == SQUARE_BAR) { compile_until_meet_jo(SQUARE_KET); }
      else { compile_jo(first_jo); }
    }
    //   (case [...]
    //     data-constructor-name [...]
    //     ...)
    //   ==>

    //     [...]

    //     dup tag 'tag eq?
    //     jump_if_false[:end-of-this-case]
    //     drop [...]
    //     jump[:end-of-case]
    //   :end-of-this-case

    //     ...

    //   :end-of-case
    //     drop

    void k_case() {
      compile_maybe_square();
      cell counter = 0;
      cell case_ends[256];

      jo_t dc;
      while (true) {
        dc = read_jo();
        if (dc == ROUND_KET) { break; }
        {
          here(str2jo("dup"));
          here(str2jo("tag"));
          {
            char* tmp = malloc(strlen(jo2str(dc) + 2 + 1));
            tmp[0] = '\0';
            strcat(tmp, "<");
            strcat(tmp, jo2str(dc));
            strcat(tmp, ">");
            here(JO_INS_LIT); here(TAG_JO); here(str2jo(tmp));
            free(tmp);
          }
          here(str2jo("eq?"));
          here(JO_INS_JZ);
          jo_t* end_of_this_case = tos(compiling_stack);
          p_compiling_stack_inc();
          here(str2jo("drop"));
          compile_maybe_square();

          here(JO_INS_JMP);
          case_ends[counter] = tos(compiling_stack);
          counter++;
          p_compiling_stack_inc();

          end_of_this_case[0] = (jo_t*)tos(compiling_stack) - end_of_this_case;
        }
      }

      jo_t* end_of_case;
      while (counter > 0) {
        counter--;
        end_of_case = case_ends[counter];
        end_of_case[0] = (jo_t*)tos(compiling_stack) - end_of_case;
      }
    }
    void ins_tail_call() {
      struct ret rp = return_stack_pop();
      current_local_counter = rp.local_counter;
      jo_t* jojo = rp.jojo;
      jo_t jo = jojo[0];
      jo_apply(jo);
    }
    void k_tail_call() {
      // no check for "no compile before define"
      here(JO_INS_TAIL_CALL);
      here(read_jo());
      k_ignore();
    }
    void ins_loop() {
      struct ret rp = return_stack_pop();
      current_local_counter = rp.local_counter;
      jo_t* jojo = rp.jojo;
      jo_t tag = jojo[0];
      cell data = jojo[1];
      exe(tag, data);
    }
    void k_loop() {
      here(JO_INS_LOOP);

      jo_t tag = pop(current_compiling_exe_stack);
      cell data = pop(current_compiling_exe_stack);
      push(current_compiling_exe_stack, data);
      push(current_compiling_exe_stack, tag);

      here(tag);
      here(data);

      k_ignore();
    }
    void ins_recur() {
      struct ret rp = return_stack_tos();
      return_stack_inc();
      return_stack_inc();
      jo_t* jojo = rp.jojo;
      jo_t tag = jojo[0];
      cell data = jojo[1];
      exe(tag, data);
    }
    void k_recur() {
      here(JO_INS_RECUR);

      jo_t tag = pop(current_compiling_exe_stack);
      cell data = pop(current_compiling_exe_stack);
      push(current_compiling_exe_stack, data);
      push(current_compiling_exe_stack, tag);

      here(tag);
      here(data);

      k_ignore();
    }
    void expose_control() {
      add_prim_keyword("note", k_ignore);
      add_prim("ins/lit", ins_lit);

      add_prim("ins/jmp", ins_jmp);
      add_prim("ins/jz", ins_jz);

      add_prim_keyword("if", k_if);
      add_prim_keyword("el", p_compile_until_round_ket);

      add_prim_keyword("case", k_case);

      add_prim("ins/tail-call", ins_tail_call);
      add_prim_keyword("tail-call", k_tail_call);

      add_prim("ins/loop", ins_loop);
      add_prim_keyword("loop", k_loop);

      add_prim("ins/recur", ins_recur);
      add_prim_keyword("recur", k_recur);
    }
    void k_run() {
      // (run ...)
      jo_t* jojo = tos(compiling_stack);

      push(current_compiling_exe_stack, jojo);
      push(current_compiling_exe_stack, TAG_JOJO);
      {
        compile_until_meet_jo(ROUND_KET);
        here(JO_END);
        here(0);
        here(0);
      }
      drop(current_compiling_exe_stack);
      drop(current_compiling_exe_stack);

      return_stack_push_new(jojo);
      eval();
    }
    void k_add_var() {
      jo_t name = read_jo();
      k_run();
      struct obj a = object_stack_pop();
      bind_name(name, a.tag, a.data);

      char name_buffer[1024];
      name_buffer[0] = '\0';
      strcat(name_buffer, jo2str(name));
      strcat(name_buffer, "!");
      bind_name(str2jo(name_buffer), str2jo("<set-global-variable>"), name);
    }
    #define MAX_FIELDS 1024

    void k_add_data() {
      jo_t name = read_jo();
      jo_t fields[MAX_FIELDS];
      cell i = 0;
      while (true) {
        if (i >= MAX_FIELDS) {
          k_ignore();
          report("- k_add_data fail\n");
          report("  too many fields\n");
          report("  MAX_FIELDS : %ld\n", MAX_FIELDS);
          return;
        }
        jo_t field = read_jo();
        if (field == ROUND_KET) {
          fields[i] = NULL;
          break;
        }
        fields[i] = field;
        i++;
      }
      jo_t* fresh_fields = (jo_t*)malloc(i*sizeof(jo_t));
      while (i > 0) {
        i--;
        fresh_fields[i] = fields[i];
      }
      _add_data(name, fresh_fields);
    }
    void k_add_jojo() {
      jo_t fun_name = read_jo();
      jo_t* jojo = tos(compiling_stack);

      push(current_compiling_exe_stack, jojo);
      push(current_compiling_exe_stack, TAG_JOJO);
      {
        compile_until_meet_jo(ROUND_KET);
        here(JO_END);
        here(0);
        here(0);
      }
      drop(current_compiling_exe_stack);
      drop(current_compiling_exe_stack);

      bind_name(fun_name, TAG_JOJO, jojo);
    }
    void expose_top() {
      add_prim_keyword("run", k_run);
      add_prim_keyword("+var", k_add_var);
      add_prim_keyword("+jojo", k_add_jojo);
      add_prim_keyword("+data", k_add_data);
    }
    void p_print_object_stack() {
      cell length = stack_length(object_stack);
      report("  * %ld *  ", length/2);
      report("-- ");
      cell cursor = 0;
      while (cursor < length) {
        report("%ld ", stack_ref(object_stack, cursor));
        report("%s ", jo2str(stack_ref(object_stack, cursor+1)));
        cursor++;
        cursor++;
      }
      report("--\n");
    }
    bool repl_flag = false;
    void p_repl_flag_on() { repl_flag = true; }
    void p_repl_flag_off() { repl_flag = false; }

    void p_repl() {
      while (true) {
        if (!has_jo_p()) {
          return;
        }
        jo_t s = read_jo();
        if (s == ROUND_BAR) {
          jo_apply(read_jo());
          if (repl_flag) {
            p_print_object_stack();
          }
        }
        else {
          // loop
        }
      }
    }
    void expose_repl() {

    }
    void p_true() {
      object_stack_push(TAG_BOOL, true);
    }
    void p_false() {
      object_stack_push(TAG_BOOL, false);
    }
    void p_not() {
      struct obj a = object_stack_pop();
      object_stack_push(TAG_BOOL, !a.data);
    }
    void p_and() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_BOOL, a.data && b.data);
    }
    void p_or() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_BOOL, a.data || b.data);
    }
    void expose_bool() {
      add_atom_data("<bool>", gc_ignore);

      add_prim("true", p_true);
      add_prim("false", p_false);
      add_prim("not", p_not);
      add_prim("and", p_and);
      add_prim("or", p_or);
    }
    void string_write(char* str) {
      while (str[0] != '\0') {
        write_byte(str[0]);
        str++;
      }
    }
    void p_string_write() {
      struct obj a = object_stack_pop();
      struct object_entry* ao = a.data;
      string_write(ao->pointer);
    }
    void p_string_len() {
      struct obj a = object_stack_pop();
      struct object_entry* ao = a.data;
      object_stack_push(TAG_INT, strlen(ao->pointer));
    }
    void p_string_ref() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      struct object_entry* bo = b.data;
      char* str = bo->pointer;
      object_stack_push(TAG_BYTE, str[a.data]);
    }
    void p_string_cat() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      struct object_entry* ao = a.data;
      struct object_entry* bo = b.data;
      char* str0 = bo->pointer;
      char* str1 = ao->pointer;

      char* str2 = (char*)malloc(strlen(str0) + strlen(str1) + 1);
      str2[0] = '\0';
      strcat(str2, str0);
      strcat(str2, str1);

      struct object_entry* object_entry = new_record_object_entry();
      object_entry->gc_actor = gc_free;
      object_entry->pointer = str2;

      object_stack_push(TAG_STRING, object_entry);
    }
    void p_string_slice() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      struct obj c = object_stack_pop();
      struct object_entry* co = c.data;
      char* str0 = co->pointer;
      cell begin = b.data;
      cell end = a.data;
      char* str1 = substring(str0, begin, end);

      struct object_entry* object_entry = new_record_object_entry();
      object_entry->gc_actor = gc_free;
      object_entry->pointer = str1;

      object_stack_push(TAG_STRING, object_entry);
    }
    void p_string_empty_p() {
      struct obj a = object_stack_pop();
      struct object_entry* ao = a.data;
      char* str = ao->pointer;
      object_stack_push(TAG_BOOL, str[0] == '\0');
    }
    void p_string_eq_p() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      struct object_entry* ao = a.data;
      struct object_entry* bo = b.data;
      object_stack_push(TAG_BOOL, string_equal(ao->pointer, ao->pointer));
    }
    void expose_string() {
      add_prim("string-write", p_string_write);
      add_prim("string-len", p_string_len);
      add_prim("string-ref", p_string_ref);
      add_prim("string-cat", p_string_cat);
      add_prim("string-slice", p_string_slice);
      add_prim("string-empty?", p_string_empty_p);
      add_prim("string-eq?", p_string_eq_p);
    }
    void p_inc() {
      struct obj a = object_stack_pop();
      object_stack_push(TAG_INT, a.data + 1);
    }
    void p_dec() {
      struct obj a = object_stack_pop();
      object_stack_push(TAG_INT, a.data - 1);
    }
    void p_neg() {
      struct obj a = object_stack_pop();
      object_stack_push(TAG_INT, - a.data);
    }
    void p_add() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_INT, a.data + b.data);
    }
    void p_sub() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_INT, b.data - a.data);
    }
    void p_mul() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_INT, a.data * b.data);
    }
    void p_div() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_INT, b.data / a.data);
    }
    void p_mod() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_INT, b.data % a.data);
    }
    void p_gt_p() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_BOOL, b.data > a.data);
    }
    void p_lt_p() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_BOOL, b.data < a.data);
    }
    void p_gteq_p() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_BOOL, b.data >= a.data);
    }
    void p_lteq_p() {
      struct obj a = object_stack_pop();
      struct obj b = object_stack_pop();
      object_stack_push(TAG_BOOL, b.data <= a.data);
    }
    void p_int_write() {
      char buffer [32];
      struct obj a = object_stack_pop();
      sprintf(buffer, "%ld", a.data);
      string_write(buffer);
    }
    void expose_int() {
      add_prim("inc", p_inc);
      add_prim("dec", p_dec);
      add_prim("neg", p_neg);

      add_prim("add", p_add);
      add_prim("sub", p_sub);

      add_prim("mul", p_mul);
      add_prim("div", p_div);
      add_prim("mod", p_mod);

      add_prim("gt?", p_gt_p);
      add_prim("lt?", p_lt_p);
      add_prim("gteq?", p_gteq_p);
      add_prim("lteq?", p_lteq_p);

      add_prim("int-write", p_int_write);
    }
    void gc_local_env(gc_state_t gc_state, struct object_entry* object_entry) {
      if (gc_state == GC_STATE_MARKING) {
        if (object_entry->mark == GC_MARK_USING) { return; }
        object_entry->mark = GC_MARK_USING;
        struct local* lr = object_entry->pointer;
        while (lr->name != NULL) {
            mark_one(lr->local_tag, lr->local_data);
          lr++;
        }
      }
      else if (gc_state == GC_STATE_SWEEPING) {
        free(object_entry->pointer);
      }
    }
    // caller free
    struct local* current_local_record() {
      struct ret rp = return_stack_tos();
      cell length = current_local_counter - rp.local_counter;
      cell i = 0;
      struct local* lr = (struct local*)
        malloc((length + 1) * sizeof(struct local));
      while (i < length) {
        lr[i].name       = local_record[rp.local_counter + i].name;
        lr[i].local_tag  = local_record[rp.local_counter + i].local_tag;
        lr[i].local_data = local_record[rp.local_counter + i].local_data;
        i++;
      }
      lr[i].name = NULL;
      return lr;
    }
    void p_current_local_env() {
      struct local* lr = current_local_record();
      struct object_entry* object_entry = new_record_object_entry();
      object_entry->gc_actor = gc_local_env;
      object_entry->pointer = lr;
      object_stack_push(str2jo("<local-env>"), object_entry);
    }
    void set_local_record(struct local* lr) {
      while (lr->name != NULL) {
        set_local(lr->name, lr->local_tag, lr->local_data);
        lr++;
      }
    }
    void exe_closure(struct object_entry* closure) {

      struct obj a = get_field(TAG_CLOSURE, closure, str2jo(".local-env"));
      struct object_entry* ao = a.data;
      struct local* lr = ao->pointer;

      struct obj b = get_field(TAG_CLOSURE, closure, str2jo(".jojo"));
      jo_t* jojo = b.data;

      cell local_counter = current_local_counter;
      set_local_record(lr);
      return_stack_push(jojo, local_counter);
    }
    void k_closure() {
      struct class* class = TAG_CLOSURE->data;
      struct object_entry* closure = new(class);

      here(JO_INS_JMP);
      jo_t* end_of_closure = tos(compiling_stack);
      p_compiling_stack_inc();
      jo_t* jojo = tos(compiling_stack);

      push(current_compiling_exe_stack, closure);
      push(current_compiling_exe_stack, TAG_CLOSURE);
      {
        compile_until_meet_jo(ROUND_KET);
        here(JO_END);
        here(0);
        here(0);
      }
      drop(current_compiling_exe_stack);
      drop(current_compiling_exe_stack);

      end_of_closure[0] = (jo_t*)tos(compiling_stack) - end_of_closure;

      here(JO_INS_LIT); here(TAG_JOJO); here(jojo);
      here(JO_INS_LIT); here(TAG_CLOSURE); here(closure);
      {
        here(JO_INS_SET_FIELD);
        here(str2jo(".jojo"));
      }

      here(str2jo("current-local-env"));
      here(JO_INS_LIT); here(TAG_CLOSURE); here(closure);
      {
        here(JO_INS_SET_FIELD);
        here(str2jo(".local-env"));
      }

      here(JO_INS_LIT); here(TAG_CLOSURE); here(closure);
    }
    void expose_closure() {
      add_prim("current-local-env", p_current_local_env);
      add_atom_data("<local-env>", gc_local_env);
      add_data_exe("<closure>", exe_closure, J(".local-env", ".jojo"));
      add_prim_keyword("jojo", k_closure);
    }
    void p_tcp_socket_listen() {
      // [:service <string> :backlog <int>] -> [<socket>]

      struct addrinfo hints, *servinfo, *p;
      int yes = 1;

      struct obj a = object_stack_pop();
      int backlog = a.data;

      struct obj b = object_stack_pop();
      struct object_entry* bo = b.data;
      char* service = bo->pointer;

      memset(&hints, 0, sizeof hints);
      hints.ai_family = AF_UNSPEC;
      hints.ai_socktype = SOCK_STREAM;
      hints.ai_flags = AI_PASSIVE;

      int rv = getaddrinfo(NULL, service, &hints, &servinfo);
      if (rv != 0) {
        report("- p_tcp_socket_listen fail to getaddrinfo\n");
        report("  service : %s\n", service);
        report("getaddrinfo: %s\n", gai_strerror(rv));
        p_debug();
        return;
      }

      int sockfd;
      for(p = servinfo; p != NULL; p = p->ai_next) {
        sockfd = socket(p->ai_family,
                        p->ai_socktype,
                        p->ai_protocol);
        if (sockfd == -1) { continue; }
        // ><><>< why setsockopt ?
        if (setsockopt(sockfd,
                       SOL_SOCKET,
                       SO_REUSEADDR, &yes, sizeof(int)) == -1) {
          report("- p_tcp_socket_listen fail to listen\n");
          report("  service : %s\n", service);
          perror("  setsockopt error : ");
          p_debug();
        }
        if (bind(sockfd, p->ai_addr, p->ai_addrlen) == -1) {
          close(sockfd);
          continue;
        }
        break;
      }
      freeaddrinfo(servinfo);

      if (p == NULL)  {
        report("- p_tcp_socket_listen fail to bind\n");
        report("  service : %s\n", service);
        p_debug();
      }

      if (listen(sockfd, backlog) == -1) {
        report("- p_tcp_socket_listen fail to listen\n");
        report("  service : %s\n", service);
        perror("  listen error : ");
        p_debug();
      }

      object_stack_push(str2jo("<socket>"), sockfd);
    }
    // get sockaddr, ipv4 or ipv6:
    void *get_in_addr(struct sockaddr *sa) {
      if (sa->sa_family == AF_INET) {
        return &(((struct sockaddr_in*)sa)->sin_addr);
      }
      return &(((struct sockaddr_in6*)sa)->sin6_addr);
    }
    void p_socket_accept() {
      // [:sockfd <socket>] ->
      // [:newfd <socket> :connector-address <string>]

      struct obj a = object_stack_pop();
      int sockfd = a.data;

      struct sockaddr_storage their_addr; // connector's address information
      socklen_t sin_size;
      char str[INET6_ADDRSTRLEN];

      sin_size = sizeof their_addr;
      int newfd = accept(sockfd, (struct sockaddr *)&their_addr, &sin_size);
      if (newfd == -1) {
        report("- p_socket_accept fail\n");
        perror("  accept error : ");
        return;
      }

      inet_ntop(their_addr.ss_family,
                get_in_addr((struct sockaddr *)&their_addr),
                str,
                sizeof(str));

      object_stack_push(str2jo("<socket>"), newfd);

      struct object_entry* object_entry = new_record_object_entry();
      object_entry->gc_actor = gc_free;
      object_entry->pointer = strdup(str);

      object_stack_push(TAG_STRING, object_entry);
    }
    void p_tcp_socket_connect() {
      // [:host <string> :service <string>] -> [<socket>]

      struct obj a = object_stack_pop();
      struct object_entry* ao = a.data;
      char* service = ao->pointer;

      struct obj b = object_stack_pop();
      struct object_entry* bo = b.data;
      char* host = bo->pointer;

      struct addrinfo hints, *servinfo, *p;

      memset(&hints, 0, sizeof hints);
      hints.ai_family = AF_UNSPEC;
      hints.ai_socktype = SOCK_STREAM;

      int rv = getaddrinfo(host, service, &hints, &servinfo);
      if (rv != 0) {
        report("- p_tcp_socket_connect fail to getaddrinfo\n");
        report("  host : %s\n", host);
        report("  service : %s\n", service);
        report("  getaddrinfo error : %s\n", gai_strerror(rv));
        p_debug();
        return;
      }

      int sockfd;
      for(p = servinfo; p != NULL; p = p->ai_next) {
        sockfd = socket(p->ai_family,
                        p->ai_socktype,
                        p->ai_protocol);
        if (sockfd == -1) { continue; }
        if (connect(sockfd, p->ai_addr, p->ai_addrlen) == -1) {
          close(sockfd);
          continue;
        }
        break;
      }
      freeaddrinfo(servinfo);

      if (p == NULL)  {
        report("- p_tcp_socket_connect fail to connect\n");
        report("  host : %s\n", host);
        report("  service : %s\n", service);
        p_debug();
      }

      object_stack_push(str2jo("<socket>"), sockfd);
    }
    void p_socket_send() {
      // [<socket> <string>] -> []

      struct obj a = object_stack_pop();
      struct object_entry* ao = a.data;
      char* str = ao->pointer;

      struct obj b = object_stack_pop();
      int sockfd = b.data;

      if (send(sockfd, str, strlen(str), 0) == -1) {
        report("- p_socket_send fail\n");
        perror("  send error : ");
      }
    }
    void p_socket_recv() {
      // [<socket>] -> [<string>]
      struct obj a = object_stack_pop();
      int sockfd = a.data;

      char* buf[1024];

      ssize_t real_bytes = recv(sockfd, buf, 1024-1, 0);
      if (real_bytes == -1) {
        report("- p_socket_recv fail\n");
        perror("  recv error : ");
      }

      struct object_entry* object_entry = new_record_object_entry();
      object_entry->gc_actor = gc_free;
      object_entry->pointer = strdup(buf);

      object_stack_push(TAG_STRING, object_entry);
    }
    void p_close() {
      // [:sockfd <socket>] -> []
      struct obj a = object_stack_pop();
      int sockfd = a.data;
      if (close(sockfd) == -1) {
        report("- p_close fail\n");
        perror("  close error : ");
      };
    }
    void expose_socket() {
      add_atom_data("<socket>", gc_ignore);
      add_prim("tcp-socket-listen", p_tcp_socket_listen);
      add_prim("socket-accept", p_socket_accept);
      add_prim("tcp-socket-connect", p_tcp_socket_connect);
      add_prim("socket-send", p_socket_send);
      add_prim("socket-recv", p_socket_recv);
      add_prim("close", p_close);
    }
    cell cmd_number;

    void p_cmd_number() {
      object_stack_push(TAG_INT, cmd_number);
    }
    char** cmd_string_array;

    void p_index_to_cmd_string() {
      // index -> string
      struct obj a = object_stack_pop();
      cell index = a.data;
      char* cmd_string = cmd_string_array[index];

      struct object_entry* object_entry = new_record_object_entry();
      object_entry->gc_actor = gc_free;
      object_entry->pointer = strdup(cmd_string);

      object_stack_push(TAG_STRING, object_entry);
    }
    void p_find_env_string() {
      // string -> [env-string true] or [false]
      struct obj a = object_stack_pop();
      struct object_entry* ao = a.data;
      char* var_string = ao->pointer;
      char* env_string = getenv(var_string);
      if (env_string == NULL) {
        object_stack_push(TAG_BOOL, false);
      }
      else {
        struct object_entry* object_entry = new_record_object_entry();
        object_entry->gc_actor = gc_free;
        object_entry->pointer = strdup(env_string);

        object_stack_push(TAG_STRING, object_entry);
        object_stack_push(TAG_BOOL, true);
      }
    }
    void expose_system() {
      add_prim("cmd-number", p_cmd_number);
      add_prim("index->cmd-string", p_index_to_cmd_string);
      add_prim("find-env-string", p_find_env_string);
    }
    void ccall (char* function_name, void* lib) {
      primitive_t fun = dlsym(lib, function_name);
      if (fun == NULL) {
        report("- ccall fail\n");
        report("  function_name : %s\n", function_name);
        report("  dynamic link error : %s\n", dlerror());
      };
      fun();
    }
    void erase_real_path_to_dir(char* path) {
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
          ((struct input_stack*)tos(reading_stack))->type == INPUT_STACK_TERMINAL) {
        realpath(path, real_reading_path);
        return real_reading_path;
      }
      else {
        char* proc_link_path = malloc(PATH_MAX);
        sprintf(proc_link_path,
                "/proc/self/fd/%d",
                ((struct input_stack*)tos(reading_stack))->file);
        ssize_t real_bytes = readlink(proc_link_path, real_reading_path, PATH_MAX);
        if (real_bytes == -1) {
          report("- get_real_reading_path fail to readlink\n");
          report("  proc_link_path : %s\n", proc_link_path);
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
    void k_clib_one() {
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
        report("- k_clib_one fail to open library\n");
        report("  real_read_path : %s\n", real_read_path);
        report("  dynamic link error : %s\n", dlerror());
        p_debug();
        return;
      };
      free(real_read_path);
      ccall("expose", lib);
    }
    void k_clib() {
      // (clib "..." ...)
      while (true) {
        jo_t s = read_raw_jo();
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
    void expose_cffi() {
      add_prim_keyword("+clib", k_clib);
    }
    void p1() {
      int file = open("README", O_RDWR);
      struct input_stack* t0_stack = input_stack_file(file);
      input_stack_push(t0_stack, '\n');
      input_stack_push(t0_stack, '\n');
      input_stack_push(t0_stack, '1');
      input_stack_push(t0_stack, '2');
      input_stack_push(t0_stack, '3');
      input_stack_push(t0_stack, '4');
      input_stack_push(t0_stack, '\n');
      input_stack_push(t0_stack, '\n');
      while (!input_stack_empty_p(t0_stack)) {
        report("%c", input_stack_pop(t0_stack));
      }
      input_stack_free(t0_stack);
      report("- input_stack test0 finished\n");

      struct input_stack* t1_stack = input_stack_terminal();
      while (!input_stack_empty_p(t1_stack)) {
        char byte = input_stack_pop(t1_stack);
        report("\n> %c", byte);
      }
      input_stack_free(t1_stack);
      report("- input_stack test1 finished\n");

      struct input_stack* t2_stack = input_stack_string("1234567890");
      input_stack_push(t2_stack, '\n');
      input_stack_push(t2_stack, '\n');
      input_stack_push(t2_stack, '1');
      input_stack_push(t2_stack, '2');
      input_stack_push(t2_stack, '3');
      input_stack_push(t2_stack, '4');
      input_stack_push(t2_stack, '\n');
      input_stack_push(t2_stack, '\n');
      while (!input_stack_empty_p(t2_stack)) {
        char byte = input_stack_pop(t2_stack);
        report("%c", byte);
      }
      input_stack_free(t2_stack);
      report("\n");
      report("- input_stack test2 finished\n");
    }
    void p2() {
      int file = open("k1~",
                      (O_CREAT | O_RDWR),
                      (S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH));

      struct output_stack* t0_stack = output_stack_file(file);
      output_stack_push(t0_stack, '1'); output_stack_pop(t0_stack);
      output_stack_push(t0_stack, '2');
      output_stack_push(t0_stack, '3'); output_stack_drop(t0_stack);
      output_stack_push(t0_stack, '4');
      output_stack_push(t0_stack, '\n');
      output_stack_file_flush(t0_stack);
      output_stack_push(t0_stack, '1');
      output_stack_push(t0_stack, '2'); output_stack_pop(t0_stack);
      output_stack_push(t0_stack, '3');
      output_stack_push(t0_stack, '4'); output_stack_drop(t0_stack);
      output_stack_push(t0_stack, '\n');
      output_stack_file_flush(t0_stack);
      output_stack_free(t0_stack);
      close(file);
      report("- output_stack test0 finished\n");

      struct output_stack* t1_stack = output_stack_terminal();
      output_stack_push(t1_stack, '\n');
      output_stack_push(t1_stack, '\n');
      output_stack_push(t1_stack, '1');
      output_stack_push(t1_stack, '2');
      output_stack_push(t1_stack, '3');
      output_stack_push(t1_stack, '4');
      output_stack_push(t1_stack, '\n');
      output_stack_push(t1_stack, '\n');
      output_stack_free(t1_stack);
      report("- output_stack test1 finished\n");

      struct output_stack* t2_stack = output_stack_string();
      output_stack_push(t2_stack, '1');
      output_stack_push(t2_stack, '2');
      report("- %c\n", output_stack_pop(t2_stack));
      output_stack_push(t2_stack, '3');
      output_stack_push(t2_stack, '4');
      report("- %c\n", output_stack_pop(t2_stack));
      output_stack_push(t2_stack, '\n');
      char* collected_string = output_stack_string_collect(t2_stack);
      report("- collected_string : %s\n", collected_string);
      output_stack_free(t2_stack);
      report("- output_stack test2 finished\n");
    }
    void p3() {
      report("- SIGSEGV : %ld\n", SIGSEGV);
    }
    void p4() {
      void* p = NULL;
      printf("- in c stack : %p\n", (void*)&p);
    }
    void path_load(char* path) {
      int file = open(path, O_RDONLY);
      if(file == -1) {
        report("- path_load fail : %s\n", path);
        perror("file open failed");
        return;
      }
      struct input_stack* input_stack = input_stack_file(file);
      push(reading_stack, input_stack);
      p_repl();
      drop(reading_stack);
      input_stack_free(input_stack);
      close(file);
    }
    void expose_play() {
      add_prim("p1", p1);
      add_prim("p2", p2);
      add_prim("p3", p3);
      add_prim("p4", p4);
      add_prim("print-object-stack", p_print_object_stack);
    }
    void init_system() {
      setvbuf(stdout, NULL, _IONBF, 0);
      setvbuf(stderr, NULL, _IONBF, 0);
    }
    void init_jotable() {
      bzero(jotable,
            JOTABLE_SIZE *
            sizeof(struct jotable_entry));
    }
    void init_literal_jo() {
      EMPTY_JO = str2jo("");

      TAG_PRIM         = str2jo("<prim>");
      TAG_JOJO         = str2jo("<jojo>");
      TAG_PRIM_KEYWORD = str2jo("<prim-keyword>");
      TAG_KEYWORD      = str2jo("<keyword>");
      TAG_CLOSURE      = str2jo("<closure>");
      TAG_CLASS        = str2jo("<class>");

      TAG_BOOL         = str2jo("<bool>");
      TAG_INT          = str2jo("<int>");
      TAG_BYTE         = str2jo("<byte>");
      TAG_STRING       = str2jo("<string>");
      TAG_JO           = str2jo("<jo>");

      TAG_UNINITIALISED_FIELD_PLACE_HOLDER =
        str2jo("<uninitialised-field-place-holder>");

      JO_DECLARED = str2jo("declared");

      ROUND_BAR    =   str2jo("(");
      ROUND_KET    =   str2jo(")");
      SQUARE_BAR   =   str2jo("[");
      SQUARE_KET   =   str2jo("]");
      FLOWER_BAR   =   str2jo("{");
      FLOWER_KET   =   str2jo("}");
      DOUBLE_QUOTE =   str2jo("\"");

      JO_INS_LIT       = str2jo("ins/lit");
      JO_INS_GET_LOCAL = str2jo("ins/get-local");
      JO_INS_SET_LOCAL = str2jo("ins/set-local");
      JO_INS_GET_FIELD = str2jo("ins/get-field");
      JO_INS_SET_FIELD = str2jo("ins/set-field");

      JO_INS_JMP = str2jo("ins/jmp");
      JO_INS_JZ  = str2jo("ins/jz");

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
    jo_t jojo_area[1024 * 1024];

    void init_stacks() {
      object_stack = new_stack("object_stack");
      return_stack = new_stack("return_stack");

      compiling_stack = new_stack("compiling_stack");
      push(compiling_stack, jojo_area);

      reading_stack = new_stack("reading_stack");
      push(reading_stack, input_stack_terminal());

      writing_stack = new_stack("writing_stack");
      push(writing_stack, output_stack_terminal());

      // binding_filter_stack = new_stack("binding_filter_stack");
      keyword_stack = new_stack("keyword_stack");

      // jo_filter_stack = new_stack("jo_filter_stack");
      // push(jo_filter_stack, str2jo("alias-filter"));

      current_compiling_exe_stack = new_stack("current_compiling_exe_stack");
    }
    void init_expose() {
      expose_object();
      expose_stack();
      expose_ending();
      expose_rw();
      expose_local();
      expose_compiler();
      expose_control();
      expose_top();
      expose_bool();
      expose_string();
      expose_int();
      expose_closure();
      expose_socket();
      expose_system();
      expose_play();
    }
    int main(int argc, char** argv) {
      cmd_number = argc;
      cmd_string_array = argv;
      init_system();
      init_jotable();
      init_literal_jo();
      init_stacks();
      init_expose();
      p_repl_flag_on();
      {
        p_print_object_stack();
      }
      p_repl();
    }
