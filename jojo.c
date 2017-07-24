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
  typedef enum { false, true } bool;
  // typedef intptr_t cell;
  typedef intmax_t cell;
  typedef uint8_t byte;
  typedef void (* primitive)();
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
    report(char* format, ...) {
      va_list arg_list;
      va_start(arg_list, format);
      vdprintf(STDERR_FILENO, format, arg_list);
      va_end(arg_list);
    }
    #define S0 (char*[]){NULL}
    #define S1(x1) (char*[]){x1, NULL}
    #define S2(x1, x2) (char*[]){x1, x2, NULL}
    #define S3(x1, x2, x3) (char*[]){x1, x2, x3, NULL}
    #define S4(x1, x2, x3, x4) (char*[]){x1, x2, x3, x4, NULL}
    #define S5(x1, x2, x3, x4, x5) (char*[]){x1, x2, x3, x4, x5, NULL}
    #define S6(x1, x2, x3, x4, x5, x6) (char*[]){x1, x2, x3, x4, x5, x6, NULL}
    #define S7(x1, x2, x3, x4, x5, x6, x7) (char*[]){x1, x2, x3, x4, x5, x6, x7, NULL}
    #define S8(x1, x2, x3, x4, x5, x6, x7, x8) (char*[]){x1, x2, x3, x4, x5, x6, x7, x8, NULL}
    #define S9(x1, x2, x3, x4, x5, x6, x7, x8, x9) (char*[]){x1, x2, x3, x4, x5, x6, x7, x8, x9, NULL}
  p_debug() {
  }
    struct jotable_entry {
      char *key;
      struct jotable_entry *tag;
      cell data;
    };

    typedef struct jotable_entry *jo;

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
      return (counter + sum) % JOTABLE_SIZE;
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
    // #define STACK_BLOCK_SIZE 1 // for test
    struct stack* new_stack(char* name) {
      struct stack* stack = (struct stack*)malloc(sizeof(struct stack));
      stack->name = name;
      stack->pointer = 0;
      stack->stack = (cell*)malloc(sizeof(cell) * STACK_BLOCK_SIZE);
      stack->link = NULL;
      return stack;
    }
    stack_free_link(struct stack_link* link) {
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
    stack_free(struct stack* stack) {
      stack_free_link(stack->link);
      free(stack->stack);
      free(stack);
    }
    // can not pop
    // for stack->pointer can not decrease under 0
    stack_block_underflow_check(struct stack* stack) {
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
    stack_block_overflow_check(struct stack* stack) {
      if (stack->pointer < STACK_BLOCK_SIZE) {
        return;
      }
      else {
        struct stack_link* new_link = (struct stack_link*)malloc(sizeof(struct stack_link));
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
    drop(struct stack* stack) {
      stack_block_underflow_check(stack);
      stack->pointer--;
    }
    push(struct stack* stack, cell data) {
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
       report("<stack_peek> index : %ld\n", index);
      if (index < stack->pointer) {

        return stack->stack[stack->pointer - index];
      }
      else {
        return stack_peek_link(stack->link, index - stack->pointer);
      }
    }
    stack_traverse_from_top_help
    (cell cursor,
     cell* stack,
     struct stack_link* link,
     void fun(cell)) {
      while (cursor > 0) {
        fun(stack[cursor - 1]);
        cursor--;
      }
      if (link != NULL) {
        stack_traverse_from_top_help
          (STACK_BLOCK_SIZE,
           link->stack,
           link->link,
           fun);
      }
    }

    stack_traverse_from_top(struct stack* stack, void fun(cell)) {
      stack_traverse_from_top_help
        (stack->pointer,
         stack->stack,
         stack->link,
         fun);
    }
    stack_traverse_from_bottom_help
    (cell cursor,
     cell* stack,
     struct stack_link* link,
     void fun(cell)) {
      if (link != NULL) {
        stack_traverse_from_bottom_help
          (STACK_BLOCK_SIZE,
           link->stack,
           link->link,
           fun);
      }
      cell i = 0;
      while (i < cursor) {
        fun(stack[i]);
        i++;
      }
    }

    stack_traverse_from_bottom(struct stack* stack, void fun(cell)) {
      stack_traverse_from_bottom_help
        (stack->pointer,
         stack->stack,
         stack->link,
         fun);
    }
    typedef enum {
      INPUT_STACK_REGULAR_FILE,
      INPUT_STACK_STRING,
      INPUT_STACK_TERMINAL,
    } input_stack_type;
    struct input_stack_link {
      byte* stack;
      cell end_pointer;
      struct input_stack_link* link;
    };
    struct input_stack {
      cell pointer;
      cell end_pointer;
      byte* stack;
      struct input_stack_link* link;
      input_stack_type type;
      union {
        int   file;
        char* string;
        // int   terminal;
      };
      cell string_pointer;
    };

    // #define INPUT_STACK_BLOCK_SIZE (4 * 1024)
    #define INPUT_STACK_BLOCK_SIZE 1 // for test
    struct input_stack* input_stack_new(input_stack_type input_stack_type) {
      struct input_stack* input_stack =
        (struct input_stack*)malloc(sizeof(struct input_stack));
      input_stack->pointer = INPUT_STACK_BLOCK_SIZE;
      input_stack->end_pointer = INPUT_STACK_BLOCK_SIZE;
      input_stack->stack = (byte*)malloc(INPUT_STACK_BLOCK_SIZE);
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
    input_stack_free_link(struct input_stack_link* link) {
      if (link == NULL) {
        return;
      }
      else {
        input_stack_free_link(link->link);
        free(link->stack);
        free(link);
      }
    }

    input_stack_free(struct input_stack* input_stack) {
      input_stack_free_link(input_stack->link);
      free(input_stack->stack);
      free(input_stack);
    }
    // can not pop
    // for input_stack->pointer can not increase over input_stack->end_pointer
    input_stack_block_underflow_check(struct input_stack* input_stack) {
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
        byte byte = input_stack->string[input_stack->string_pointer];
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
    input_stack_block_overflow_check(struct input_stack* input_stack) {
      if (input_stack->pointer > 0) {
        return;
      }
      else {
        struct input_stack_link* new_link =
          (struct input_stack_link*)malloc(sizeof(struct input_stack_link));
        new_link->stack = input_stack->stack;
        new_link->link = input_stack->link;
        new_link->end_pointer = input_stack->end_pointer;
        input_stack->link = new_link;
        input_stack->stack = (byte*)malloc(INPUT_STACK_BLOCK_SIZE);
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
      else if (input_stack->type == INPUT_STACK_TERMINAL) {
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
      else {
        report("- input_stack_empty_p meet unknow stack type\n");
        report("  stack type number : %ld\n", input_stack->type);
        p_debug();
      }
    }
    byte input_stack_pop(struct input_stack* input_stack) {
      input_stack_block_underflow_check(input_stack);
      byte byte = input_stack->stack[input_stack->pointer];
      input_stack->pointer++;
      return byte;
    }
    byte input_stack_tos(struct input_stack* input_stack) {
      input_stack_block_underflow_check(input_stack);
      byte byte = input_stack->stack[input_stack->pointer];
      return byte;
    }
    input_stack_drop(struct input_stack* input_stack) {
      input_stack_block_underflow_check(input_stack);
      input_stack->pointer++;
    }
    input_stack_push(struct input_stack* input_stack, byte byte) {
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
      byte* stack;
      struct output_stack_link* link;
    };
    struct output_stack {
      cell pointer;
      byte* stack;
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

    // #define OUTPUT_STACK_BLOCK_SIZE (4 * 1024)
    #define OUTPUT_STACK_BLOCK_SIZE 1 // for test
    struct output_stack* output_stack_new(output_stack_type output_stack_type) {
      struct output_stack* output_stack =
        (struct output_stack*)malloc(sizeof(struct output_stack));
      output_stack->pointer = 0;
      output_stack->stack = (byte*)malloc(OUTPUT_STACK_BLOCK_SIZE);
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
    output_stack_free_link(struct output_stack_link* link) {
      if (link == NULL) {
        return;
      }
      else {
        output_stack_free_link(link->link);
        free(link->stack);
        free(link);
      }
    }

    output_stack_free(struct output_stack* output_stack) {
      output_stack_free_link(output_stack->link);
      free(output_stack->stack);
      free(output_stack);
    }
    output_stack_file_flush_link(int file, struct output_stack_link* link) {
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

    output_stack_file_flush(struct output_stack* output_stack) {
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


    byte* output_stack_string_collect_link(byte* buffer, struct output_stack_link* link) {
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
      byte* string = (byte*)malloc(1 + output_stack_string_length(output_stack));
      byte* buffer = string;
      buffer = output_stack_string_collect_link(buffer, output_stack->link);
      memcpy(buffer, output_stack->stack, output_stack->pointer);
      buffer[output_stack->pointer] = '\0';
      return string;
    }
    // can not pop
    // for output_stack->pointer can not decrease under 0
    output_stack_block_underflow_check(struct output_stack* output_stack) {
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
    output_stack_block_overflow_check(struct output_stack* output_stack) {
      if (output_stack->pointer < OUTPUT_STACK_BLOCK_SIZE) {
        return;
      }
      else {
        struct output_stack_link* new_link =
          (struct output_stack_link*)malloc(sizeof(struct output_stack_link));
        new_link->stack = output_stack->stack;
        new_link->link = output_stack->link;
        output_stack->link = new_link;
        output_stack->stack = (byte*)malloc(OUTPUT_STACK_BLOCK_SIZE);
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
      else if (output_stack->type == OUTPUT_STACK_TERMINAL) {
        return true;
      }
      else {
        report("- output_stack_empty_p meet unknow stack type\n");
        report("  stack type number : %ld\n", output_stack->type);
        p_debug();
      }
    }
    byte output_stack_pop(struct output_stack* output_stack) {
      output_stack_block_underflow_check(output_stack);
      output_stack->pointer--;
      return output_stack->stack[output_stack->pointer];
    }
    byte output_stack_tos(struct output_stack* output_stack) {
      output_stack_block_underflow_check(output_stack);
      return output_stack->stack[output_stack->pointer - 1];
    }
    output_stack_drop(struct output_stack* output_stack) {
      output_stack_block_underflow_check(output_stack);
      output_stack->pointer--;
    }
    output_stack_push(struct output_stack* output_stack, byte b) {
      if (output_stack->type == OUTPUT_STACK_TERMINAL) {
        byte buffer[1];
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
    struct object {
      jo tag;
      cell data;
    };

    struct stack* object_stack;

    struct object object_stack_pop() {
      struct object p;
      p.tag = pop(object_stack);
      p.data = pop(object_stack);
      return p;
    }

    struct object object_stack_tos() {
      struct object p;
      p.tag = pop(object_stack);
      p.data = pop(object_stack);
      push(object_stack, p.data);
      push(object_stack, p.tag);
      return p;
    }

    bool object_stack_empty_p() {
      return stack_empty_p(object_stack);
    }

    object_stack_push(jo tag, cell data) {
      push(object_stack, data);
      push(object_stack, tag);
    }

    jo object_stack_peek_tag(cell index) {
      return stack_peek(object_stack, (index*2) - 1);
    }

    cell object_stack_peek_data(cell index) {
      return stack_peek(object_stack, (index*2));
    }
    struct local {
      jo name;
      cell local_tag;
      cell local_data;
    };

    struct local local_record[4 * 1024];
    cell current_local_pointer = 0;
    struct ret {
      jo* jojo;
      cell local_pointer;
    };

    struct stack* return_stack;

    struct ret return_stack_pop() {
      struct ret p;
      p.jojo = pop(return_stack);
      p.local_pointer = pop(return_stack);
      return p;
    }

    struct ret return_stack_tos() {
      struct ret p;
      p.jojo = pop(return_stack);
      p.local_pointer = pop(return_stack);
      push(return_stack, p.local_pointer);
      push(return_stack, p.jojo);
      return p;
    }

    bool return_stack_empty_p() {
      return stack_empty_p(return_stack);
    }

    return_stack_push(jo* jojo, cell local_pointer) {
      push(return_stack, local_pointer);
      push(return_stack, jojo);
    }

    return_stack_push_new(jo* jojo) {
      return_stack_push(jojo, current_local_pointer);
    }

    return_stack_inc() {
      jo* jojo = pop(return_stack);
      push(return_stack, jojo + 1);
    }
    struct stack* compiling_stack; // of jojo

    p_compiling_stack_inc() {
      jo* jojo = pop(compiling_stack);
      push(compiling_stack, jojo + 1);
    }
    here(cell n) {
      jo* jojo = pop(compiling_stack);
      jojo[0] = n;
      push(compiling_stack, jojo + 1);
    }
    typedef enum {
      GC_IGNORE,
      GC_CLEANER,
      GC_RECUR,
    } gc_type;
    // typedef void (* cleaner__t)(cell);

    struct class {
      jo class_name;
      jo super_name;
      gc_type gc_type;
      // cleaner__t cleaner;
      cell object_size;
    };
    define_atom_class(char* class_name,
                      gc_type gc_type) {
      struct class* class = (struct class*)malloc(sizeof(struct class));
      class->class_name = str2jo(class_name);
      class->super_name = str2jo("<object>");
      class->gc_type = gc_type;

      jo name = str2jo(class_name);
      name->tag = str2jo("<class>");
      name->data = class;
    }
    define_field(char* class_name, char* field, cell index) {
      char name_buffer[1024];
      jo name;
      strcpy(name_buffer+1, class_name);
      strcpy(name_buffer+1+strlen(class_name), field);

      name_buffer[0] = '.';
      name = str2jo(name_buffer);
      name->tag = ("<get-object-field>");
      name->data = index;

      name_buffer[0] = '!';
      name = str2jo(name_buffer);
      name->tag = ("<set-object-field>");
      name->data = index;
    }
    define_class(char* class_name,
                 char* super_name,
                 char* fields[]) {
      struct class* class = (struct class*)malloc(sizeof(struct class));
      jo name = str2jo(class_name);
      jo super = str2jo(super_name);
      class->class_name = name;
      class->super_name = super;
      class->gc_type = GC_RECUR;
      struct class* super_class = super->data;
      cell i = super_class->object_size;
      while (fields[i] != NULL) {
        define_field(class_name, fields[i], i);
        i++;
      }
      class->object_size = i;

      name->tag = str2jo("<class>");
      name->data = class;
    }
    init_class() {
      struct class* class = (struct class*)malloc(sizeof(struct class));
      jo name = str2jo("<object>");
      class->class_name = name;
      class->gc_type = GC_RECUR;
      class->object_size = 0;
      name->tag = str2jo("<class>");
      name->data = class;
    }
    struct object_record_entry {
      cell mark;
      jo tag;
      cell pointer;
    };

    struct object_record_entry object_record[1024 * 1024];
    mark_object_record(jo tag, cell pointer) {

    }
    sweep_object_record() {

    }
    new(struct class* class) {
      cell* object = (cell*)malloc(class->object_size*2*sizeof(cell));
      object_stack_push(class->class_name, object);
    }
    bool check_function_arity(char* function_name, cell arity) {
      jo name = str2jo(function_name);
      if (used_jo_p(name)) {
        return name->tag == str2jo("<generic-prototype>") && name->data == arity;
      }
      else {
        name->tag = str2jo("<generic-prototype>");
        name->data = arity;
        return true;
      }
    }
    define_prim(char* function_name,
                char* tags[],
                primitive fun) {
      char name_buffer[1024];
      char* cursor = name_buffer;
      cell i = 0;
      while (tags[i] != NULL) {
        strcpy(cursor, tags[i]);
        cursor = cursor + strlen(tags[i]);
        i++;
      }
      strcpy(cursor, function_name);
      jo name = str2jo(name_buffer);

      report("<define_prim> name_buffer : %s\n", name_buffer);

      cell arity = i;
      if (arity == 0) {
        name->tag = TAG_PRIM;
        name->data = fun;
      }
      else if (check_function_arity(function_name, arity)) {
        name->tag = TAG_PRIM;
        name->data = fun;
      }
      else {
        report("- define_prim fall\n");
        report("  arity of %s should not be %ld\n", function_name, arity);
      }
    }
    struct stack* keyword_stack; // of alias_pointer
    struct alias {
      jo nick;
      jo name;
    };

    struct alias alias_record[1024];
    cell current_alias_pointer = 0;
    jo object_tag(cell* o, cell index) {
      return o[index*2];
    }
    jo object_data(cell* o, cell index) {
      return o[index*2+1];
    }
    jo jo2real_jo(jo j) {
      cell arity = j->data;
      char name_buffer[1024];
      char* cursor = name_buffer;
      cell i = arity;
      jo tag;
      while (i > 0) {
        tag = object_stack_peek_tag(i);
        strcpy(cursor, jo2str(tag));
        cursor = cursor + strlen(jo2str(tag));
        i--;
      }
      strcpy(cursor, jo2str(j));
      report("<jo2real_jo> name_buffer : %s\n", name_buffer);
      return str2jo(name_buffer);
    }
    generic_apply(jo jo) {
      jo = jo2real_jo(jo);
      cell tag = jo->tag;
      if (tag == TAG_PRIM) {
        report("<generic_apply> primitive\n");
        primitive primitive = jo->data;
        primitive();
      }
      else if (tag == TAG_JOJO) {
        cell jojo = jo->data;
        return_stack_push_new(jojo);
      }
      else if (tag == TAG_PRIM_KEYWORD) {
        push(keyword_stack, current_alias_pointer);
        primitive primitive = jo->data;
        primitive();
        current_alias_pointer = pop(keyword_stack);
      }
      else if (tag == TAG_KEYWORD) {
        // keywords are always evaled
        push(keyword_stack, current_alias_pointer);
        cell jojo = jo->data;
        return_stack_push_new(jojo);
        eval();
        current_alias_pointer = pop(keyword_stack);
      }
      else if (tag == str2jo("<get-object-field>")) {
        cell index = jo->data;
        struct object p = object_stack_pop();
        object_stack_push(object_tag(p.data, index),
                          object_data(p.data, index));
      }
      else if (tag == str2jo("<set-object-field>")) {
        cell index = jo->data;
        //
      }
      else {
        report("- generic_apply meet unknown tag : %s\n", jo2str(tag));
      }
    }
    p_debug();

    jo_apply(jo jo) {
      if (!used_jo_p(jo)) {
        report("- jo_apply meet undefined jo : %s\n", jo2str(jo));
        p_debug();
        return;
      }
      if (jo->tag == str2jo("<generic-prototype>")) {
        report("<jo_apply> jo : %s\n", jo2str(jo));
        generic_apply(jo);
      }
      else if (jo->tag == TAG_PRIM) {
        primitive primitive = jo->data;
        primitive();
      }
      else {
        push(object_stack, jo->data);
        push(object_stack, jo->tag);
      }
    }
    eval() {
      cell base = return_stack->pointer;
      while (return_stack->pointer >= base) {
        struct ret rp = return_stack_tos();
        return_stack_inc();
        jo* jojo = rp.jojo;
        jo jo = jojo[0];
        jo_apply(jo);
      }
    }
    p_drop() {
      object_stack_pop();
    }
    p_dup() {
      struct object a = object_stack_pop();
      object_stack_push(a.tag, a.data);
      object_stack_push(a.tag, a.data);
    }
    p_over() {
      // b a -> b a b
      struct object a = object_stack_pop();
      struct object b = object_stack_pop();
      object_stack_push(b.tag, b.data);
      object_stack_push(a.tag, a.data);
      object_stack_push(b.tag, b.data);
    }
    p_tuck() {
      // b a -> a b a
      struct object a = object_stack_pop();
      struct object b = object_stack_pop();
      object_stack_push(a.tag, a.data);
      object_stack_push(b.tag, b.data);
      object_stack_push(a.tag, a.data);
    }
    p_swap() {
      // b a -> a b
      struct object a = object_stack_pop();
      struct object b = object_stack_pop();
      object_stack_push(a.tag, a.data);
      object_stack_push(b.tag, b.data);
    }
    expose_stack_operation() {
      define_prim("drop", S1("<object>"), p_drop);
      define_prim("dup", S1("<object>"), p_dup);
      define_prim("over", S2("<object>", "<object>"), p_over);
      define_prim("tuck", S2("<object>", "<object>"), p_tuck);
      define_prim("swap", S2("<object>", "<object>"), p_swap);
    }
    p_end() {
      struct ret rp = return_stack_pop();
      current_local_pointer = rp.local_pointer;
    }
    p_bye() {
      report("bye bye ^-^/\n");
      exit(0);
    }
    expose_ending() {
      define_prim("end", S0, p_end);
      define_prim("bye", S0, p_bye);
    }
    p1() {
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
        byte byte = input_stack_pop(t1_stack);
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
        byte byte = input_stack_pop(t2_stack);
        report("%c", byte);
      }
      input_stack_free(t2_stack);
      report("\n");
      report("- input_stack test2 finished\n");
    }
    p2() {
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
    p3() {
      define_class("<rectangle>", "<object>", S3("height", "width", "k1"));
    }
    print_object_stack_data(cell data) {
      report("%ld ", data);
    }

    p4() {
      report("<p4>\n");
      report("  * %ld *  ", stack_length(object_stack));
      report("-- ");
      stack_traverse_from_bottom(object_stack, print_object_stack_data);
      report("--\n");
    }
    init_play() {
      p3();
    }
    expose_play() {
      define_prim("p1", S0, p1);
      define_prim("p2", S0, p2);
      define_prim("p3", S0, p3);
      define_prim("p4", S0, p4);
    }
    init_system() {
      setvbuf(stdout, NULL, _IONBF, 0);
      setvbuf(stderr, NULL, _IONBF, 0);
    }
    init_jotable() {
      bzero(jotable, JOTABLE_SIZE * sizeof(struct jotable_entry));
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
    jo jojo_area[1024 * 1024];

    init_stacks() {
      object_stack                 = new_stack("object_stack");
      return_stack                 = new_stack("return_stack");
      compiling_stack              = new_stack("compiling_stack");
      // reading_stack                = new_stack("reading_stack");
      // writing_stack                = new_stack("writing_stack");
      // binding_filter_stack         = new_stack("binding_filter_stack");
      keyword_stack                = new_stack("keyword_stack");
      // jo_filter_stack              = new_stack("jo_filter_stack");
      // current_compiling_jojo_stack = new_stack("current_compiling_jojo_stack");

      push(compiling_stack, jojo_area);
      // push(reading_stack, input_stack_terminal());
      // push(writing_stack, output_stack_terminal());
      // push(jo_filter_stack, str2jo("alias-filter"));
    }
    init_jojo() {
      init_jotable();
      init_literal_jo();
      init_stacks();
      init_class();

      expose_stack_operation();
      expose_ending();

      expose_play();
    }
  int main(int argc, char** argv) {
    // cmd_number = argc;
    // cmd_string_array = argv;
    init_system();
    init_jojo();
    init_play();
    {
      object_stack_push(str2jo("<object>"), 666);

      here(str2jo("dup"));
      here(str2jo("dup"));
      here(str2jo("swap"));
      here(str2jo("p4"));
      here(str2jo("p1"));
      here(str2jo("p2"));
      here(str2jo("end"));
      return_stack_push_new(jojo_area);
      eval();
    }
    return 66;
  }
