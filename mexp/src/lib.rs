#![feature (uniform_paths)]
#![feature (box_patterns)]
#![feature (box_syntax)]

#![allow (unused_parens)]
#![allow (unused_imports)]
#![allow (dead_code)]

use std::collections::HashSet;
use error_report::{
    Span,
    ErrorInCtx,
    ErrorMsg,
    ErrorCtx,
};
use lexing::{
    CharTable, Token,
};

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Mexp <'a> {
    Str  { span: Span, string: &'a str },
    Char { span: Span, ch: &'a str },
    Sym  { span: Span, symbol: &'a str },
    Dot  { span: Span,
           head: Box <Mexp <'a>>,
           tail: Box <Mexp <'a>> },
    Apply { span: Span,
            head: Box <Mexp <'a>>,
            arg: MexpArg <'a> },
    Array { span: Span, body: Vec <Mexp <'a>> },
    Arrow { span: Span,
            ante: Vec <Mexp <'a>>,
            ret: Box <Mexp <'a>> },
    /// infix are all left-associative
    Infix {
        span: Span,
        op: &'a str,
        lhs: Box <Mexp <'a>>,
        rhs: Box <Mexp <'a>> },
    InfixApply {
        span: Span,
        op: &'a str,
        lhs: Box <Mexp <'a>>,
        arg: MexpArg <'a> },
}

impl <'a> Mexp <'a> {
    pub fn span (&self) -> Span {
        match self {
            Mexp::Str   { span, .. } => span.clone (),
            Mexp::Char  { span, .. } => span.clone (),
            Mexp::Sym   { span, .. } => span.clone (),
            Mexp::Dot   { span, .. } => span.clone (),
            Mexp::Apply { span, .. } => span.clone (),
            Mexp::Array { span, .. } => span.clone (),
            Mexp::Arrow { span, .. } => span.clone (),
            Mexp::Infix { span, .. } => span.clone (),
            Mexp::InfixApply { span, .. } => span.clone (),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum MexpArg <'a> {
    Tuple { span: Span, body: Vec <Mexp <'a>> },
    Block { span: Span, body: Vec <Mexp <'a>> },
}

impl <'a> MexpArg <'a> {
    pub fn span (&self) -> Span {
        match self {
            MexpArg::Tuple { span, .. } => span.clone (),
            MexpArg::Block { span, .. } => span.clone (),
        }
    }
}

impl <'a> MexpArg <'a> {
    pub fn body (&self) -> Vec <Mexp <'a>> {
        match self {
            MexpArg::Tuple { body, .. } => body.clone (),
            MexpArg::Block { body, .. } => body.clone (),
        }
    }
}

impl <'a> ToString for Mexp <'a> {
    fn to_string (&self) -> String {
        match self {
            Mexp::Str   { string, .. } => {
                format! ("\"{}\"", string)
            }
            Mexp::Char  { ch, .. } => {
                format! ("'{}'", ch)
            }
            Mexp::Sym   { symbol, .. } => {
                format! ("{}", symbol)
            }
            Mexp::Dot   { head, tail, .. } => {
                format! ("{}.{}",
                         head.to_string (),
                         tail.to_string ())
            }
            Mexp::Apply { head, arg, .. } => {
                format! ("{} {}",
                         head.to_string (),
                         arg.to_string ())
            }
            Mexp::Array { body, .. } => {
                format! ("[{}]", mexp_vec_to_string (&body))
            }
            Mexp::Arrow { ante, ret, .. } => {
                format! ("-- {} -> {}",
                         mexp_vec_to_string (&ante),
                         ret.to_string ())
            }
            Mexp::Infix { op, lhs, rhs, .. } => {
                format! ("{} {} {}",
                         lhs.to_string (),
                         op,
                         rhs.to_string ())
            }
            Mexp::InfixApply { op, lhs, arg, .. } => {
                format! ("{} {} {}",
                         lhs.to_string (),
                         op,
                         arg.to_string ())
            }
        }
    }
}

impl <'a> ToString for MexpArg <'a> {
    fn to_string (&self) -> String {
        match self {
            MexpArg::Tuple { body, .. } => {
                format! ("({})",
                         mexp_vec_to_string (&body))
            }
            MexpArg::Block { body, .. } => {
                if body.is_empty () {
                    format! ("{{}}")
                } else {
                    format! ("{{ {} }}",
                             mexp_vec_to_string (&body))
                }
            }
        }
    }
}

fn mexp_vec_to_string <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> String {
    let mut string = String::new ();
    for mexp in mexp_vec {
        string += &mexp.to_string ();
        string += " ";
    }
    string.pop ();
    string
}

impl <'a> Mexp <'a> {
    pub fn to_tree_format (&self) -> String {
        match self {
            Mexp::Str   { string, .. } => {
                let mut s = String::new ();
                s += "mexp:str ";
                s += "{ ";
                s += r#"quotation ('"') "#;
                s += "{ ";
                s += &format! ("\"{}\"", string);
                s += " ";
                s += "} ";
                s += "}";
                s
            }
            Mexp::Char  { ch, .. } => {
                let mut s = String::new ();
                s += "mexp:char ";
                s += "{ ";
                s += r#"quotation ('\'') "#;
                s += "{ ";
                s += &format! ("'{}' ", ch);
                s += " ";
                s += "} ";
                s += "}";
                s
            }
            Mexp::Sym   { symbol, .. } => {
                let mut s = String::new ();
                s += "mexp:sym ";
                s += "{ ";
                s += &format! ("\"{}\"", symbol);
                s += " ";
                s += "}";
                s
            }
            Mexp::Dot   { head, tail, .. } => {
                let mut s = String::new ();
                s += "mexp:dot ";
                s += "{ ";
                s += &head.to_tree_format ();
                s += " ";
                s += "'.' ";
                s += &tail.to_tree_format ();
                s += " ";
                s += "}";
                s
            }
            Mexp::Apply { head, arg, .. } => {
                let mut s = String::new ();
                s += "mexp:apply ";
                s += "{ ";
                s += &head.to_tree_format ();
                s += " ";
                s += &arg.to_tree_format ();
                s += " ";
                s += "}";
                s
            }
            Mexp::Array { body, .. } => {
                let mut s = String::new ();
                s += "mexp:array ";
                s += "{ ";
                s += "'[' ";
                s += &mexp_vec_to_tree_fromat (&body);
                s += " ";
                s += "']' ";
                s += "}";
                s
            }
            Mexp::Arrow { ante, ret, .. } => {
                let mut s = String::new ();
                s += "mexp:arrow ";
                s += "{ ";
                s += r#""--" "#;
                s += &mexp_vec_to_tree_fromat (&ante);
                s += " ";
                s += r#""->" "#;
                s += &ret.to_tree_format ();
                s += " ";
                s += "}";
                s
            }
            Mexp::Infix { op, lhs, rhs, .. } => {
                let mut s = String::new ();
                s += "mexp:infix ";
                s += "{ ";
                s += &lhs.to_tree_format ();
                s += " ";
                s += &format! ("\"{}\"", op);
                s += " ";
                s += &rhs.to_tree_format ();
                s += " ";
                s += "}";
                s
            }
            Mexp::InfixApply { op, lhs, arg, .. } => {
                let mut s = String::new ();
                s += "mexp:infix-apply ";
                s += "{ ";
                s += &lhs.to_tree_format ();
                s += " ";
                s += &format! ("\"{}\"", op);
                s += " ";
                s += &arg.to_tree_format ();
                s += " ";
                s += "}";
                s
            }
        }
    }
}

impl <'a> MexpArg <'a> {
    pub fn to_tree_format (&self) -> String {
        match self {
            MexpArg::Tuple { body, .. } => {
                let mut s = String::new ();
                s += "arg:tuple ";
                s += "{ ";
                s += "'(' ";
                s += &mexp_vec_to_tree_fromat (&body);
                s += " ";
                s += "')' ";
                s += "}";
                s
            }
            MexpArg::Block { body, .. } => {
                let mut s = String::new ();
                s += "arg:block ";
                s += "{ ";
                s += "'{' ";
                s += &mexp_vec_to_tree_fromat (&body);
                s += " ";
                s += "'}' ";
                s += "}";
                s
            }
        }
    }
}

fn mexp_vec_to_tree_fromat <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> String {
    let mut string = String::from ("list:null (mexp) {}");
    for mexp in mexp_vec.iter () .rev () {
        let mut s = String::new ();
        s += "list:cons (mexp) ";
        s += "{ ";
        s += &mexp.to_tree_format ();
        s += " ";
        s += &string;
        s += " ";
        s += "}";
        string = s;
    }
    string
}

impl <'a> Mexp <'a> {
    pub fn to_pretty_string (&self) -> String {
        self.to_string_indent (0)
    }
}

const INDENT_UNIT: usize = 2;

fn indent (level: usize) -> String {
   let space = " ";
   space.repeat (level * INDENT_UNIT)
}

impl <'a> Mexp <'a> {
    fn to_string_indent (&self, level: usize) -> String {
        match self {
            Mexp::Str   { string, .. } => {
                format! ("\"{}\"", string)
            }
            Mexp::Char  { ch, .. } => {
                format! ("'{}'", ch)
            }
            Mexp::Sym   { symbol, .. } => {
                format! ("{}", symbol)
            }
            Mexp::Dot   { head, tail, .. } => {
                format! ("{}.{}",
                         head.to_string (),
                         tail.to_string ())
            }
            Mexp::Apply { head, arg, .. } => {
                format! ("{} {}",
                         head.to_string (),
                         arg.to_string_indent (level))
            }
            Mexp::Array { body, .. } => {
                format! ("[{}]", mexp_vec_to_string (&body))
            }
            Mexp::Arrow { ante, ret, .. } => {
                format! ("-- {} -> {}",
                         mexp_vec_to_string (&ante),
                         ret.to_string ())
            }
            Mexp::Infix { op, lhs, rhs, .. } => {
                format! ("{} {} {}",
                         lhs.to_string (),
                         op,
                         rhs.to_string_indent (level))
            }
            Mexp::InfixApply { op, lhs, arg, .. } => {
                format! ("{} {} {}",
                         lhs.to_string (),
                         op,
                         arg.to_string_indent (level))
            }
        }
    }
}

impl <'a> MexpArg <'a> {
    fn to_string_indent (&self, level: usize) -> String {
        match self {
            MexpArg::Tuple { body, .. } => {
                format! ("({})",
                         mexp_vec_to_string (&body))
            }
            MexpArg::Block { body, .. } => {
                if body.is_empty () {
                    format! ("{{}}")
                } else {
                    format! (
                        "{{\n{}\n{}}}",
                        mexp_vec_to_string_indent (
                            &body, level + 1),
                        indent (level))
                }
            }
        }
    }
}

fn mexp_vec_to_string_indent <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
    level: usize,
) -> String {
    let mut string = String::new ();
    for mexp in mexp_vec {
        string += &indent (level);
        string += &mexp.to_string_indent (level);
        string += "\n";
    }
    string.pop ();
    string
}

fn extend_op_set_by_inferring <'a> (
    op_set: &mut HashSet <String>,
    mexp: &Mexp <'a>,
) {
    match mexp {
        Mexp::Dot   { head, .. } => {
            extend_op_set_by_inferring (op_set, head);
        }
        Mexp::Apply { head, arg, .. } => {
            extend_op_set_by_inferring (op_set, head);
            for mexp in arg.body () {
                extend_op_set_by_inferring (op_set, &mexp);
            }
        }
        Mexp::Array { body, .. } => {
            for mexp in body {
                extend_op_set_by_inferring (op_set, &mexp);
            }
        }
        Mexp::Arrow { ante, ret, .. } => {
            for mexp in ante {
                extend_op_set_by_inferring (op_set, &mexp);
            }
            extend_op_set_by_inferring (op_set, ret);
        }
        Mexp::Infix { op, lhs, rhs, .. } => {
            op_set.insert (op.to_string ());
            extend_op_set_by_inferring (op_set, lhs);
            extend_op_set_by_inferring (op_set, rhs);
        }
        Mexp::InfixApply { op, lhs, arg, .. } => {
            op_set.insert (op.to_string ());
            extend_op_set_by_inferring (op_set, lhs);
            for mexp in arg.body () {
                extend_op_set_by_inferring (op_set, &mexp);
            }
        }
        _ => {}
    }
}

impl <'a> Mexp <'a> {
    pub fn to_pretty_tree_format (&self) -> String {
        let tree_string = self.to_tree_format ();
        let mut syntax_table = SyntaxTable::new ();
        extend_op_set_by_inferring (
            &mut syntax_table.op_set,
            self);
        let mut mexp_vec = syntax_table
            .parse (&tree_string)
            .unwrap ();
        assert! (mexp_vec.len () == 1);
        let mexp = mexp_vec.pop () .unwrap ();
        mexp.to_pretty_string ()
    }
}

impl <'a> Mexp <'a> {
    pub fn prettify (
        input: &str
    ) -> Result <String, ErrorInCtx> {
        let syntax_table = SyntaxTable::default ();
        let mexp_vec = syntax_table.parse (input)?;
        let mut output = String::new ();
        for mexp in mexp_vec {
            output += &mexp.to_pretty_string ();
            output += "\n";
        }
        Ok (output)
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct SyntaxTable {
   pub op_set: HashSet <String>,
   pub char_table: CharTable,
}

impl SyntaxTable {
    pub fn new () -> Self {
        let char_table = CharTable::new ()
            .quotation_mark ('"')
            .quotation_mark ('\'')
            .space (' ')
            .space ('\n') .space ('\t')
            .space (',')
            .char ('(') .char (')')
            .char ('[') .char (']')
            .char ('{') .char ('}')
            .char ('.')
            .char (';');
        SyntaxTable {
            op_set: HashSet::new (),
            char_table,
        }
    }
}

impl SyntaxTable {
    pub fn op (mut self, op: &str) -> Self {
        self.op_set.insert (op.to_string ());
        self
    }
}

impl <'a> SyntaxTable {
    pub fn parse (
        &self, input: &'a str,
    ) -> Result <Vec <Mexp <'a>>, ErrorInCtx> {
        let parsing = Parsing {
            cursor: 0,
            syntax_table: self.clone (),
            token_vec: self.char_table.lex (input)?,
            result_stack: Vec::new (),
        };
        parsing.run ()
    }
}

impl Default for SyntaxTable {
    fn default () -> Self {
        SyntaxTable::new ()
            .op ("=")
            .op (":") .op ("::")
            .op ("=>") .op ("<=")
    }
}

impl <'a> SyntaxTable {
    fn arrow_start_token_p (&self, token: &Token <'a>) -> bool {
        if let Token::Word { word, .. } = token {
            word == &"--"
        } else {
            false
        }
    }
    fn arrow_end_token_p (&self, token: &Token <'a>) -> bool {
        if let Token::Word { word, .. } = token {
            word == &"->"
        } else {
            false
        }
    }
    fn op_token_p (&self, token: &Token <'a>) -> bool {
        if let Token::Word { word, .. } = token {
            self.op_set.contains (*word)
        } else {
            false
        }
    }
    fn str_token_p (&self, token: &Token <'a>) -> bool {
        if let Token::Quotation { quotation_mark, .. } = token {
            quotation_mark == &'"'
        } else {
            false
        }
    }
    fn char_token_p (&self, token: &Token <'a>) -> bool {
        if let Token::Quotation { quotation_mark, .. } = token {
            quotation_mark == &'\''
        } else {
            false
        }
    }
    fn sym_token_p (&self, token: &Token <'a>) -> bool {
        if let Token::Word { .. } = token {
            ! (self.arrow_start_token_p (token) ||
               self.arrow_end_token_p (token) ||
               self.op_token_p (token))
        } else {
            false
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
struct Parsing <'a> {
    cursor: usize,
    syntax_table: SyntaxTable,
    token_vec: Vec <Token <'a>>,
    result_stack: Vec <Result <Mexp <'a>, Token <'a>>>,
}

impl <'a> Parsing <'a> {
    fn run (mut self) -> Result <Vec <Mexp <'a>>, ErrorInCtx> {
        loop {
            if self.finished_p () {
                return Ok (self.mexp_vec_as_result ());
            }
            self.collect_mexp ()?;
        }
    }
}

impl <'a> Parsing <'a> {
    fn mexp_vec_as_result (self) -> Vec <Mexp <'a>> {
        let mut mexp_vec = Vec::new ();
        for result in self.result_stack {
            if let Ok (mexp) = result {
                mexp_vec.push (mexp);
            }
        }
        mexp_vec
    }
}

impl <'a> Parsing <'a> {
    fn finished_p (&self) -> bool {
        self.cursor == self.token_vec.len ()
    }
}

fn arg_start_char_p (ch: char) -> bool {
    (ch == '(' ||
     ch == '{')
}

impl <'a> Parsing <'a> {
    fn str_p (&self) -> Result <bool, ErrorInCtx> {
        if let Some (token) = self.token_vec.get (self.cursor) {
            Ok (self.syntax_table.str_token_p (token))
        } else {
            Ok (false)
        }
    }
}

impl <'a> Parsing <'a> {
    fn char_p (&self) -> Result <bool, ErrorInCtx> {
        if let Some (token) = self.token_vec.get (self.cursor) {
            Ok (self.syntax_table.char_token_p (token))
        } else {
            Ok (false)
        }
    }
}

impl <'a> Parsing <'a> {
    fn sym_p (&self) -> Result <bool, ErrorInCtx> {
        if let Some (token) = self.token_vec.get (self.cursor) {
            Ok (self.syntax_table.sym_token_p (token))
        } else {
            Ok (false)
        }
    }
}

fn note_about_surely_exp () -> ErrorMsg {
    ErrorMsg::new ()
        .line ("mexp in this context \
                should be one of the following :")
        .line ("- mexp:sym mexp:dot mexp:apply")
}

impl <'a> Parsing <'a> {
    fn dot_p (&self) -> Result <bool, ErrorInCtx> {
        if let Some (Token::Char {
            ch: '.', span, ..
        }) = self.token_vec.get (self.cursor) {
            match self.result_stack.last () {
                Some (Ok (Mexp::Sym { .. })) |
                Some (Ok (Mexp::Dot { .. })) |
                Some (Ok (Mexp::Apply { .. })) => {
                    Ok (true)
                }
                Some (Ok (mexp)) => {
                    ErrorInCtx::new ()
                        .head ("invalid-mexp-before-dot")
                        .span (mexp.span ())
                        .note (note_about_surely_exp ())
                        .wrap_in_err ()
                }
                Some (Err (token)) => {
                    ErrorInCtx::new ()
                        .head ("invalid-token-before-dot")
                        .span (token.span ())
                        .wrap_in_err ()
                }
                None => {
                    ErrorInCtx::new ()
                        .head ("no-token-before-dot")
                        .span (span.clone ())
                        .wrap_in_err ()
                }
            }
        } else {
            Ok (false)
        }
    }
}

impl <'a> Parsing <'a> {
    fn apply_p (&self) -> Result <bool, ErrorInCtx> {
        if let Some (Token::Char {
            ch, span, ..
        }) = self.token_vec.get (self.cursor) {
            if arg_start_char_p (*ch) {
                match self.result_stack.last () {
                    Some (Ok (Mexp::Sym { .. })) |
                    Some (Ok (Mexp::Dot { .. })) |
                    Some (Ok (Mexp::Apply { .. })) => {
                        Ok (true)
                    }
                    Some (Ok (mexp)) => {
                        ErrorInCtx::new ()
                            .head ("invalid-mexp-before-arg")
                            .span (mexp.span ())
                            .note (note_about_surely_exp ())
                            .wrap_in_err ()
                    }
                    Some (Err (token)) => {
                        ErrorInCtx::new ()
                            .head ("invalid-token-before-arg")
                            .span (token.span ())
                            .wrap_in_err ()
                    }
                    None => {
                        ErrorInCtx::new ()
                            .head ("no-token-before-arg")
                            .span (span.clone ())
                            .wrap_in_err ()
                    }
                }
            } else {
                Ok (false)
            }
        } else {
            Ok (false)
        }
    }
}

impl <'a> Parsing <'a> {
    fn array_p (&self) -> Result <bool, ErrorInCtx> {
        if let Some (Token::Char {
            ch, ..
        }) = self.token_vec.get (self.cursor) {
            Ok (ch == &'[')
        } else {
            Ok (false)
        }
    }
}

impl <'a> Parsing <'a> {
    fn arrow_p (&self) -> Result <bool, ErrorInCtx> {
        if let Some (token) = self.token_vec.get (self.cursor) {
            Ok (self.syntax_table.arrow_start_token_p (token))
        } else {
            Ok (false)
        }
    }
}

impl <'a> Parsing <'a> {
    fn infix_p (&self) -> Result <bool, ErrorInCtx> {
        if let Some (token) = self.token_vec.get (self.cursor) {
            if self.syntax_table.op_token_p (token) {
                if let Some (Token::Char {
                    ch, ..
                }) = self.token_vec.get (self.cursor + 1) {
                    Ok (! arg_start_char_p (*ch))
                } else {
                    Ok (true)
                }
            } else {
                Ok (false)
            }
        } else {
            Ok (false)
        }
    }
}

impl <'a> Parsing <'a> {
    fn infix_apply_p (&self) -> Result <bool, ErrorInCtx> {
        if let Some (token) = self.token_vec.get (self.cursor) {
            if self.syntax_table.op_token_p (token) {
                if let Some (Token::Char {
                    ch, ..
                }) = self.token_vec.get (self.cursor + 1) {
                    Ok (arg_start_char_p (*ch))
                } else {
                    Ok (false)
                }
            } else {
                Ok (false)
            }
        } else {
            Ok (false)
        }
    }
}

impl <'a> Parsing <'a> {
    fn collect_str (&mut self) -> Result <(), ErrorInCtx> {
        if let Some (Token::Quotation {
            span, string, ..
        }) = self.token_vec.get (self.cursor) {
            self.cursor += 1;
            let mexp = Mexp::Str {
                span: span.clone (),
                string,
            };
            self.result_stack.push (Ok (mexp));
            Ok (())
        } else {
            ErrorInCtx::new ()
                .head ("internal-error")
                .line ("Parsing::collect_str")
                .wrap_in_err ()
        }
    }
}

impl <'a> Parsing <'a> {
    fn collect_char (&mut self) -> Result <(), ErrorInCtx> {
        if let Some (Token::Quotation {
            span, string, ..
        }) = self.token_vec.get (self.cursor) {
            self.cursor += 1;
            let mexp = Mexp::Char {
                span: span.clone (),
                ch: string,
            };
            self.result_stack.push (Ok (mexp));
            Ok (())
        } else {
            ErrorInCtx::new ()
                .head ("internal-error")
                .line ("Parsing::collect_char")
                .wrap_in_err ()
        }
    }
}

impl <'a> Parsing <'a> {
    fn collect_sym (&mut self) -> Result <(), ErrorInCtx> {
        if let Some (Token::Word {
            span, word
        }) = self.token_vec.get (self.cursor) {
            self.cursor += 1;
            let mexp = Mexp::Sym {
                span: span.clone (),
                symbol: word,
            };
            self.result_stack.push (Ok (mexp));
            Ok (())
        } else {
            ErrorInCtx::new ()
                .head ("internal-error")
                .line ("Parsing::collect_sym")
                .wrap_in_err ()
        }
    }
}

impl <'a> Parsing <'a> {
    fn collect_dot (&mut self) -> Result <(), ErrorInCtx> {
        self.cursor += 1;
        if let Err (error) = self.collect_sym () {
            if let Some (
                token
            ) = self.token_vec.get (self.cursor) {
                return error
                    .head ("non-symbol-after-dot")
                    .span (Span {
                        lo: token.span () .lo,
                        hi: token.span () .hi,
                    })
                    .wrap_in_err ();
            } else {
                let span = self.token_vec.get (self.cursor-1)
                    .unwrap ()
                    .span ();
                return error
                    .head ("no-token-after-dot")
                    .span (span)
                    .wrap_in_err ();
            }
        }
        let tail = self.result_stack.pop ()
            .unwrap () .unwrap ();
        let head = self.result_stack.pop ()
            .unwrap () .unwrap ();
        let mexp = Mexp::Dot {
            span: Span {
                lo: head.span () .lo,
                hi: tail.span () .hi,
            },
            head: box head,
            tail: box tail,
        };
        self.result_stack.push (Ok (mexp));
        Ok (())
    }
}

impl <'a> Parsing <'a> {
    fn collect_apply (&mut self) -> Result <(), ErrorInCtx> {
        let arg = self.get_arg ()?;
        let head = self.result_stack.pop ()
            .unwrap () .unwrap ();
        let mexp = Mexp::Apply {
            span: Span {
                lo: head.span () .lo,
                hi: arg.span () .hi,
            },
            head: box head,
            arg,
        };
        self.result_stack.push (Ok (mexp));
        Ok (())
    }
}

impl <'a> Parsing <'a> {
    fn get_arg (
        &mut self,
    ) -> Result <MexpArg <'a>, ErrorInCtx> {
        match self.token_vec.get (self.cursor) {
            Some (Token::Char { ch: '(', .. }) => {
                let (body, span) = self.get_body_and_span ()?;
                let arg = MexpArg::Tuple { span, body };
                Ok (arg)
            }
            Some (Token::Char { ch: '{', .. }) => {
                let (body, span) = self.get_body_and_span ()?;
                let arg = MexpArg::Block { span, body };
                Ok (arg)
            }
            Some (token) => {
                ErrorInCtx::new ()
                    .head ("Parsing::get_arg")
                    .line ("expecting '(' or '{'")
                    .span (token.span ())
                    .wrap_in_err ()
            }
            None => {
                ErrorInCtx::new ()
                    .head ("internal-error")
                    .line ("Parsing::get_arg")
                    .wrap_in_err ()
            }
        }
    }
}

impl <'a> Parsing <'a> {
    fn get_body_and_span (
        &mut self,
    ) -> Result <(Vec <Mexp <'a>>, Span), ErrorInCtx> {
        let token = self.token_vec.get (self.cursor)
            .unwrap () .clone ();
        self.cursor += 1;
        let bra = if let Token::Char {
            ch, ..
        } = token {
            ch
        } else {
            return ErrorInCtx::new ()
                .head ("Parsing::get_body_and_span")
                .line ("expecting bra char")
                .span (token.span ())
                .wrap_in_err ();
        };
        let lo = token.span () .lo;
        let bra_span = token.span () .clone ();
        self.result_stack.push (Err (token));
        loop {
            if self.finished_p () {
                return ErrorInCtx::new ()
                    .head ("ket-mismatching")
                    .span (bra_span)
                    .wrap_in_err ();
            }
            if let Some (Token::Char {
                span, ch, ..
            }) = self.token_vec.get (self.cursor) {
                if *ch == ket_of_bra (bra)? {
                    let hi = span.hi;
                    self.cursor += 1;
                    return Ok ((
                        self.mexp_vec_before_char (bra)?,
                        Span { lo, hi },
                    ));
                } else {
                    self.collect_mexp ()?;
                }
            } else {
                self.collect_mexp ()?;
            }
        }
    }
}

fn ket_of_bra (bra: char) -> Result <char, ErrorInCtx> {
    if bra == '(' {
        Ok (')')
    } else if bra == '[' {
        Ok (']')
    } else if bra == '{' {
        Ok ('}')
    } else {
        ErrorInCtx::new ()
            .head ("internal-error")
            .line ("Parsing::ket_of_bra")
            .line ("char is not bra")
            .line (&format! ("char = {}", bra))
            .wrap_in_err ()
    }
}

impl <'a> Parsing <'a> {
    fn mexp_vec_before_char (
        &mut self,
        ch: char,
    ) -> Result <Vec <Mexp <'a>>, ErrorInCtx> {
        let mut mexp_vec = Vec::new ();
        while let Some (result) = self.result_stack.pop () {
            match result {
                Err (Token::Char { ch: tos_ch, .. }) => {
                    if tos_ch == ch {
                        let mexp_vec = mexp_vec.into_iter ()
                            .rev () .collect ();
                        return Ok (mexp_vec);
                    } else {
                        return ErrorInCtx::new ()
                            .head ("internal-error")
                            .line ("Parsing::mexp_vec_before_char")
                            .line ("expecting char in result_stack")
                            .line (&format! ("char = {}", ch))
                            .line (&format! (
                                "found char: {:?}",
                                tos_ch))
                            .wrap_in_err ()
                    }
                }
                Err (token) => {
                    return ErrorInCtx::new ()
                        .head ("internal-error")
                        .line ("Parsing::mexp_vec_before_char")
                        .line ("expecting char in result_stack")
                        .line (&format! ("char = {}", ch))
                        .line (&format! (
                            "found token: {:?}",
                            token))
                        .wrap_in_err ()
                }
                Ok (mexp) => {
                    mexp_vec.push (mexp);
                }
            }
        }
        ErrorInCtx::new ()
            .head ("internal-error")
            .head ("Parsing::mexp_vec_before_char")
            .line ("expecting char in result_stack")
            .line (&format! ("char = {}", ch))
            .line ("but result_stack is empty")
            .wrap_in_err ()
    }
}

impl <'a> Parsing <'a> {
    fn mexp_vec_before_word (
        &mut self,
        word: &'a str,
    ) -> Result <Vec <Mexp <'a>>, ErrorInCtx> {
        let mut mexp_vec = Vec::new ();
        while let Some (result) = self.result_stack.pop () {
            match result {
                Err (Token::Word { word: tos_word, .. }) => {
                    if tos_word == word {
                        let mexp_vec = mexp_vec.into_iter ()
                            .rev () .collect ();
                        return Ok (mexp_vec);
                    } else {
                        return ErrorInCtx::new ()
                            .head ("internal-error")
                            .line ("Parsing::mexp_vec_before_word")
                            .line ("expecting word in result_stack")
                            .line (&format! ("word = {}", word))
                            .line (&format! (
                                "found word: {:?}",
                                tos_word))
                            .wrap_in_err ()
                    }
                }
                Err (token) => {
                    return ErrorInCtx::new ()
                        .head ("internal-error")
                        .line ("Parsing::mexp_vec_before_word")
                        .line ("expecting word in result_stack")
                        .line (&format! ("word = {}", word))
                        .line (&format! (
                            "found token: {:?}",
                            token))
                        .wrap_in_err ()
                }
                Ok (mexp) => {
                    mexp_vec.push (mexp);
                }
            }
        }
        ErrorInCtx::new ()
            .head ("internal-error")
            .head ("Parsing::mexp_vec_before_word")
            .line ("expecting word in result_stack")
            .line (&format! ("word = {}", word))
            .line ("but result_stack is empty")
            .wrap_in_err ()
    }
}

impl <'a> Parsing <'a> {
    fn collect_array (&mut self) -> Result <(), ErrorInCtx> {
        let (body, span) = self.get_body_and_span ()?;
        let mexp = Mexp::Array { span, body };
        self.result_stack.push (Ok (mexp));
        Ok (())
    }
}

fn note_about_syntax_of_arrow () -> ErrorMsg {
    ErrorMsg::new ()
        .head ("syntax of arrow :")
        .line ("-- mexp ... -> mexp")
}

impl <'a> Parsing <'a> {
    fn collect_arrow (&mut self) -> Result <(), ErrorInCtx> {
        let token = self.token_vec.get (self.cursor)
            .unwrap () .clone ();
        self.cursor += 1;
        let lo = token.span () .lo;
        let arrow_start_span = token.span () .clone ();
        self.result_stack.push (Err (token));
        loop {
            if self.finished_p () {
                return ErrorInCtx::new ()
                    .head ("arrow-end-mismatching")
                    .span (arrow_start_span)
                    .note (note_about_syntax_of_arrow ())
                    .wrap_in_err ();
            }
            if let Some (Token::Word {
                word, ..
            }) = self.token_vec.get (self.cursor) {
                if *word == "->" {
                    self.cursor += 1;
                    let ret = self.get_mexp ()?;
                    let ante = self.mexp_vec_before_word ("--")?;
                    let hi = ret.span () .hi;
                    let mexp = Mexp::Arrow {
                        span: Span { lo, hi },
                        ante,
                        ret: box ret,
                    };
                    self.result_stack.push (Ok (mexp));
                    return Ok (());
                } else {
                    self.collect_mexp ()?;
                }
            } else {
                self.collect_mexp ()?;
            }
        }
    }
}

impl <'a> Parsing <'a> {
    fn collect_infix (&mut self) -> Result <(), ErrorInCtx> {
        if let Some (Token::Word {
            word, ..
        }) = self.token_vec.get (self.cursor) {
            let word = word.clone ();
            self.cursor += 1;
            let rhs = self.get_mexp ()?;
            let lhs = self.result_stack.pop ()
                .unwrap () .unwrap ();
            let mexp = Mexp::Infix {
                span: Span {
                    lo: lhs.span () .lo,
                    hi: rhs.span () .hi,
                },
                op: word,
                lhs: box lhs,
                rhs: box rhs,
            };
            self.result_stack.push (Ok (mexp));
            Ok (())
        } else {
            ErrorInCtx::new ()
                .head ("internal-error")
                .line ("Parsing::collect_infix")
                .wrap_in_err ()
        }
    }
}

impl <'a> Parsing <'a> {
    fn collect_infix_apply (&mut self) -> Result <(), ErrorInCtx> {
        if let Some (Token::Word {
            word, ..
        }) = self.token_vec.get (self.cursor) {
            let word = word.clone ();
            self.cursor += 1;
            let arg = self.get_arg ()?;
            let lhs = self.result_stack.pop ()
                .unwrap () .unwrap ();
            let mexp = Mexp::InfixApply {
                span: Span {
                    lo: lhs.span () .lo,
                    hi: arg.span () .hi,
                },
                op: word,
                lhs: box lhs,
                arg,
            };
            self.result_stack.push (Ok (mexp));
            Ok (())
        } else {
            ErrorInCtx::new ()
                .head ("internal-error")
                .line ("Parsing::collect_infix_apply")
                .wrap_in_err ()
        }
    }
}

impl <'a> Parsing <'a> {
    fn collect_mexp (
        &mut self,
    ) -> Result <(), ErrorInCtx> {
        if      self.str_p   ()? { self.collect_str   () }
        else if self.char_p  ()? { self.collect_char  () }
        else if self.sym_p   ()? { self.collect_sym   () }
        else if self.dot_p   ()? { self.collect_dot   () }
        else if self.apply_p ()? { self.collect_apply () }
        else if self.array_p ()? { self.collect_array () }
        else if self.arrow_p ()? { self.collect_arrow () }
        else if self.infix_p ()? { self.collect_infix () }
        else if self.infix_apply_p ()? { self.collect_infix_apply () }
        else {
            ErrorInCtx::new ()
                .head ("internal-error")
                .line ("Parsing::collect_mexp")
                .wrap_in_err ()
        }
    }
}

impl <'a> Parsing <'a> {
    fn get_mexp (
        &mut self,
    ) -> Result <Mexp <'a>, ErrorInCtx> {
        self.collect_mexp ()?;
        loop {
            if      self.dot_p   ()? { self.collect_dot   ()?; }
            else if self.apply_p ()? { self.collect_apply ()?; }
            else if self.infix_p ()? { self.collect_infix ()?; }
            else if self.infix_apply_p ()? { self.collect_infix_apply ()?; }
            else {
                let mexp = self.result_stack.pop ()
                    .unwrap () .unwrap ();
                return Ok (mexp);
            }
        }
    }
}

fn assert_parse_mexp_sentence (sentence: &str) {
    let syntax_table = SyntaxTable::default ();
    let mut mexp_vec = syntax_table.parse (sentence) .unwrap ();
    assert! (mexp_vec.len () == 1);
    let mexp = mexp_vec.pop () .unwrap ();
    // println! ("- {}", mexp.to_tree_format ());
    if ! lexing::token_vec_eq (
        &syntax_table.char_table
            .lex (&mexp.to_string ())
            .unwrap (),
        &syntax_table.char_table
            .lex (sentence)
            .unwrap (),
    ) {
        eprintln! ("- assert_parse_mexp_sentence");
        eprintln! ("  result = {}", mexp.to_string ());
        eprintln! ("  assert = {}", sentence);
        panic! ("assert_parse_mexp_sentence");
    }
}

fn assert_parse_mexp_to_tree_format (
    sentence: &str,
    tree_format: &str,
) {
    let syntax_table = SyntaxTable::default ();
    let mut mexp_vec = syntax_table.parse (sentence) .unwrap ();
    assert! (mexp_vec.len () == 1);
    let mexp = mexp_vec.pop () .unwrap ();
    if ! lexing::token_vec_eq (
        &syntax_table.char_table
            .lex (&mexp.to_tree_format ())
            .unwrap (),
        &syntax_table.char_table
            .lex (tree_format)
            .unwrap (),
    ) {
        eprintln! ("- assert_parse_mexp_to_tree_format");
        eprintln! ("  result = {}", mexp.to_tree_format ());
        eprintln! ("  assert = {}", tree_format);
        panic! ("assert_parse_mexp_to_tree_format");
    }
}

fn print_mexp_sentence (sentence: &str) {
    let syntax_table = SyntaxTable::default ();
    let mut mexp_vec = syntax_table.parse (sentence) .unwrap ();
    assert! (mexp_vec.len () == 1);
    let mexp = mexp_vec.pop () .unwrap ();
    println! ("{}", mexp.to_string ());
}

#[test]
fn test_parsing () {
    assert_parse_mexp_sentence (r#"
    a
    "#);

    assert_parse_mexp_sentence (r#"
    "b"
    "#);

    assert_parse_mexp_sentence (r#"
    '符'
    "#);

    assert_parse_mexp_sentence (r#"
    a.b.c
    "#);

    assert_parse_mexp_sentence (r#"
    [a b c]
    "#);

    assert_parse_mexp_sentence (r#"
    a (x)
    "#);

    assert_parse_mexp_sentence (r#"
    a.b.c (x y z)
    "#);

    assert_parse_mexp_sentence (r#"
    a.b.c {
      a : b
    } {
      a = b
      a = b
    }
    "#);

    assert_parse_mexp_sentence (r#"
    a = {
      a = b
    }
    "#);

    assert_parse_mexp_sentence (r#"
    a (x) = {
      a = b
    }
    "#);

    assert_parse_mexp_sentence (r#"
    -- a : t
       b : t
       c
    -> d
    "#);

    assert_parse_mexp_sentence (r#"
    nat-add (x y)
    "#);

    assert_parse_mexp_sentence (r#"
    [x y] : nat-t
    "#);

    assert_parse_mexp_sentence (r#"
    eqv-t (
       nat-add (x y)
       nat-add (y x))
    "#);

    assert_parse_mexp_sentence (r#"
    -- a -> b (c)
    "#);

    assert_parse_mexp_sentence (r#"
    -- a -> b ()
    "#);

    assert_parse_mexp_sentence (r#"
    nat-add-commutative :
      -- [x y] : nat-t
      -> eqv-t (
           nat-add (x y)
           nat-add (y x))
    "#);

    assert_parse_mexp_sentence (r#"
    nat-add-commutative (x y) = case (x) {
      zero-t => nat-add-zero-commutative (y)
      succ-t => eqv-compose (
        eqv-apply (succ-c nat-add-commutative (x.prev y))
        nat-add-succ-commutative (y x.prev))
    }
    "#);

    assert_parse_mexp_sentence (r#"
    data {
      t : type-tt
      car : t
      cdr : list-t (t)
    }
    "#);

    assert_parse_mexp_sentence (r#"
    cons-t = data {
      t : type-tt
      car : t
      cdr : list-t (t)
    }
    "#);

    assert_parse_mexp_sentence (r#"
    [
    mexp:str  = { quotation ('"') }
    mexp:char = { quotation ('\'') }
    mexp:sym  = { symbol? }
    mexp:dot   = { [ mexp:sym mexp:dot mexp:apply ] '.' mexp:sym }
    mexp:apply = { [ mexp:sym mexp:dot mexp:apply ] arg }
    mexp:array = { '[' list (mexp) ']' }
    mexp:arrow = { "--" list (mexp) "->" mexp }
    mexp:infix       = { mexp op? mexp }
    mexp:infix-apply = { mexp op? arg }

    arg:tuple = { '(' list (mexp) ')' }
    arg:block = { '{' list (mexp) '}' }

    list : -- Gr -> Gr
    list:null (t) = {}
    list:cons (t) = { t list (t) }
    ]
    "#);

    // > mexp:sym = { symbol? }
    assert_parse_mexp_sentence (r#"
    mexp:infix {
      mexp:sym { "mexp:sym" }
      "="
      arg:block {
        '{'
        list:cons (mexp) {
           mexp:sym { "symbol?" }
           list:null (mexp) {}
        }
        '}'
      }
    }
    "#);
}

#[test]
fn test_to_tree_format () {
    assert_parse_mexp_to_tree_format (
        r#"
        mexp:sym = { symbol? }
        "#,
        r#"
        mexp:infix-apply {
          mexp:sym { "mexp:sym" }
          "="
          arg:block {
            '{'
            list:cons (mexp) {
               mexp:sym { "symbol?" }
               list:null (mexp) {}
            }
            '}'
          }
        }
        "#);

    assert_parse_mexp_to_tree_format (
        r#"
        list-t = data {
          t : type-tt
        }
        "#,
        r#"
        mexp:infix {
          mexp:sym { "list-t" }
          "="
          mexp:apply {
            mexp:sym { "data" }
            arg:block {
              '{'
              list:cons (mexp) {
                mexp:infix {
                  mexp:sym { "t" }
                  ":"
                  mexp:sym { "type-tt" }
                }
                list:null (mexp) {}
              }
              '}'
            }
          }
        }
        "#);
}
