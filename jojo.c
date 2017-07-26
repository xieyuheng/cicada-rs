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
    report(char* format, ...) {
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
      cell string_count_member(char* s, byte b) {
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
      bool string_member_p(char* s, byte b) {
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
        if ((end != NULL && end > len) ||
            (begin < 0)) {
          report("- substring fail\n");
          report("  string : %s\n", str);
          report("  length : %ld\n", len);
          report("  begin index : %ld\n", begin);
          report("  end index : %ld\n", end);
          return;
        }

        char* buf = strdup(str);
        if (end != NULL) {
          buf[end] = '\0';
        }
        if (begin == 0) {
          return buf;
        }
        else {
          char* s = strdup(buf+begin);
          free(buf);
          return s;
        }
      }
    #define S0 (char*[]){NULL}
    #define S(...) (char*[]){__VA_ARGS__, NULL}
  p_debug() {
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
    jo_t TAG_INT;
    jo_t TAG_STRING;

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

    jo_t JO_INS_JUMP;
    jo_t JO_INS_JUMP_IF_FALSE;

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
    report_name_record() {
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
    bind_name(name, tag, data)
      jo_t name;
      jo_t tag;
      cell data;
    {
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
    rebind_name(name, tag, data)
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
      jo_t tag;
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

    object_stack_push(jo_t tag, cell data) {
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

    struct local local_record[4 * 1024];
    cell current_local_pointer = 0;
    struct ret {
      jo_t* jojo;
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

    return_stack_push(jo_t* jojo, cell local_pointer) {
      push(return_stack, local_pointer);
      push(return_stack, jojo);
    }

    return_stack_push_new(jo_t* jojo) {
      return_stack_push(jojo, current_local_pointer);
    }

    return_stack_inc() {
      jo_t* jojo = pop(return_stack);
      push(return_stack, jojo + 1);
    }
    struct stack* compiling_stack; // of jojo

    p_compiling_stack_inc() {
      jo_t* jojo = pop(compiling_stack);
      push(compiling_stack, jojo + 1);
    }
    here(cell n) {
      jo_t* jojo = pop(compiling_stack);
      jojo[0] = n;
      push(compiling_stack, jojo + 1);
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
      cell object_size;
      cell pointer;
    };

    // #define OBJECT_RECORD_SIZE 64 * 1024
    #define OBJECT_RECORD_SIZE 3
    struct object_entry object_record[OBJECT_RECORD_SIZE];

    struct object_entry* object_record_pointer = object_record;
    bool object_record_end_p() {
      return object_record_pointer >= (object_record + OBJECT_RECORD_SIZE);
    }
    init_object_record() {
      bzero(object_record,
            OBJECT_RECORD_SIZE *
            sizeof(struct object_entry));
    }
    typedef void (* executer_t)(cell);
    struct class {
      jo_t class_name;
      jo_t super_name;
      gc_actor_t gc_actor;
      bool executable;
      executer_t executer;
      cell object_size; // unit cell*2
    };
    jo_t get_field_tag(cell* fields, cell field_index) {
      return fields[field_index*2+1];
    }

    set_field_tag(cell* fields, cell field_index, jo_t tag) {
      fields[field_index*2+1] = tag;
    }

    cell get_field_data(cell* fields, cell field_index) {
      return fields[field_index*2];
    }

    set_field_data(cell* fields, cell field_index, cell data) {
      fields[field_index*2] = data;
    }
    jo_t get_object_field_tag(object_entry, field_index)
      struct object_entry* object_entry;
      cell field_index;
    {
      cell* fields = object_entry->pointer;
      return get_field_tag(fields, field_index);
    }

    set_object_field_tag(object_entry, field_index, tag)
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

    set_object_field_data(object_entry, field_index, data)
      struct object_entry* object_entry;
      cell field_index;
      cell data;
    {
      cell* fields = object_entry->pointer;
      set_field_data(fields, field_index, data);
    }
    mark_one(jo_t tag, cell data) {
      struct class* class = tag->data;
      class->gc_actor(GC_STATE_MARKING, data);
    }
    mark_object_record() {
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
    sweep_one(struct object_entry* object_entry) {
      if (object_entry->mark == GC_MARK_USING) {
        return;
      }
      else {
        object_entry->gc_actor(GC_STATE_SWEEPING, object_entry);
      }
    }
    sweep_object_record() {
      cell i = 0;
      while (i < OBJECT_RECORD_SIZE) {
        sweep_one(object_record + i);
        i++;
      }
    }
      gc_ignore(gc_state_t gc_state, cell data) {
        if (gc_state == GC_STATE_MARKING) {
        }
        else if (gc_state == GC_STATE_SWEEPING) {
        }
      }
      gc_free(gc_state_t gc_state, struct object_entry* object_entry) {
        if (gc_state == GC_STATE_MARKING) {
          object_entry->mark = GC_MARK_USING;
        }
        else if (gc_state == GC_STATE_SWEEPING) {
          free(object_entry->pointer);
        }
      }
      gc_recur(gc_state_t gc_state, struct object_entry* object_entry) {
        if (gc_state == GC_STATE_MARKING) {
          if (object_entry->mark == GC_MARK_USING) {
            return;
          }
          object_entry->mark = GC_MARK_USING;
          cell object_size = object_entry->object_size;
          cell* fields = object_entry->pointer;
          cell i = 0;
          while (i < object_size) {
            mark_one(get_object_field_tag(object_entry, i),
                     get_object_field_data(object_entry, i));
            i++;
          }
        }
        else if (gc_state == GC_STATE_SWEEPING) {
          free(object_entry->pointer);
        }
      }
    run_gc() {
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
    next_free_record_object_entry() {
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
        if (next_free_record_object_entry() < OBJECT_RECORD_SIZE) {
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
      struct object_entry* object_entry =
        (struct object_entry*)malloc(sizeof(struct object_entry));
      return object_entry;
    }
    p_new() {
      // [<class>] -> [<object> of <class>]
      struct object a = object_stack_pop();
      struct class* class = a.data;
      cell* fields = (cell*)malloc(class->object_size*2*sizeof(cell));

      cell i = 0;
      while (i < class->object_size) {
        set_field_tag(fields, i, str2jo("<uninitialised-field-place-holder>"));
        // set_field_data(fields, i, 0);
        i++;
      }

      struct object_entry* object_entry = new_record_object_entry();
      object_entry->gc_actor = gc_recur;
      object_entry->pointer = fields;
      object_entry->object_size = class->object_size;

      object_stack_push(class->class_name, object_entry);
    }
      add_atom_class(class_name, gc_actor)
        char* class_name;
        gc_actor_t gc_actor;
      {
        struct class* class = (struct class*)malloc(sizeof(struct class));
        class->class_name = str2jo(class_name);
        class->super_name = str2jo("<object>");
        class->gc_actor = gc_actor;
        class->executable = false;

        jo_t name = str2jo(class_name);
        bind_name(name, str2jo("<class>"), class);
      }
      add_executable_atom_class(class_name, gc_actor, executer)
        char* class_name;
        gc_actor_t gc_actor;
        executer_t executer;
      {
        struct class* class = (struct class*)malloc(sizeof(struct class));
        class->class_name = str2jo(class_name);
        class->super_name = str2jo("<object>");
        class->gc_actor = gc_actor;
        class->executable = true;
        class->executer = executer;

        jo_t name = str2jo(class_name);
        bind_name(name, str2jo("<class>"), class);
      }
      bool check_function_arity(char* function_name, cell arity) {
        jo_t name = str2jo(function_name);
        if (used_jo_p(name)) {
          return name->tag == str2jo("<generic-prototype>") && name->data == arity;
        }
        else {
          bind_name(name, str2jo("<generic-prototype>"), arity);
          return true;
        }
      }
      add_field(class_name, field_name, index)
        char* class_name;
        char* field_name;
        cell index;
      {
        char name_buffer[1024];

        name_buffer[0] = '\0';
        strcat(name_buffer, ".");
        strcat(name_buffer, field_name);
        if (check_function_arity(name_buffer, 1)) {
          name_buffer[0] = '\0';
          strcat(name_buffer, class_name);
          strcat(name_buffer, ".");
          strcat(name_buffer, field_name);
          bind_name(str2jo(name_buffer), str2jo("<get-object-field>"), index);
        }
        else {
          report("- add_field fail\n");
          return;
        }

        name_buffer[0] = '\0';
        strcat(name_buffer, ".");
        strcat(name_buffer, field_name);
        strcat(name_buffer, "!");
        if (check_function_arity(name_buffer, 2)) {
          name_buffer[0] = '\0';
          strcat(name_buffer, "<object>");
          strcat(name_buffer, class_name);
          strcat(name_buffer, ".");
          strcat(name_buffer, field_name);
          strcat(name_buffer, "!");
          bind_name(str2jo(name_buffer), str2jo("<set-object-field>"), index);
        }
        else {
          report("- add_field fail\n");
          return;
        }
      }
      _add_class(name, super, fields)
        jo_t name;
        jo_t super;
        jo_t fields[];
      {
        struct class* class = (struct class*)malloc(sizeof(struct class));
        class->class_name = name;
        class->super_name = super;
        class->gc_actor = gc_recur;
        class->executable = false;
        struct class* super_class = super->data;
        cell i = 0;
        while (fields[i] != NULL) {
          add_field(jo2str(name),
                    jo2str(fields[i]),
                    super_class->object_size + i);
          i++;
        }
        class->object_size = super_class->object_size + i;

        bind_name(name, str2jo("<class>"), class);
      }
      add_class(class_name, super_name, fields)
        char* class_name;
        char* super_name;
        char* fields[];
      {
        struct class* class = (struct class*)malloc(sizeof(struct class));
        jo_t name = str2jo(class_name);
        jo_t super = str2jo(super_name);
        class->class_name = name;
        class->super_name = super;
        class->gc_actor = gc_recur;
        class->executable = false;
        struct class* super_class = super->data;
        cell i = 0;
        while (fields[i] != NULL) {
          add_field(class_name, fields[i], super_class->object_size + i);
          i++;
        }
        class->object_size = super_class->object_size + i;

        bind_name(name, str2jo("<class>"), class);
      }
      add_executable_class(class_name, super_name, executer, fields)
        char* class_name;
        char* super_name;
        executer_t executer;
        char* fields[];
      {
        struct class* class = (struct class*)malloc(sizeof(struct class));
        jo_t name = str2jo(class_name);
        jo_t super = str2jo(super_name);
        class->class_name = name;
        class->super_name = super;
        class->gc_actor = gc_recur;
        class->executable = true;
        class->executer = executer;
        struct class* super_class = super->data;
        cell i = super_class->object_size;
        while (fields[i] != NULL) {
          add_field(class_name, fields[i], i);
          i++;
        }
        class->object_size = i;

        bind_name(name, str2jo("<class>"), class);
      }
      add_prim(function_name, tags, fun)
        char* function_name;
        char* tags[];
        primitive_t fun;
      {
        char name_buffer[1024];
        char* cursor = name_buffer;
        cell i = 0;
        while (tags[i] != NULL) {
          strcpy(cursor, tags[i]);
          cursor = cursor + strlen(tags[i]);
          i++;
        }
        strcpy(cursor, function_name);

        jo_t name = str2jo(name_buffer);

        cell arity = i;
        if (arity == 0 ||
            check_function_arity(function_name, arity)) {
          bind_name(name, TAG_PRIM, fun);
        }
        else {
          report("- add_prim fall\n");
          report("  arity of %s should not be %ld\n", function_name, arity);
        }
      }
      add_prim_keyword(function_name, tags, fun)
        char* function_name;
        char* tags[];
        primitive_t fun;
      {
        char name_buffer[1024];
        char* cursor = name_buffer;
        cell i = 0;
        while (tags[i] != NULL) {
          strcpy(cursor, tags[i]);
          cursor = cursor + strlen(tags[i]);
          i++;
        }
        strcpy(cursor, function_name);
        jo_t name = str2jo(name_buffer);
        cell arity = i;
        if (arity == 0 ||
            check_function_arity(function_name, arity)) {
          bind_name(name, TAG_PRIM_KEYWORD, fun);
        }
        else {
          report("- add_prim_keyword fall\n");
          report("  arity of %s should not be %ld\n", function_name, arity);
        }
      }
    struct stack* keyword_stack; // of alias_pointer
    struct alias {
      jo_t nick;
      jo_t name;
    };

    struct alias alias_record[1024];
    cell current_alias_pointer = 0;
      exe_prim(primitive_t primitive) {
        primitive();
      }
      exe_prim_keyword(primitive_t primitive) {
        push(keyword_stack, current_alias_pointer);
        primitive();
        current_alias_pointer = pop(keyword_stack);
      }
      exe_jojo(jo_t* jojo) {
        return_stack_push_new(jojo);
      }
      exe_keyword(jo_t* jojo) {
        // keywords are always evaled
        push(keyword_stack, current_alias_pointer);
        return_stack_push_new(jojo);
        eval();
        current_alias_pointer = pop(keyword_stack);
      }
      exe_get_object_field(cell index) {
        struct object a = object_stack_pop();
        object_stack_push(get_object_field_tag(a.data, index),
                          get_object_field_data(a.data, index));
      }
      exe_set_object_field(cell index) {
        struct object a = object_stack_pop();
        struct object b = object_stack_pop();
        set_object_field_tag(a.data, index, b.tag);
        set_object_field_data(a.data, index, b.data);
      }
      exe_set_global_variable(jo_t name) {
        struct object a = object_stack_pop();
        rebind_name(name, a.tag, a.data);
      }
    add_the_object_class() {
      struct class* class = (struct class*)malloc(sizeof(struct class));
      jo_t name = str2jo("<object>");
      class->class_name = name;
      class->gc_actor = gc_recur;
      class->object_size = 0;
      class->executable = false;

      bind_name(name, str2jo("<class>"), class);
    }
    expose_object() {
      init_object_record();

      add_the_object_class();

      add_atom_class("<int>", gc_ignore);
      add_atom_class("<jo>", gc_ignore);
      add_atom_class("<string>", gc_free);
      add_atom_class("<class>", gc_ignore);
      add_atom_class("<generic-prototype>", gc_ignore);
      add_atom_class("<uninitialised-field-place-holder>", gc_ignore);

      add_executable_atom_class("<prim>", gc_ignore, exe_prim);
      add_executable_atom_class("<prim-keyword>", gc_ignore, exe_prim_keyword);
      add_executable_atom_class("<jojo>", gc_ignore, exe_jojo);
      add_executable_atom_class("<keyword>", gc_ignore, exe_keyword);
      add_executable_atom_class("<set-object-field>", gc_ignore, exe_set_object_field);
      add_executable_atom_class("<get-object-field>", gc_ignore, exe_get_object_field);
      add_executable_atom_class("<set-global-variable>", gc_ignore, exe_set_global_variable);

      add_prim("new", S("<class>"), p_new);
    }
      struct absolute_t {
        jo_t root;
        jo_t current;
      };
      bool absolute_end_p(absolute_array, arity)
        struct absolute_t absolute_array[];
        cell arity;
      {
        cell i = 0;
        while (i < arity) {
          if (absolute_array[i].current != str2jo("<object>")) {
            return false;
          }
          i++;
        }
        return true;
      }
      absolute_next(absolute_array, arity)
        struct absolute_t absolute_array[];
        cell arity;
      {
        // inc non <object>
        cell i = arity-1;
        while (i >= 0) {
          if (absolute_array[i].current != str2jo("<object>")) {
            struct class* class = absolute_array[i].current->data;
            absolute_array[i].current = class->super_name;
            i++;
            break;
          }
          i--;
        }
        // reset the rest to object
        while (i < arity) {
          absolute_array[i].current = absolute_array[i].root;
          i++;
        }
      }
      jo_t absolute_currnet_jo(jo, absolute_array, arity)
        jo_t jo;
        struct absolute_t absolute_array[];
        cell arity;
      {
        char buffer[1024];
        char* cursor = buffer;
        cell i = 0;
        while (i < arity) {
          jo_t class_name = absolute_array[i].current;
          char* str = jo2str(class_name);
          strcpy(cursor, str);
          cursor = cursor + strlen(str);
          i++;
        }
        strcpy(cursor, jo2str(jo));
        return str2jo(buffer);
      }
      jo_t absolute_jo_loop(jo, absolute_array, arity)
        jo_t jo;
        struct absolute_t absolute_array[];
        cell arity;
      {
        jo_t new_jo = absolute_currnet_jo(jo, absolute_array, arity);
        if (used_jo_p(new_jo)) {
          return new_jo;
        }
        else if (absolute_end_p(absolute_array, arity)) {
          return NULL;
        }
        else {
          absolute_next(absolute_array, arity);
          return absolute_jo_loop(jo, absolute_array, arity);
        }
      }
      jo_t absolute_jo(jo_t jo) {
        cell arity = jo->data;
        struct absolute_t absolute_array[256];
        cell tag_index = arity;
        cell i = 0;
        jo_t tag;
        while (i < arity) {
          tag = object_stack_peek_tag(tag_index);
          absolute_array[i].root = tag;
          absolute_array[i].current = tag;
          tag_index--;
          i++;
        }
        jo_t new_jo = absolute_jo_loop(jo, absolute_array, arity);
        if (new_jo == NULL) {
          report("- absolute_jo can not find\n");
          return NULL;
        }
        else {
          return new_jo;
        }
      }
    p_debug();

    jo_apply(jo_t jo) {
      if (!used_jo_p(jo)) {
        report("- jo_apply meet undefined jo : %s\n", jo2str(jo));
        p_debug();
        return;
      }
      if (jo->tag == str2jo("<generic-prototype>")) {
        jo_apply(absolute_jo(jo));
        return;
      }

      struct class* class = jo->tag->data;
      if (class->executable) {
        class->executer(jo->data);
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
        jo_t* jojo = rp.jojo;
        jo_t jo = jojo[0];
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
      add_prim("drop", S("<object>"), p_drop);
      add_prim("dup",  S("<object>"), p_dup);
      add_prim("over", S("<object>", "<object>"), p_over);
      add_prim("tuck", S("<object>", "<object>"), p_tuck);
      add_prim("swap", S("<object>", "<object>"), p_swap);
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
      add_prim("end", S0, p_end);
      add_prim("bye", S0, p_bye);
    }
    struct stack* reading_stack; // of input_stack
    struct stack* writing_stack; // of output_stack
    bool has_byte_p() {
      return !input_stack_empty_p(tos(reading_stack));
    }
    byte read_byte() {
      return input_stack_pop(tos(reading_stack));
    }
    byte_unread(byte b) {
      input_stack_push(tos(reading_stack), b);
    }
    byte_print(byte b) {
      output_stack_push(tos(writing_stack), b);
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
    jo_t read_raw_jo() {
      byte buf[1024];
      cell cur = 0;
      cell collecting = false;
      byte c;
      byte go = true;

      while (go) {

        if (!has_byte_p()) {
          if (!collecting) {
            report("- p_read_raw_jo meet end-of-file\n");
            return;
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
    string_unread(char* str) {
      if (str[0] == '\0') {
        return;
      }
      else {
        string_unread(str+1);
        byte_unread(str[0]);
      }
    }
    jo_unread(jo_t jo) {
      char* str = jo2str(jo);
      byte_unread(' ');
      string_unread(str);
      byte_unread(' ');
    }
    expose_rw() {

    }
    k_ignore() {
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
    ins_lit() {
      // [] -> [cell] {return_stack}
      struct ret rp = return_stack_tos();
      return_stack_inc();
      return_stack_inc();
      jo_t* jojo = rp.jojo;
      jo_t tag = jojo[0];
      cell data = jojo[1];
      object_stack_push(tag, data);
    }
    compile_string() {
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
      object_entry->mark = GC_MARK_USING;
      object_entry->gc_actor = gc_ignore;
      object_entry->pointer = str;

      here(JO_INS_LIT);
      here(TAG_STRING);
      here(object_entry);
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
      cell local_find(jo_t name) {
        // return index of local_record
        // -1 -- no found
        struct ret rp = return_stack_tos();
        cell cursor = current_local_pointer - 1;
        while (cursor >= rp.local_pointer) {
          if (local_record[cursor].name == name) {
            return cursor;
          }
          else {
            cursor--;
          }
        }
        return -1;
      }
      ins_set_local() {
        struct ret rp = return_stack_tos();
        return_stack_inc();
        jo_t* jojo = rp.jojo;
        jo_t jo = jojo[0];

        cell index = local_find(jo);

        struct object a = object_stack_pop();
        if (index != -1) {
          local_record[index].name = jo;
          local_record[index].local_tag = a.tag;
          local_record[index].local_data = a.data;
        }
        else {
          local_record[current_local_pointer].name = jo;
          local_record[current_local_pointer].local_tag = a.tag;
          local_record[current_local_pointer].local_data = a.data;
          current_local_pointer++;
        }
      }
      ins_get_local() {
        struct ret rp = return_stack_tos();
        return_stack_inc();
        jo_t* jojo = rp.jojo;
        jo_t jo = jojo[0];

        cell index = local_find(jo);

        if (index != -1) {
          struct local lp = local_record[index];
          object_stack_push(lp.local_tag, lp.local_data);
        }
        else {
          report("- ins_get_local fatal error\n");
          report("  name is not bound\n");
          report("  name : %s\n", jo2str(jo));
          p_debug();
        }
      }
    compile_jo(jo_t jo) {
      if (jo == ROUND_BAR) {
        jo_apply(read_jo());
        return;
      }
      char* str = jo2str(jo);
      if (int_string_p(str)) {
        here(JO_INS_LIT);
        here(TAG_INT);
        here(string_to_int(str));
      }
      else if (jo == DOUBLE_QUOTE) {
        compile_string();
      }
      else if (get_local_string_p(str)) {
        here(JO_INS_GET_LOCAL);
        here(jo);
      }
      else if (set_local_string_p(str)) {
        here(JO_INS_SET_LOCAL);
        char* tmp = substring(str, 0, strlen(str) -1);
        here(str2jo(tmp));
        free(tmp);
      }
      else if (used_jo_p(jo)) {
        here(jo);
      }
      else {
        // no compile before define
        report("- compile_jo undefined : %s\n", jo2str(jo));
        p_debug();
      }
    }
    compile_until_meet_jo(jo_t ending_jo) {
      while (true) {
        jo_t jo = read_jo();
        if (jo == ending_jo) {
          return;
        }
        else {
          compile_jo(jo);
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
    p_compile_until_round_ket() {
      compile_until_meet_jo(ROUND_KET);
    }
    struct stack* current_compiling_jojo_stack; // of jo
    p_compile_jojo() {
      jo_t* jojo = tos(compiling_stack);
      push(current_compiling_jojo_stack, jojo);
      compile_until_meet_jo(ROUND_KET);
      here(JO_END);
      here(0);
      here(0);
      drop(current_compiling_jojo_stack);
    }
    k_run() {
      // (run ...)
      jo_t* jojo = tos(compiling_stack);
      p_compile_jojo();
      return_stack_push_new(jojo);
      eval();
    }
    k_add_var() {
      jo_t name = read_jo();
      k_run();
      struct object a = object_stack_pop();
      bind_name(name, a.tag, a.data);

      char name_buffer[1024];
      name_buffer[0] = '\0';
      strcat(name_buffer, jo2str(name));
      strcat(name_buffer, "!");
      if (check_function_arity(name_buffer, 1)) {
        name_buffer[0] = '\0';
        strcat(name_buffer, "<object>");
        strcat(name_buffer, jo2str(name));
        strcat(name_buffer, "!");
        bind_name(str2jo(name_buffer), str2jo("<set-global-variable>"), name);
      }
      else {
        report("- def fail\n");
        return;
      }
    }
    #define MAX_FIELDS 1024

    k_add_class() {
      jo_t name = read_jo();
      read_jo(); // drop '('
      jo_t super = read_jo();
      k_ignore();
      jo_t fields[MAX_FIELDS];
      cell i = 0;
      while (true) {
        if (i >= MAX_FIELDS) {
          k_ignore;
          report("- k_add_class fail\n");
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
      _add_class(name, super, fields);
    }
    k_add_fun_args(jo_t* args) {
      if (args[0] == NULL) { return; }
      k_add_fun_args(args+1);
      // ><><>< little syntax check here
      {
        here(JO_INS_SET_LOCAL);
        here(args[0]);
      }
    }

    // caller free
    jo_t* k_add_fun_tags() {
      read_jo(); // drop '('
      jo_t* tags = (jo_t*)malloc(64 * sizeof(jo_t));
      jo_t args[64];
      cell i = 0;
      while (true) {
        jo_t arg = read_jo(); if (arg == ROUND_KET) { break; }
        args[i] = arg;
        jo_t tag = read_jo();
        tags[i] = tag;
        i++;
      }
      args[i] = NULL;
      k_add_fun_args(args);
      tags[i] = NULL;
      return tags;
    }
    k_add_fun() {
      jo_t fun_name = read_jo();
      jo_t* jojo = tos(compiling_stack);
      jo_t* tags = k_add_fun_tags();
      p_compile_jojo();

      char name_buffer[1024];
      char* cursor = name_buffer;
      cell i = 0;
      while (tags[i] != NULL) {
        strcpy(cursor, jo2str(tags[i]));
        cursor = cursor + strlen(jo2str(tags[i]));
        i++;
      }
      strcpy(cursor, jo2str(fun_name));
      free(tags);
      jo_t name = str2jo(name_buffer);

      cell arity = i;
      if (arity == 0 ||
          check_function_arity(jo2str(fun_name), arity)) {
        bind_name(name, TAG_JOJO, jojo);
      }
      else {
        report("- k_add_fun fall\n");
        report("  arity of %s should not be %ld\n", jo2str(fun_name), arity);
      }
    }
    expose_add() {
      add_prim_keyword("+var", S0, k_add_var);
      add_prim_keyword("+fun", S0, k_add_fun);
      add_prim_keyword("+class", S0, k_add_class);

      add_prim_keyword("run", S0, k_run);

      add_prim("ins/lit", S0, ins_lit);
      add_prim("ins/get-local", S0, ins_get_local);
      add_prim("ins/set-local", S0, ins_set_local);
    }
    p_print_object_stack() {
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
    p_repl_flag_on() { repl_flag = true; }
    p_repl_flag_off() { repl_flag = false; }

    p_repl() {
      while (true) {
        if (!has_jo_p()) {
          return 69;
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
    expose_repl() {
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
      report("- SIGSEGV : %ld\n", SIGSEGV);
    }
    expose_play() {
      add_prim("p1", S0, p1);
      add_prim("p2", S0, p2);
      add_prim("p3", S0, p3);
      add_prim("print-object-stack", S0, p_print_object_stack);
    }
    init_system() {
      setvbuf(stdout, NULL, _IONBF, 0);
      setvbuf(stderr, NULL, _IONBF, 0);
    }
    init_jotable() {
      bzero(jotable,
            JOTABLE_SIZE *
            sizeof(struct jotable_entry));
    }
    init_literal_jo() {
      EMPTY_JO = str2jo("");

      TAG_PRIM         = str2jo("<prim>");
      TAG_JOJO         = str2jo("<jojo>");
      TAG_PRIM_KEYWORD = str2jo("<prim-keyword>");
      TAG_KEYWORD      = str2jo("<keyword>");
      TAG_INT          = str2jo("<int>");
      TAG_STRING       = str2jo("<string>");

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
    jo_t jojo_area[1024 * 1024];

    init_stacks() {
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

      current_compiling_jojo_stack = new_stack("current_compiling_jojo_stack");
    }
    init_jojo() {
      init_jotable();
      init_literal_jo();
      init_stacks();

      expose_object();
      expose_stack_operation();
      expose_ending();
      expose_add();

      expose_play();
    }
  int main(int argc, char** argv) {
    // cmd_number = argc;
    // cmd_string_array = argv;
    init_system();
    init_jojo();
    p_repl_flag_on();
    {
      add_class("<rectangle>", "<object>", S("height", "width"));

      object_stack_push(str2jo("<int>"), 666);
      object_stack_push(str2jo("<int>"), 888);

      here(str2jo("over"));
      here(str2jo("swap"));
      here(str2jo("<rectangle>"));
      here(str2jo("new"));
      here(str2jo("tuck"));
      here(str2jo(".width!"));
      here(str2jo("dup"));
      here(str2jo(".width"));
      here(str2jo("swap"));
      here(str2jo(".width"));
      here(str2jo("<string>"));
      here(str2jo("print-object-stack"));

      here(str2jo("<rectangle>"));
      here(str2jo("new"));

      here(str2jo("<rectangle>"));
      here(str2jo("new"));
      here(str2jo("drop"));

      here(str2jo("<rectangle>"));
      here(str2jo("new"));
      here(str2jo("drop"));

      here(str2jo("<rectangle>"));
      here(str2jo("new"));
      here(str2jo("drop"));

      here(str2jo("<rectangle>"));
      here(str2jo("new"));
      here(str2jo(".height"));

      here(str2jo("print-object-stack"));

      here(str2jo("end"));
      return_stack_push_new(jojo_area);
      eval();
    }

    return p_repl();
  }
