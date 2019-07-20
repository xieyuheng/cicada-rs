

use std::collections::HashSet;
use error_report::{
    Span,
    ErrorInCtx,
};

#[cfg (test)]
use error_report::{
    ErrorMsg,
    ErrorCtx,
};

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Token <'a> {
    Quotation {
        span: Span,
        quotation_mark: char,
        string: &'a str,
    },
    Word {
        span: Span,
        word: &'a str,
    },
    Char {
        span: Span,
        ch: char,
    },
}

impl <'a> Token <'a> {
    pub fn span (&self) -> Span {
        match self {
            Token::Quotation { span, .. } => span.clone (),
            Token::Word { span, .. } => span.clone (),
            Token::Char { span, .. } => span.clone (),
        }
    }
}

impl <'a> Token <'a> {
    pub fn len (&self) -> usize {
        self.span () .len ()
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct LexTable {
    pub quotation_mark_set: HashSet <char>,
    pub space_set: HashSet <char>,
    pub char_set: HashSet <char>,
    pub line_comment_word_set: HashSet <String>,
}

impl LexTable {
    pub fn new () -> Self {
        LexTable {
            quotation_mark_set: HashSet::new (),
            space_set: HashSet::new (),
            char_set: HashSet::new (),
            line_comment_word_set: HashSet::new (),
        }
    }
}

impl LexTable {
    pub fn quotation_mark (mut self, ch: char) -> Self {
        self.quotation_mark_set.insert (ch);
        self
    }
}

impl LexTable {
    pub fn quotation_mark_p (&self, ch: char) -> bool {
        self.quotation_mark_set.contains (&ch)
    }
}

impl LexTable {
    pub fn space (mut self, ch: char) -> Self {
        self.space_set.insert (ch);
        self
    }
}

impl LexTable {
    pub fn space_p (&self, ch: char) -> bool {
        self.space_set.contains (&ch)
    }
}

impl LexTable {
    pub fn char (mut self, ch: char) -> Self {
        self.char_set.insert (ch);
        self
    }
}

impl LexTable {
    pub fn char_p (&self, ch: char) -> bool {
        self.char_set.contains (&ch)
    }
}

impl LexTable {
    pub fn line_comment_word (mut self, word: &str) -> Self {
        self.line_comment_word_set.insert (word.to_string ());
        self
    }
}

impl LexTable {
    pub fn line_comment_word_p (&self, word: &str) -> bool {
        self.line_comment_word_set.contains (word)
    }
}

impl <'a> LexTable {
    pub fn lex (
        &self,
        input: &'a str,
    ) -> Result <Vec <Token <'a>>, ErrorInCtx> {
        let lexing = Lexing {
            cursor: 0,
            lex_table: self.clone (),
            token_vec: Vec::new (),
            input,
        };
        lexing.run ()
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
struct Lexing <'a> {
    cursor: usize,
    input: &'a str,
    lex_table: LexTable,
    token_vec: Vec <Token <'a>>,
}

impl <'a> Lexing <'a> {
    fn run (
        mut self,
    ) -> Result <Vec <Token <'a>>, ErrorInCtx> {
        loop {
            self.ignore_space ();
            if self.finished_p () {
                return Ok (self.token_vec);
            }
            self.next_token ()?;
            if let Some (Token::Word {
                word, ..
            }) = self.token_vec.last () {
                if self.lex_table.line_comment_word_p (word) {
                    self.token_vec.pop ();
                    self.ignore_rest_line ();
                }
            }
        }
    }
}

impl <'a> Lexing <'a> {
    fn finished_p (&self) -> bool {
        self.cursor == self.input.len ()
    }
}

impl <'a> Lexing <'a> {
    fn ignore_space (&mut self) {
        loop {
            let progress = &self.input [self.cursor ..];
            if let Some (ch) = progress.chars () .next () {
                if self.lex_table.space_p (ch) {
                    self.cursor += ch.len_utf8 ();
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }
}

impl <'a> Lexing <'a> {
    fn ignore_rest_line (&mut self) {
        loop {
            let progress = &self.input [self.cursor ..];
            if let Some (ch) = progress.chars () .next () {
                if ch == '\n' {
                    self.cursor += ch.len_utf8 ();
                    return;
                } else {
                    self.cursor += ch.len_utf8 ();
                }
            } else {
                return;
            }
        }
    }
}

// assumptions :
// - Lexing is not finished_p
// - spaces are ignored
// - line comments are ignored
impl <'a> Lexing <'a> {
    fn next_token (
        &mut self,
    ) -> Result <(), ErrorInCtx> {
        let progress = &self.input [self.cursor ..];
        if let Some (ch) = progress.chars () .next () {
            if self.lex_table.char_p (ch) {
                self.next_char (ch)
            } else if self.lex_table.quotation_mark_p (ch) {
                self.next_quote (ch)
            } else {
                self.next_word ()
            }
        } else {
            Err (ErrorInCtx::new ()
                 .head ("internal-error"))
        }
    }
}

impl <'a> Lexing <'a> {
    fn next_char (
        &mut self,
        ch: char,
    ) -> Result <(), ErrorInCtx> {
        let lo = self.cursor;
        let ch_len = ch.len_utf8 ();
        self.cursor += ch_len;
        let hi = self.cursor;
        let span = Span { lo, hi };
        let token = Token::Char { span, ch };
        self.token_vec.push (token);
        Ok (())
    }
}

impl <'a> Lexing <'a> {
    fn next_quote (
        &mut self,
        quotation_mark: char,
    ) -> Result <(), ErrorInCtx> {
        let lo = self.cursor;
        let ch_len = quotation_mark.len_utf8 ();
        self.cursor += ch_len;
        let progress = &self.input [self.cursor ..];
        if let Some (
            quote_end
        ) = find_quote_end (progress, quotation_mark) {
            let string = &progress [.. quote_end];
            self.cursor += string.len ();
            self.cursor += ch_len;
            let hi = self.cursor;
            let span = Span { lo, hi };
            let token = Token::Quotation {
                span, quotation_mark, string,
            };
            self.token_vec.push (token);
            Ok (())
        } else {
            Err (ErrorInCtx::new ()
                 .head ("quotation-mark-mismatching")
                 .line (&format! (
                     "quotation_mark = {:?}",
                     quotation_mark))
                 .span (Span { lo, hi: self.input.len () }))
        }
    }
}

impl <'a> Lexing <'a> {
    fn goto_word_end (&mut self) {
        loop {
            let progress = &self.input [self.cursor ..];
            if let Some (ch) = progress.chars () .next () {
                if self.lex_table.space_p (ch) {
                    return;
                } else if self.lex_table.char_p (ch) {
                    return;
                } else if self.lex_table.quotation_mark_p (ch) {
                    return;
                } else {
                    self.cursor += ch.len_utf8 ();
                }
            } else {
                return;
            }
        }
    }
}

fn find_quote_end (
    string: &str,
    quotation_mark: char,
) -> Option <usize> {
    let mut cursor = 0;
    loop {
        let progress = &string [cursor ..];
        let mut chars = progress.chars ();
        if let Some (ch) = chars.next () {
            if ch == quotation_mark {
                return Some (cursor);
            } else if ch == '\\' {
                cursor += 1;
                if let Some (ch) = chars.next () {
                    cursor += ch.len_utf8 ();
                } else {
                    return None;
                }
            } else {
                cursor += ch.len_utf8 ();
            }
        } else {
            return None;
        }
    }
}

impl <'a> Lexing <'a> {
    fn next_word (
        &mut self,
    ) -> Result <(), ErrorInCtx> {
        let lo = self.cursor;
        self.goto_word_end ();
        let hi = self.cursor;
        let word = &self.input [lo .. hi];
        let span = Span { lo, hi };
        let token = Token::Word {
            span, word,
        };
        self.token_vec.push (token);
        Ok (())
    }
}

/// Equality without metadata -- without `span`.
pub fn token_eq <'a> (
    lhs: &Token <'a>,
    rhs: &Token <'a>,
) -> bool {
    match (lhs, rhs) {
        (Token::Quotation { quotation_mark: l1, string: l2, .. },
         Token::Quotation { quotation_mark: r1, string: r2, .. }
        ) => {
            (l1 == r1 &&
             l2 == r2)
        }
        (Token::Word { word: l1, .. },
         Token::Word { word: r1, .. }
        ) => {
            (l1 == r1)
        }
        (Token::Char { ch: l1, .. },
         Token::Char { ch: r1, .. }
        ) => {
            (l1 == r1)
        }
        (_, _) => false,
    }
}

/// Equality without metadata -- without `span`.
pub fn token_vec_eq <'a> (
    lhs: &Vec <Token <'a>>,
    rhs: &Vec <Token <'a>>,
) -> bool {
    (lhs.len () == rhs.len () &&
     lhs.iter () .zip (rhs.iter ())
     .all (|p| token_eq (&p.0, &p.1)))
}

#[test]
fn test_lexing () -> Result<(), ErrorInCtx> {
    let lex_table = LexTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';');
    let input = r#"aa "sss" c;"#;
    let token_vec = lex_table.lex (input)?;
    let mut iter = token_vec.iter ();
    assert_eq! (iter.next () .unwrap (), &Token::Word {
        span: Span { lo: 0, hi: 2 },
        word: "aa",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Quotation {
        span: Span { lo: 3, hi: 8 },
        quotation_mark: '"',
        string: "sss",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Word {
        span: Span { lo: 9, hi: 10 },
        word: "c",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Char {
        span: Span { lo: 10, hi: 11 },
        ch: ';',
    });
    assert_eq! (iter.next (), None);
    Ok (())
}

#[test]
fn test_lexing_unicode () -> Result<(), ErrorInCtx> {
    let lex_table = LexTable::new ()
        .space ('\n') .space ('\t') .space (' ')
        .char ('「') .char ('」');
    let input = r#"子游曰「敢問其方」"#;
    let token_vec = lex_table.lex (input)?;
    let mut iter = token_vec.iter ();
    assert! (
        if let Some (Token::Word { word, .. }) = iter.next () {
            word == &"子游曰"
        } else {
            false
        }
    );
    assert! (
        if let Some (Token::Char { ch, .. }) = iter.next () {
            ch == &'「'
        } else {
            false
        }
    );
    assert! (
        if let Some (Token::Word { word, .. }) = iter.next () {
            word == &"敢問其方"
        } else {
            false
        }
    );
    assert! (
        if let Some (Token::Char { ch, .. }) = iter.next () {
            ch == &'」'
        } else {
            false
        }
    );
    assert_eq! (iter.next (), None);
    Ok (())
}

#[test]
fn test_error () -> Result<(), ErrorInCtx> {
    let lex_table = LexTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';');
    let input = r#"aa "sss c;"#;
    assert! (
        if let Err (error) = lex_table.lex (input) {
            error
                .note (ErrorMsg::new ()
                       .head ("no worry")
                       .line ("this is just a testing error"))
                .print (ErrorCtx::new ()
                        .source ("in function test_error")
                        .body (input));
            true
        } else {
            false
        }
    );
    Ok (())
}

#[test]
fn test_escape () -> Result<(), ErrorInCtx> {
    let lex_table = LexTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';');
    let input = r#"aa "s\"" c;"#;
    let token_vec = lex_table.lex (input)?;
    let mut iter = token_vec.iter ();
    assert_eq! (iter.next () .unwrap (), &Token::Word {
        span: Span { lo: 0, hi: 2 },
        word: "aa",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Quotation {
        span: Span { lo: 3, hi: 8 },
        quotation_mark: '"',
        string: r#"s\""#,
    });
    assert_eq! (iter.next () .unwrap (), &Token::Word {
        span: Span { lo: 9, hi: 10 },
        word: "c",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Char {
        span: Span { lo: 10, hi: 11 },
        ch: ';',
    });
    assert_eq! (iter.next (), None);

    let lex_table = LexTable::new ()
        .quotation_mark ('"')
        .quotation_mark ('\'')
        .space ('\n') .space ('\t') .space (' ');
    let input = r#"'\''"#;
    let token_vec = lex_table.lex (input)?;
    let mut iter = token_vec.iter ();
    assert_eq! (iter.next () .unwrap (), &Token::Quotation {
        span: Span { lo: 0, hi: 4 },
        quotation_mark: '\'',
        string: r#"\'"#,
    });
    assert_eq! (iter.next (), None);
    Ok (())
}

#[test]
fn test_equality () -> Result<(), ErrorInCtx> {
    let lex_table = LexTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';');
    assert! (token_vec_eq (
        &lex_table.lex (r#"aa    "sss" c;"#)?,
        &lex_table.lex (r#"aa "sss" c;"#)?,
    ));
    Ok (())
}
