#![feature (uniform_paths)]
#![feature (range_contains)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

use std::collections::VecDeque;
use std::cmp;
use ansi_term::{
    Style,
    Color,
};

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn len (&self) -> usize {
        self.hi - self.lo
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct ErrorMsg {
    head: Option <String>,
    line_vec: Vec <String>,
}

impl ErrorMsg {
    pub fn new () -> Self {
        ErrorMsg {
            head: None,
            line_vec: Vec::new (),
        }
    }
}

impl ErrorMsg {
    pub fn head (mut self, head: &str) -> Self {
        let head = head.to_string ();
        self.head = Some (head);
        self
    }
}

impl ErrorMsg {
    pub fn line (mut self, line: &str) -> Self {
        let line = line.to_string ();
        self.line_vec.push (line);
        self
    }
}

impl ErrorMsg {
    pub fn report (&self) {
        if let Some (head) = &self.head {
            eprintln! ("- {}", head);
        } else {
            eprintln! ("-");
        }
        for line in &self.line_vec {
            eprintln! ("  {}", line);
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct ErrorInCtx {
    msg: ErrorMsg,
    info_vec: Vec <ErrorMsg>,
    note_vec: Vec <ErrorMsg>,
    span: Option <Span>,
}

impl ErrorInCtx {
    pub fn new () -> Self {
        ErrorInCtx {
            msg: ErrorMsg::new (),
            info_vec: Vec::new (),
            note_vec: Vec::new (),
            span: None,
        }
    }
}

impl ErrorInCtx {
    pub fn head (mut self, head: &str) -> Self {
        self.msg = self.msg.head (head);
        self
    }
}

impl ErrorInCtx {
    pub fn line (mut self, line: &str) -> Self {
        self.msg = self.msg.line (line);
        self
    }
}

impl ErrorInCtx {
    pub fn info (mut self, info: ErrorMsg) -> Self {
        self.info_vec.push (info);
        self
    }
}

impl ErrorInCtx {
    pub fn note (mut self, note: ErrorMsg) -> Self {
        self.note_vec.push (note);
        self
    }
}

impl ErrorInCtx {
    pub fn span (mut self, span: Span) -> Self {
        self.span = Some (span);
        self
    }
}

impl <'a> ErrorInCtx {
    pub fn report (&self, ctx: ErrorCtx <'a>) {
        fn print_key (key: &str, color: Color) {
            eprint! ("- {} ", Style::new ()
                     .fg (color)
                     .paint (key));
        }
        print_key ("error", Color::Red);
        self.msg.report ();
        for info in &self.info_vec {
            print_key ("info", Color::Purple);
            info.report ();
        }
        if let Some (span) = &self.span {
            ctx.report (span.clone ());
        }
        for note in &self.note_vec {
            print_key ("note", Color::Cyan);
            note.report ();
        }
        eprintln! ("");
    }
}

impl ErrorInCtx {
    pub fn wrap_in_err <T> (self) -> Result <T, Self> {
        Err (self)
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct ErrorCtx <'a> {
    source: Option <&'a str>,
    body: Option <&'a str>,
}

impl <'a> ErrorCtx <'a> {
    pub fn new () -> Self {
        ErrorCtx {
            source: None,
            body: None,
        }
    }
}

impl <'a> ErrorCtx <'a> {
    pub fn source (mut self, source: &'a str) -> Self {
        self.source = Some (source);
        self
    }
}

impl <'a> ErrorCtx <'a> {
    pub fn body (mut self, body: &'a str) -> Self {
        self.body = Some (body);
        self
    }
}

impl <'a> ErrorCtx <'a> {
    pub fn report (&self, span: Span) {
        let key = Style::new ()
            .fg (Color::Blue)
            .paint ("context");
        let source = if let Some (source) = &self.source {
            source.to_string ()
        } else {
            String::new ()
        };
        eprintln! ("- {} - {}", key, source);
        if let Some (body) = &self.body {
            let line_vec: Vec <&str> = body.lines () .collect ();
            let len = line_vec.len ();
            let width = format! ("{}", len) .len ();
            let fringe = 3;
            let focus = self.focus (line_vec, span, fringe);
            for (i, line, span) in focus {
                let n = i + 1;
                print_line (n, width, line, span);
            }
        }
    }
}

type Focus <'a> = VecDeque <(usize, &'a str, Option <Span>)>;

impl <'a> ErrorCtx <'a> {
    pub fn focus (
        &self,
        line_vec: Vec <&'a str>,
        span: Span,
        fringe: usize,
    ) -> Focus <'a> {
        let mut before = Focus::new ();
        let mut focus  = Focus::new ();
        let mut after  = Focus::new ();
        let mut met_span_p = false;
        let mut lo = span.lo as isize;
        let mut hi = span.hi as isize;
        for (i, line) in line_vec.into_iter () .enumerate () {
            let len = line.len () as isize;
            if lo >= len || hi <= 0 {
                if met_span_p {
                    after.push_back ((i, line, None));
                } else {
                    before.push_back ((i, line, None));
                }
            } else {
                met_span_p = true;
                let span = Span {
                    lo: cmp::max (lo, 0) as usize,
                    hi: cmp::min (hi, len) as usize,
                };
                focus.push_back ((i, line, Some (span)));
            }
            lo -= (line.len () + 1) as isize;
            hi -= (line.len () + 1) as isize;
        }
        for _ in 0..fringe {
            if let Some (x) = before.pop_back () {
                focus.push_front (x);
            }
            if let Some (x) = after.pop_front () {
                focus.push_back (x);
            }
        }
        focus
    }
}

fn print_line (
    n: usize,
    width: usize,
    line: &str,
    span: Option <Span>,
) {
    print_line_number_prefix (n, width);
    if let Some (span) = span {
        let lo = span.lo;
        let hi = span.hi;
        let mid = &line [lo .. hi];
        let mid = Style::new () .reverse () .paint (mid);
        eprintln! ("{}{}{}",
                   &line [.. lo],
                   mid,
                   &line [hi ..],);
    } else {
        eprintln! ("{}", line);
    }
}

fn print_line_number_prefix (n: usize, width: usize) {
    let prefix = format! ("  {:w$} | ", n, w = width);
    let prefix = Style::new ()
        .fg (Color::Blue)
        .paint (prefix);
    eprint! ("{}", prefix);
}

#[cfg (test)]
const EXAMPLE_INPUT: &'static str = "\
list-t : type-tt
list-t = union (null-t cons-t) {
  t : type-tt
}

null-t : type-tt
null-t = data {
  t : type-tt
}

cons-t : type-tt
cons-t = data {
  t : type-tt
  car : t
  cdr : list-t (t)
}
";

#[test]
fn test_report () {
    let input = EXAMPLE_INPUT;
    let error = ErrorInCtx::new ()
        .head ("no worry")
        .line ("this is just a testing error")
        .span (Span { lo: 84, hi: 115 })
        // thanks to github.com/xuchunyang/region-state.el
        .info (ErrorMsg::new ()
               .line ("x = x")
               .line ("y = y")
               .line ("显然成立"))
        .note (ErrorMsg::new ()
               .line ("打雷要下雨 雷欧")
               .line ("下雨要打伞 雷欧"))
        .note (ErrorMsg::new ()
               .line ("不知道的奥秘万万千千 智慧简单又不简单"));
    error.report (ErrorCtx::new ()
                  .source ("fn test_report")
                  .body (input));
}
