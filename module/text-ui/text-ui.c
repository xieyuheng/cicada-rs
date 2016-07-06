#include <ncurses.h>
#include <string.h>
#include <locale.h>
#include <wctype.h>
#include <wchar.h>
#include <stdio.h>
#include "libjojo.h"

void init_ui() {
  setlocale(LC_ALL, "");
  initscr();
  raw();
  keypad(stdscr, TRUE);
  noecho();
}

void get_char() {
  // (-> char)
  as_push(getch());
}

void unget_char() {
  // (char ->)
  ungetch(as_pop());
}

void refresh_window() {
  // (window ->)
  wrefresh(as_pop());
}

void draw_char() {
  // (window char -> [text-window])
  int c = as_pop();
  WINDOW* win = as_pop();
  waddch(win, c);
}

void draw_string() {
  // (window string -> [text-window])
  string str = as_pop();
  WINDOW* win = as_pop();
  waddch(win, str);
}

void current_window() {
  // (-> window)
  as_push(stdscr);
}

void get_size() {
  // (window -> x y)
  int y, x;
  getmaxyx((WINDOW*)as_pop(), y, x);
  as_push(x);
  as_push(y);
}

void get_cursor() {
  // (window -> x y)
  int y, x;
  getyx((WINDOW*)as_pop(), y, x);
  as_push(x);
  as_push(y);
}

void set_cursor() {
  // (window x y ->)
  int y = as_pop();
  int x = as_pop();
  WINDOW* win = as_pop();
  wmove(win, y, x);
}

void export() {
  defprim("init-ui", init_ui);
  defprim("end-ui", endwin);
  defprim("get-char", get_char);
  defprim("unget-char", unget_char);
  defprim("refresh", refresh_window);
  defprim("draw-char", draw_char);
  defprim("draw-string", draw_string);
  defprim("current-window", current_window);
  defprim("get-size", get_size);
  defprim("get-cursor", get_cursor);
  defprim("set-cursor", set_cursor);
}
