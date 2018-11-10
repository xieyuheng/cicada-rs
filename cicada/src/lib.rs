#![feature (uniform_paths)]
#![feature (bind_by_move_pattern_guards)]
#![feature (box_patterns)]
#![feature (box_syntax)]

#![allow (dead_code)]

use std::fmt;
use std::sync::Arc;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;
use dic::Dic;
use error_report::{
    Span,
    ErrorMsg,
    ErrorInCtx,
};
#[cfg (test)]
use error_report::{
    ErrorCtx,
};
use mexp::{
    SyntaxTable,
    Mexp,
    MexpArg,
};

fn vec_to_string <T> (vec: &Vec <T>, delimiter: &str) -> String
where T : ToString {
    let mut s = String::new ();
    for x in vec {
        s += &x.to_string ();
        s += delimiter;
    }
    for _ in 0 .. delimiter.len () {
        s.pop ();
    }
    s
}

fn vec_to_lines <T> (vec: &Vec <T>) -> String
where T : ToString {
    let mut s = vec_to_string (vec, "\n");
    if ! s.is_empty () {
        s += "\n";
    }
    s
}

fn dic_to_string <T> (dic: &Dic <T>) -> String
where T : ToString {
    let mut s = String::new ();
    for (k, v) in dic.iter () {
        s += &k.to_string ();
        s += " = ";
        s += &v.to_string ();
        s += ", ";
    }
    for _ in 0 .. ", ".len () {
        s.pop ();
    }
    s
}

fn dic_to_lines <T> (dic: &Dic <T>) -> String
where T : ToString {
    let mut s = String::new ();
    for (k, v) in dic.iter () {
        s += &k.to_string ();
        s += " = ";
        s += &v.to_string ();
        s += "\n";
    }
    s
}

fn add_tag (tag: &str, input: String) -> String {
    let start = tag;
    let end = &tag[1 .. tag.len () - 1];
    let end = format! ("</{}>", end);
    if input.is_empty () {
        format! ("{}{}", start, end)
    } else {
        format! ("{}\n{}{}\n", start, input, end)
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Term {
    Var (Span, Var),
    Cons (Span, String, Arg),
    Prop (Span, String, Arg),
    FieldRef (Span, String),
    TypeOfType (Span),
}

impl ToString for Term {
    fn to_string (&self) -> String {
        match self {
            Term::Var (_span, var) => {
                var.to_string ()
            }
            Term::Cons (_span, name, arg) |
            Term::Prop (_span, name, arg) => {
                if arg.is_empty () {
                    format! ("{}", name)
                } else {
                    format! ("{} {}", name, arg.to_string ())
                }
            }
            Term::FieldRef (_span, name) => {
                format! ("{}", name)
            }
            Term::TypeOfType (_span) => {
                format! ("type")
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Arg {
    Vec (Vec <Term>),
    Rec (Vec <Binding>),
}

impl Arg {
    fn is_empty (&self) -> bool {
        match self {
            Arg::Vec (term_vec) => term_vec.is_empty (),
            Arg::Rec (bind_vec) => bind_vec.is_empty (),
        }
    }
}

impl ToString for Arg {
    fn to_string (&self) -> String {
        match self {
            Arg::Vec (term_vec) => {
                format! ("({})",
                         vec_to_string (term_vec, " "))
            }
            Arg::Rec (bind_vec) => {
                format! ("{{ {} }}",
                         vec_to_string (bind_vec, ", "))
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Binding {
    EqualTo (String, Term),
    Inhabit (String, Term),
}

impl ToString for Binding {
    fn to_string (&self) -> String {
        match self {
            Binding::EqualTo (name, term) => {
                format! ("{} = {}", name, term.to_string ())
            }
            Binding::Inhabit (name, term) => {
                format! ("{} : {}", name, term.to_string ())
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub struct Var {
    name: String,
    id: Option <Id>,
}

impl Var {
    fn new (s: &str) -> Self {
        Var {
            name: s.to_string (),
            id: Some (Id::uuid ()),
        }
    }
}

impl Var {
    fn no_id (s: &str) -> Self {
        Var {
            name: s.to_string (),
            id: None,
        }
    }
}

impl Var {
    fn local (s: &str, counter: usize) -> Self {
        Var {
            name: s.to_string (),
            id: Some (Id::local (counter)),
        }
    }
}

impl ToString for Var {
    fn to_string (&self) -> String {
        let mut s = format! ("{}", self.name);
        if let Some (id) = &self.id {
            s += &format! ("#{}", id.to_string ());
        }
        s
    }
}

#[derive (Clone)]
#[derive (PartialEq, Eq, Hash)]
pub enum Id {
    Uuid (uuid::adapter::Hyphenated),
    Local (usize),
}

impl Id {
    fn uuid () -> Self {
        Id::Uuid (Uuid::new_v4 () .to_hyphenated ())
    }
}

impl Id {
    fn local (counter: usize) -> Self {
        Id::Local (counter)
    }
}

impl fmt::Debug for Id {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Id::Uuid (uuid) => write! (f, "{}", uuid),
            Id::Local (counter) => write! (f, "{}", counter),
        }
    }
}

impl ToString for Id {
    fn to_string (&self) -> String {
        match self {
            Id::Uuid (uuid) => format! ("{}", uuid),
            Id::Local (counter) => format! ("{}", counter),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Value {
    Var (Var),
    Data (String, Dic <Value>),
    TypeOfType,
}

impl ToString for Value {
    fn to_string (&self) -> String {
        match self {
            Value::Var (var) => var.to_string (),
            Value::Data (name, dic) => {
                if dic.len () == 0 {
                    format! ("{}", name)
                } else {
                    format! (
                        "{} {{ {} }}",
                        name,
                        &dic_to_string (dic))
                }
            }
            Value::TypeOfType => "type".to_string (),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Subst {
    Null,
    Cons (Var, Value, Arc <Subst>),
}

impl Subst {
    fn new () -> Self {
        Subst::Null
    }
}

impl Subst {
    fn extend (&self, var: Var, value: Value) -> Self {
        Subst::Cons (var, value, Arc::new (self.clone ()))
    }
}

impl Subst {
    pub fn find (&self, var: &Var) -> Option <&Value> {
        match self {
            Subst::Null => None,
            Subst::Cons (
                var1, value, next,
            ) => {
                if var1 == var {
                    Some (value)
                } else {
                    next.find (var)
                }
            }
        }
    }
}

impl Subst {
    pub fn walk (&self, value: &Value) -> Value {
        match value {
            Value::Var (var) => {
                if let Some (new_value) = self.find (var) {
                    self.walk (new_value)
                } else {
                    value.clone ()
                }
            }
            _ => value.clone ()
        }
    }
}

impl Subst {
    pub fn unify (
        &self,
        u: &Value,
        v: &Value,
    ) -> Option <Subst> {
        let u = self.walk (u);
        let v = self.walk (v);
        match (u, v) {
            (Value::Var (u),
             Value::Var (v),
            ) if u == v => {
                Some (self.clone ())
            }
            (Value::Var (u), v) => {
                if self.var_occur_p (&u, &v) {
                    None
                } else {
                    Some (self.extend (u, v))
                }
            }
            (u, Value::Var (v)) => {
                if self.var_occur_p (&v, &u) {
                    None
                } else {
                    Some (self.extend (v, u))
                }
            }
            (Value::Data (u_name, u_dic),
             Value::Data (v_name, v_dic),
            ) => {
                if u_name != v_name {
                    return None;
                }
                if u_dic.len () != v_dic.len () {
                    return None;
                }
                let mut subst = self.clone ();
                let zip = u_dic.entries () .zip (v_dic.entries ());
                for (u_entry, v_entry) in zip {
                    if u_entry.name != v_entry.name {
                        eprintln! ("- [warning] Subst::unify");
                        eprintln! ("  dic mismatch");
                        eprintln! ("  u_name = {}", u_name);
                        eprintln! ("  v_name = {}", v_name);
                        eprintln! ("  u_entry.name = {}", u_entry.name);
                        eprintln! ("  v_entry.name = {}", v_entry.name);
                        return None;
                    } else {
                        if let (
                            Some (u_value),
                            Some (v_value),
                        ) = (&u_entry.value, &v_entry.value) {
                            subst = subst.unify (
                                u_value,
                                v_value)?;
                        } else {
                            return None
                        }
                    }
                }
                Some (subst)
            }
            (u, v) => {
                if u == v {
                    Some (self.clone ())
                } else {
                    None
                }
            }
        }
    }
}

impl Subst {
    pub fn var_occur_p (
        &self,
        var: &Var,
        value: &Value,
    ) -> bool {
        let value = self.walk (value);
        match value {
            Value::Var (var1) => {
                var == &var1
            }
            Value::Data (_name, dic) => {
                for value in dic.values () {
                    if self.var_occur_p (var, value) {
                        return true;
                    }
                }
                return false;
            }
            _ => {
                false
            }
        }
    }
}

impl Subst {
    pub fn len (&self) -> usize {
        let mut len = 0;
        let mut subst = self;
        while let Subst::Cons (
            _var, _value, next
        ) = subst {
            len += 1;
            subst = &next;
        }
        len
    }
}

impl ToString for Subst {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        let mut subst = self.clone ();
        while let Subst::Cons (
            var, value, next,
        ) = subst {
            s += &var.to_string ();
            s += " = ";
            s += &value.to_string ();
            s += "\n";
            subst = (*next) .clone ();
        }
        add_tag ("<subst>", s)
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Den {
    Disj (Vec <String>, Vec <Binding>),
    Conj (Vec <Binding>),
}

impl ToString for Den {
    fn to_string (&self) -> String {
        match self {
            Den::Disj (name_vec, bind_vec) => {
                if bind_vec.is_empty () {
                    format! ("disj ({}) {{}}",
                             vec_to_string (name_vec, " "))
                } else {
                    format! ("disj ({}) {{ {} }}",
                             vec_to_string (name_vec, " "),
                             vec_to_string (bind_vec, ", "))
                }
            }
            Den::Conj (bind_vec) => {
                if bind_vec.is_empty () {
                    format! ("conj {{}}")
                } else {
                    format! ("conj {{ {} }}",
                             vec_to_string (bind_vec, ", "))
                }
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Wissen {
    den_dic: Dic <Den>,
}

impl Wissen {
    pub fn new () -> Self {
        Wissen {
            den_dic: Dic::new (),
        }
    }
}

impl Wissen {
    pub fn den (&mut self, name: &str, den: &Den) {
       self.den_dic.ins (name, Some (den.clone ()));
    }
}

impl Wissen {
    pub fn wis <'a> (
        &'a mut self,
        input: &str,
    ) -> Result <Vec <WissenOutput>, ErrorInCtx> {
        let syntax_table = SyntaxTable::default ();
        let mexp_vec = syntax_table.parse (input)?;
        let statement_vec = mexp_vec_to_statement_vec (&mexp_vec)?;
        for statement in &statement_vec {
            if let Statement::Den (
                name, den
            ) = statement {
                self.den (name, den);
            }
        }
        let mut output_vec = Vec::new ();
        // ><><><
        Ok (output_vec)
    }
}

impl ToString for Wissen {
    fn to_string (&self) -> String {
        add_tag ("<wissen>", dic_to_lines (&self.den_dic))
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct WissenOutput;

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Statement {
    Den (String, Den),
    PlaceHolder,
    // Query (usize, Vec <Prop>),
    // Prove (usize, Vec <Prop>),
}

const GRAMMAR: &'static str = r#"
Statement::Den = { prop-name? "=" Den }
Den::Disj = { "disj" '(' list (prop-name?) ')' Arg::Rec }
Den::Conj = { "conj" Arg::Rec }
Term::Var = { var-name? }
Term::Cons = { cons-name? Arg }
Term::Prop = { prop-name? Arg }
Arg::Vec = { '(' list (Term) ')' }
Arg::Rec = { '{' list (Binding) '}' }
Binding::Term = { field-name? "=" Term }
"#;

fn note_about_grammar () -> ErrorMsg {
    ErrorMsg::new ()
        .head ("grammar :")
        .lines (GRAMMAR)
}

fn var_symbol_p (symbol: &str) -> bool {
    (symbol.len () == 1 ||
     symbol.starts_with (":"))
}

fn cons_name_symbol_p (symbol: &str) -> bool {
    (! var_symbol_p (symbol) &&
     symbol.ends_with ("-c"))
}

fn prop_name_symbol_p (symbol: &str) -> bool {
    (! var_symbol_p (symbol) &&
     symbol.ends_with ("-t"))
}

fn type_of_type_symbol_p (symbol: &str) -> bool {
    symbol == "type"
}

fn field_name_symbol_p (symbol: &str) -> bool {
    (! var_symbol_p (symbol) &&
     ! cons_name_symbol_p (symbol) &&
     ! prop_name_symbol_p (symbol) &&
     ! type_of_type_symbol_p (symbol))
}

fn mexp_to_prop_name <'a> (
    mexp: &Mexp <'a>,
) -> Result <String, ErrorInCtx> {
    if let Mexp::Sym {
        symbol,
        ..
    } = mexp {
        if prop_name_symbol_p (symbol) {
            Ok (symbol.to_string ())
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name")
                .line ("which must end with `-t`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .line ("expecting prop name")
            .line (&format! ("mexp = {}", mexp.to_string ()))
            .span (mexp.span ())
            .wrap_in_err ()
    }
}

fn mexp_to_var_term <'a> (
    mexp: &Mexp <'a>,
) -> Result <Term, ErrorInCtx> {
    if let Mexp::Sym {
        span,
        symbol,
    } = mexp {
        if var_symbol_p (symbol) {
            Ok (Term::Var (
                span.clone (),
                Var::no_id (symbol)))
        } else {
            ErrorInCtx::new ()
                .head ("syntex error")
                .line ("expecting var symbol")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_field_ref_term <'a> (
    mexp: &Mexp <'a>,
) -> Result <Term, ErrorInCtx> {
    if let Mexp::Sym {
        span,
        symbol,
    } = mexp {
        if field_name_symbol_p (symbol) {
            Ok (Term::FieldRef (
                span.clone (),
                symbol.to_string ()))
        } else {
            ErrorInCtx::new ()
                .head ("syntex error")
                .line ("expecting field name symbol")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_arg_to_arg <'a> (
    mexp_arg: &MexpArg <'a>,
) -> Result <Arg, ErrorInCtx> {
    match mexp_arg {
        MexpArg::Tuple { body, .. } => {
            Ok (Arg::Vec (mexp_vec_to_term_vec (body)?))
        }
        MexpArg::Block { body, .. } => {
            Ok (Arg::Rec (mexp_vec_to_binding_vec (body)?))
        }
    }
}

fn mexp_to_cons_term <'a> (
    mexp: &Mexp <'a>,
) -> Result <Term, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol,
            ..
        },
        arg,
        ..
    } = mexp {
        if cons_name_symbol_p (symbol) {
            Ok (Term::Cons (
                mexp.span (),
                symbol.to_string (),
                mexp_arg_to_arg (arg)?))
        } else {
            ErrorInCtx::new ()
                .line ("expecting cons name symbol")
                .line ("which must end with `-c`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else if let Mexp::Sym {
        symbol,
        span,
    } = mexp {
        if cons_name_symbol_p (symbol) {
            Ok (Term::Cons (
                span.clone (),
                symbol.to_string (),
                Arg::Rec (Vec::new ())))
        } else {
            ErrorInCtx::new ()
                .line ("expecting cons name symbol")
                .line ("which must end with `-c`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_prop_term <'a> (
    mexp: &Mexp <'a>,
) -> Result <Term, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol,
            ..
        },
        arg,
        ..
    } = mexp {
        if prop_name_symbol_p (symbol) {
            Ok (Term::Prop (
                mexp.span (),
                symbol.to_string (),
                mexp_arg_to_arg (arg)?))
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name symbol")
                .line ("which must end with `-t`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else if let Mexp::Sym {
        symbol,
        span,
    } = mexp {
        if prop_name_symbol_p (symbol) {
            Ok (Term::Prop (
                span.clone (),
                symbol.to_string (),
                Arg::Rec (Vec::new ())))
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name symbol")
                .line ("which must end with `-t`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_type_of_type_term <'a> (
    mexp: &Mexp <'a>,
) -> Result <Term, ErrorInCtx> {
    if let Mexp::Sym {
        span,
        symbol,
    } = mexp {
        if type_of_type_symbol_p (symbol) {
            Ok (Term::TypeOfType (span.clone ()))
        } else {
            ErrorInCtx::new ()
                .head ("syntex error")
                .line ("expecting type-of-type symbol")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_term <'a> (
    mexp: &Mexp <'a>,
) -> Result <Term, ErrorInCtx> {
    mexp_to_var_term (mexp)
        .or (mexp_to_cons_term (mexp))
        .or (mexp_to_prop_term (mexp))
        .or (mexp_to_field_ref_term (mexp))
        .or (mexp_to_type_of_type_term (mexp))
}

fn mexp_vec_to_term_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <Term>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_term (&mexp)?);
    }
    Ok (vec)
}

fn mexp_to_field_name <'a> (
    mexp: &Mexp <'a>,
) -> Result <String, ErrorInCtx> {
    if let Mexp::Sym {
        symbol,
        ..
    } = mexp {
        if field_name_symbol_p (symbol) {
            Ok (symbol.to_string ())
        } else {
            ErrorInCtx::new ()
                .line ("expecting field name symbol")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .line ("expecting prop name")
            .line (&format! ("mexp = {}", mexp.to_string ()))
            .span (mexp.span ())
            .wrap_in_err ()
    }
}

fn mexp_to_binding <'a> (
    mexp: &Mexp <'a>,
) -> Result <Binding, ErrorInCtx> {
    if let Mexp::Infix {
        op,
        lhs, rhs,
        ..
    } = mexp {
        if op == &"=" {
            Ok (Binding::EqualTo (
                mexp_to_field_name (lhs)?,
                mexp_to_term (rhs)?))
        } else if op == &":" {
            Ok (Binding::Inhabit (
                mexp_to_field_name (lhs)?,
                mexp_to_term (rhs)?))
        } else {
            ErrorInCtx::new ()
                .line ("expecting binding infix op")
                .line ("which might be `=` or `:`")
                .line (&format! ("op = {}", op))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_vec_to_binding_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <Binding>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_binding (&mexp)?);
    }
    Ok (vec)
}

fn mexp_to_disj_den <'a> (
    mexp: &Mexp <'a>,
) -> Result <Den, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "disj",
                ..
            },
            arg: MexpArg::Tuple {
                body: body1,
                ..
            },
            ..
        },
        arg: MexpArg::Block {
            body: body2,
            ..
        },
        ..
    } = mexp {
        Ok (Den::Disj (
            mexp_vec_to_prop_name_vec (body1)?,
            mexp_vec_to_binding_vec (body2)?))
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_conj_den <'a> (
    mexp: &Mexp <'a>,
) -> Result <Den, ErrorInCtx> {
    if let Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "conj",
                ..
            },
            arg: MexpArg::Block {
                body,
                ..
            },
            ..
        } = mexp {
        Ok (Den::Conj (mexp_vec_to_binding_vec (body)?))
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_den <'a> (
    mexp: &Mexp <'a>,
) -> Result <Den, ErrorInCtx> {
    mexp_to_disj_den (mexp)
        .or (mexp_to_conj_den (mexp))
}

fn mexp_to_den_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    if let Mexp::Infix {
        op: "=",
        lhs: box Mexp::Sym {
            symbol,
            ..
        },
        rhs,
        ..
    } = mexp {
        if prop_name_symbol_p (symbol) {
            Ok (Statement::Den (
                symbol.to_string (),
                mexp_to_den (rhs)?))
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name")
                .line ("which must end with `-t`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    mexp_to_den_statement (mexp)
        // .or (mexp_to_prop_statement (mexp))
        // .or (mexp_to_prove_statement (mexp))
}

fn mexp_vec_to_prop_name_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <String>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_prop_name (&mexp)?);
    }
    Ok (vec)
}

fn mexp_vec_to_statement_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <Statement>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_statement (&mexp)?);
    }
    Ok (vec)
}

const PRELUDE: &'static str = include_str! ("prelude.cic");

#[test]
fn test_unify () {
    let u = Value::Var (Var::new ("u"));
    let v = Value::Var (Var::new ("v"));
    let subst = Subst::new () .unify (
        &Value::Data ("cons-c" .to_string (), vec! [
            ("car", u.clone ()),
            ("cdr", v.clone ()),
        ] .into ()),
        &Value::Data ("cons-c" .to_string (), vec! [
            ("car", v.clone ()),
            ("cdr", Value::Data ("unit-c" .to_string (),
                                 Dic::new ())),
        ] .into ())) .unwrap ();
    println! ("{}", subst.to_string ());
    assert_eq! (subst.len (), 2);
}

#[test]
fn test_wis () {
    let mut wissen = Wissen::new ();
    let input = PRELUDE;
    match wissen.wis (input) {
        Ok (mut output_vec) => {
            println! ("{}", wissen.to_string ());
        }
        Err (error) => {
            let ctx = ErrorCtx::new ()
                .body (input);
            error.report (ctx);
        }
    }
}
