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
    TypeOfType,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Arg {
    Vec (Vec <Term>),
    Dic (Dic <Term>),
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
    Disj (Vec <String>, Dic <Term>),
    Conj (Dic <Term>),
}

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
