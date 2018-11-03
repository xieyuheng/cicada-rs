use std::str::Chars;
use std::iter::Peekable;
use std::slice;

pub type Num = f64;

pub type TokenVec = Vec <Token>;

type Word = String;
type WordVec = Vec <Word>;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    List { token_vec: TokenVec },
    Vect { token_vec: TokenVec },
    Dict { token_vec: TokenVec },
    QuotationMark {
        mark: String,
        mark_name: String,
        token: Box <Token>,
    },
    Num { num: Num },
    Str { str: String },
    Sym { sym: String },
}

type WordIter <'a> = Peekable <slice::Iter <'a, Word>>;

fn collect_list_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let next_word = iter.next () .unwrap ();
    assert_eq! (next_word, &"(");
    let mut token_vec = TokenVec::new ();
    loop {
        match iter.peek () {
            Some (&word) if word == ")" => {
                iter.next ();
                let token = Token::List { token_vec };
                break (token, iter);
            }
            Some (_) => {
                let (token, rest) = collect_token (iter);
                token_vec.push (token);
                iter = rest;
            }
            None => {
                panic! ("collect_list_token fail!");
            }
        }
    }
}

fn collect_vect_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let next_word = iter.next () .unwrap ();
    assert_eq! (next_word, &"[");
    let mut token_vec = TokenVec::new ();
    loop {
        match iter.peek () {
            Some (&word) if word == "]" => {
                iter.next ();
                let token = Token::Vect { token_vec };
                break (token, iter);
            }
            Some (_) => {
                let (token, rest) = collect_token (iter);
                token_vec.push (token);
                iter = rest;
            }
            None => {
                panic! ("collect_vect_token fail!");
            }
        }
    }
}

fn collect_dict_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let next_word = iter.next () .unwrap ();
    assert_eq! (next_word, &"{");
    let mut token_vec = TokenVec::new ();
    loop {
        match iter.peek () {
            Some (&word) if word == "}" => {
                iter.next ();
                let token = Token::Dict { token_vec };
                break (token, iter);
            }
            Some (_) => {
                let (token, rest) = collect_token (iter);
                token_vec.push (token);
                iter = rest;
            }
            None => {
                panic! ("collect_dict_token fail!");
            }
        }
    }
}

fn collect_quote_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let mark = iter.next () .unwrap ();
    assert_eq! (mark, &"'");
    let (next_token, next_iter) = collect_token (iter);
    let token = Token::QuotationMark {
        mark: mark.clone (),
        mark_name: String::from ("quote"),
        token: Box::new (next_token),
    };
    (token, next_iter)
}

fn collect_quasiquote_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let mark = iter.next () .unwrap ();
    assert_eq! (mark, &"`");
    let (next_token, next_iter) = collect_token (iter);
    let token = Token::QuotationMark {
        mark: mark.clone (),
        mark_name: String::from ("quasiquote"),
        token: Box::new (next_token),
    };
    (token, next_iter)
}

fn collect_unquote_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let mark = iter.next () .unwrap ();
    assert_eq! (mark, &"~");
    let (next_token, next_iter) = collect_token (iter);
    let token = Token::QuotationMark {
        mark: mark.clone (),
        mark_name: String::from ("unquote"),
        token: Box::new (next_token),
    };
    (token, next_iter)
}

fn collect_unquote_splicing_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let mark = iter.next () .unwrap ();
    assert_eq! (mark, &"~@");
    let (next_token, next_iter) = collect_token (iter);
    let token = Token::QuotationMark {
        mark: mark.clone (),
        mark_name: String::from ("unquote-splicing"),
        token: Box::new (next_token),
    };
    (token, next_iter)
}

pub fn num_word_p (word: &str) -> bool {
    word.parse::<Num> () .is_ok ()
}

pub fn str_word_p (word: &str) -> bool {
    let len = word.len ();
    (len >= 2 &&
     word.starts_with ("\"") &&
     word.ends_with ("\""))
}

pub fn sym_word_p (word: &str) -> bool {
    // not real check for sym for now
    word.len () > 0
}

fn collect_num_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let word = iter.next () .unwrap ();
    let num = word.parse::<Num> () .unwrap ();
    let token = Token::Num { num };
    (token, iter)
}

fn collect_str_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let word = iter.next () .unwrap ();
    let len = word.len ();
    let str = String::from (&word [1..len-1]);
    let token = Token::Str { str };
    (token, iter)
}

fn collect_sym_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let word = iter.next () .unwrap ();
    let sym = word.clone ();
    let token = Token::Sym { sym };
    (token, iter)
}

fn collect_token (
    mut iter: WordIter
) -> (Token, WordIter) {
    let word = iter.peek () .unwrap ();
    match word.as_str () {
        "(" => collect_list_token (iter),
        "[" => collect_vect_token (iter),
        "{" => collect_dict_token (iter),
        "'" => collect_quote_token (iter),
        "`" => collect_quasiquote_token (iter),
        "~" => collect_unquote_token (iter),
        "~@" => collect_unquote_splicing_token (iter),
        _ if num_word_p (word) => collect_num_token (iter),
        _ if str_word_p (word) => collect_str_token (iter),
        _ if sym_word_p (word) => collect_sym_token (iter),
        _  => { panic! ("collect_token!"); }
    }
}

pub fn scan (code: &str) -> TokenVec {
    let word_vec = split_to_word_vec (code);
    let mut iter = word_vec .iter () .peekable ();
    let mut token_vec = TokenVec::new ();
    while let Some (_) = iter.peek () {
        let (token, rest) = collect_token (iter);
        token_vec.push (token);
        iter = rest;
    }
    token_vec
}

fn space_char_p (c: char) -> bool {
    (c == ' ' ||
     c == '\n' ||
     c == '\t')
}

fn comma_char_p (c: char) -> bool {
    (c == ',')
}

fn delimiter_char_p (c: char) -> bool {
    (c == '(' ||
     c == ')' ||
     c == '[' ||
     c == ']' ||
     c == '{' ||
     c == '}' ||
     c == ',' ||
     c == '`' ||
     c == '\'')
}

fn semicolon_char_p (c: char) -> bool {
    (c == ';')
}

fn newline_char_p (c: char) -> bool {
    (c == '\n')
}

fn doublequote_char_p (c: char) -> bool {
    (c == '"')
}

fn tilde_char_p (c: char) -> bool {
    (c == '~')
}

fn punctuation_char_p (c: char) -> bool {
    (c == '~' ||
     c == '@' ||
     c == '#' ||
     c == '$' ||
     c == '%' ||
     c == '^' ||
     c == '&' ||
     c == '*' ||
     c == '-' ||
     c == '_' ||
     c == '+' ||
     c == '=' ||
     c == '?' ||
     c == '\\' ||
     c == '/' ||
     delimiter_char_p (c) ||
     space_char_p (c) ||
     semicolon_char_p (c) ||
     comma_char_p (c) ||
     doublequote_char_p (c))
}

fn collect_delimiter_word (iter: &mut Peekable <Chars>) -> Word {
    let mut word = String::from ("");
    let c = iter.next () .unwrap ();
    word.push (c);
    word
}

fn ignore_comment (iter: &mut Peekable <Chars>) {
    iter.next ();
    loop {
        if let Some (c) = iter.next () {
            if newline_char_p (c) {
                break;
            }
        }
        else {
            break;
        }
    }
}

fn collect_doublequote_word (iter: &mut Peekable <Chars>) -> Word {
    let mut word = String::from ("");
    let c = iter.next () .unwrap ();
    word.push (c);
    loop {
        if let Some (c) = iter.next () {
            word.push (c);
            if doublequote_char_p (c) {
                break;
            }
        }
        else {
            panic! ("doublequote mismatch!");
        }
    }
    word
}

fn collect_tilde_word (iter: &mut Peekable <Chars>) -> Word {
    let mut word = String::from ("");
    loop {
        if let Some (&c) = iter.peek () {
            if ((! punctuation_char_p (c)) ||
                space_char_p (c) ||
                doublequote_char_p (c) ||
                semicolon_char_p (c) ||
                delimiter_char_p (c)){
                break;
            }
            word.push (c);
            iter.next ();
        }
        else {
            iter.next ();
            break;
        }
    }
    word
}

fn collect_simple_word (iter: &mut Peekable <Chars>) -> Word {
    let mut word = String::from ("");
    loop {
        if let Some (&c) = iter.peek () {
            if (space_char_p (c) ||
                doublequote_char_p (c) ||
                semicolon_char_p (c) ||
                delimiter_char_p (c))
            {
                break;
            }
            word.push (c);
            iter.next ();
        }
        else {
            iter.next ();
            break;
        }
    }
    word
}

fn split_to_word_vec (code: &str) -> WordVec {
    let mut word_vec = WordVec::new ();
    let mut iter = code.chars () .peekable ();
    while let Some (&c) = iter.peek () {
        if space_char_p (c) {
            iter.next ();
        } else if comma_char_p (c) {
            iter.next ();
        } else if delimiter_char_p (c) {
            word_vec.push (collect_delimiter_word (&mut iter));
        } else if semicolon_char_p (c) {
            ignore_comment (&mut iter);
        } else if doublequote_char_p (c) {
            word_vec.push (collect_doublequote_word (&mut iter));
        } else if tilde_char_p (c) {
            word_vec.push (collect_tilde_word (&mut iter));
        } else {
            word_vec.push (collect_simple_word (&mut iter));
        }
    }
    word_vec
}

#[test]
fn test_split_to_word_vec () {
    assert_eq! (split_to_word_vec (""), WordVec::new ());
    assert_eq! (split_to_word_vec ("a b c"),
                vec! ["a", "b", "c"]);
    assert_eq! (split_to_word_vec ("a, b, c,"),
                vec! ["a", "b", "c"]);
    assert_eq! (split_to_word_vec ("(a b c)"),
                vec! ["(", "a", "b", "c", ")"]);
    assert_eq! (split_to_word_vec ("(a (b) c)"),
                vec! ["(", "a", "(", "b", ")", "c", ")"]);
    assert_eq! (split_to_word_vec ("(\"a\" (b) c)"),
                vec! ["(", "\"a\"", "(", "b", ")", "c", ")"]);
    assert_eq! (split_to_word_vec ("(\"a\" (b) c) ;;;; 123"),
                vec! ["(", "\"a\"", "(", "b", ")", "c", ")"]);
    assert_eq! (split_to_word_vec ("'(a b c)"),
                vec! ["'", "(", "a", "b", "c", ")"]);
    assert_eq! (split_to_word_vec ("~@(a b c)"),
                vec! ["~@", "(", "a", "b", "c", ")"]);
}

#[test]
fn test_scan () {
    assert_eq! (scan (""), TokenVec::new ());
    assert_eq! (scan ("a b c"), vec! [
        Token::Sym { sym: String::from ("a") },
        Token::Sym { sym: String::from ("b") },
        Token::Sym { sym: String::from ("c") },
    ]);
    assert_eq! (scan ("(a b c)"), vec! [
        Token::List { token_vec: vec! [
            Token::Sym { sym: String::from ("a") },
            Token::Sym { sym: String::from ("b") },
            Token::Sym { sym: String::from ("c") },
        ]},
    ]);
    assert_eq! (scan (",(a, b, c,),"), vec! [
        Token::List { token_vec: vec! [
            Token::Sym { sym: String::from ("a") },
            Token::Sym { sym: String::from ("b") },
            Token::Sym { sym: String::from ("c") },
        ]},
    ]);
    assert_eq! (scan ("(a (b) c)"), vec! [
        Token::List { token_vec: vec! [
            Token::Sym { sym: String::from ("a") },
            Token::List { token_vec: vec! [
                Token::Sym { sym: String::from ("b") },
            ]},
            Token::Sym { sym: String::from ("c") },
        ]},
    ]);
    assert_eq! (scan ("(\"a\" (b) c)"), vec! [
        Token::List { token_vec: vec! [
            Token::Str { str: String::from ("a") },
            Token::List { token_vec: vec! [
                Token::Sym { sym: String::from ("b") },
            ]},
            Token::Sym { sym: String::from ("c") },
        ]},
    ]);
    assert_eq! (scan ("(\"a\" (b) c) ;;;; 123"), vec! [
        Token::List { token_vec: vec! [
            Token::Str { str: String::from ("a") },
            Token::List { token_vec: vec! [
                Token::Sym { sym: String::from ("b") },
            ]},
            Token::Sym { sym: String::from ("c") },
        ]},
    ]);
    assert_eq! (scan ("(\"a\" (-3.14) c) ;;;; 123"), vec! [
        Token::List { token_vec: vec! [
            Token::Str { str: String::from ("a") },
            Token::List { token_vec: vec! [
                Token::Num { num: Num::from (-3.14) },
            ]},
            Token::Sym { sym: String::from ("c") },
        ]},
    ]);
    assert_eq! (scan ("(\"a\" [-3.14] c) ;;;; 123"), vec! [
        Token::List { token_vec: vec! [
            Token::Str { str: String::from ("a") },
            Token::Vect { token_vec: vec! [
                Token::Num { num: Num::from (-3.14) },
            ]},
            Token::Sym { sym: String::from ("c") },
        ]},
    ]);
    assert_eq! (scan ("('(a b c))"), vec! [
        Token::List { token_vec: vec! [
            Token::QuotationMark {
                mark: String::from ("'"),
                mark_name: String::from ("quote"),
                token: Box::new (Token::List { token_vec: vec! [
                    Token::Sym { sym: String::from ("a") },
                    Token::Sym { sym: String::from ("b") },
                    Token::Sym { sym: String::from ("c") },
                ]}),
            },
        ]},
    ]);
    assert_eq! (scan ("(~@(a b c))"), vec! [
        Token::List { token_vec: vec! [
            Token::QuotationMark {
                mark: String::from ("~@"),
                mark_name: String::from ("unquote-splicing"),
                token: Box::new (Token::List { token_vec: vec! [
                    Token::Sym { sym: String::from ("a") },
                    Token::Sym { sym: String::from ("b") },
                    Token::Sym { sym: String::from ("c") },
                ]}),
            },
        ]},
    ]);
    assert_eq! (scan ("('[a b c])"), vec! [
        Token::List { token_vec: vec! [
            Token::QuotationMark {
                mark: String::from ("'"),
                mark_name: String::from ("quote"),
                token: Box::new (Token::Vect { token_vec: vec! [
                    Token::Sym { sym: String::from ("a") },
                    Token::Sym { sym: String::from ("b") },
                    Token::Sym { sym: String::from ("c") },
                ]}),
            },
        ]},
    ]);
    assert_eq! (scan ("(~@[a b c])"), vec! [
        Token::List { token_vec: vec! [
            Token::QuotationMark {
                mark: String::from ("~@"),
                mark_name: String::from ("unquote-splicing"),
                token: Box::new (Token::Vect { token_vec: vec! [
                    Token::Sym { sym: String::from ("a") },
                    Token::Sym { sym: String::from ("b") },
                    Token::Sym { sym: String::from ("c") },
                ]}),
            },
        ]},
    ]);
    assert_eq! (scan ("{a = 1 b = 2 c = 3}"), vec! [
        Token::Dict { token_vec: vec! [
            Token::Sym { sym: String::from ("a") },
            Token::Sym { sym: String::from ("=") },
            Token::Num { num: 1.0 },
            Token::Sym { sym: String::from ("b") },
            Token::Sym { sym: String::from ("=") },
            Token::Num { num: 2.0 },
            Token::Sym { sym: String::from ("c") },
            Token::Sym { sym: String::from ("=") },
            Token::Num { num: 3.0 },
        ]},
    ]);
}
