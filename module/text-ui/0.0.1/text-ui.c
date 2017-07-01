#include <ncurses.h>
#include <string.h>
#include <locale.h>
#include <wctype.h>
#include <wchar.h>
#include <stdio.h>
#include "libjojo.h"

void text_ui_init() {
  setlocale(LC_ALL, "");
  initscr();
  keypad(stdscr, TRUE);
  raw();
  noecho();
}

void receive_byte() {
  // (-> byte)
  as_push(getch());
}

void byte_unreceive() {
  // (byte ->)
  unreceivech(as_pop());
}

void byte_draw() {
  // (byte -> [text-window])
  int c = as_pop();
  addch(c);
}

void string_draw() {
  // (string -> [text-window])
  string str = as_pop();
  addstr(str);
}

void x_border() {
  // (-> x)
  int y, x;
  getmaxyx(stdscr, y, x);
  as_push(x);
}

void y_border() {
  // (-> y)
  int y, x;
  getmaxyx(stdscr, y, x);
  as_push(y);
}

void x_cursor() {
  // (-> x)
  int y, x;
  getyx(stdscr, y, x);
  as_push(x);
}

void y_cursor() {
  // (-> y)
  int y, x;
  getyx(stdscr, y, x);
  as_push(y);
}

void set_x_cursor() {
  // (x ->)
  int x = as_pop();
  int y0, x0;
  getyx(stdscr, y0, x0);
  move(y0, x);
}

void set_y_cursor() {
  // (y ->)
  int y = as_pop();
  int y0, x0;
  getyx(stdscr, y0, x0);
  move(y, x0);
}

void export() {
  defprim("text-ui/init", text_ui_init);
  defprim("text-ui/end", endwin);
  defprim("text-ui/refresh", refresh);

  defprim("receive/byte", receive_byte);
  defprim("byte/unreceive", byte_unreceive);

  defprim("byte/draw", byte_draw);
  defprim("string/draw", string_draw);

  defprim("x-border", x_border);
  defprim("y-border", y_border);

  defprim("x-cursor", x_cursor);
  defprim("y-cursor", y_cursor);

  defprim("set-x-cursor", set_x_cursor);
  defprim("set-y-cursor", set_y_cursor);
}
