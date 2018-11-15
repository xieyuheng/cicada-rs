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
        format! ("{}{}\n", start, end)
    } else {
        format! ("{}\n{}{}\n", start, input, end)
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Term {
    Var (Span, String),
    Cons (Span, String, Arg),
    Prop (Span, String, Arg),
    FieldRef (Span, String),
    TypeOfType (Span),
}

impl ToString for Term {
    fn to_string (&self) -> String {
        match self {
            Term::Var (_span, name) => {
                name.clone ()
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

impl Term {
    fn span (&self) -> Span {
        match self {
            Term::Var (span, _name) => {
                span.clone ()
            }
            Term::Cons (span, _name, _arg) |
            Term::Prop (span, _name, _arg) => {
                span.clone ()
            }
            Term::FieldRef (span, _name) => {
                span.clone ()
            }
            Term::TypeOfType (span) => {
                span.clone ()
            }
        }
    }
}

impl Term {
    pub fn value (
        &self,
        wissen: &Wissen,
        subst: &mut Subst,
        body: &Dic <Value>,
        var_dic: &mut Dic <Value>,
        against: Option <&Value>,
    ) -> Result <Value, ErrorInCtx> {
        match self {
            Term::Var (_span, name) => {
                if let Some (value) = var_dic.get (name) {
                    Ok (value.clone ())
                } else {
                    let new_var = Value::Var (Var::new (name));
                    var_dic.ins (name, Some (new_var.clone ()));
                    Ok (new_var)
                }
            }
            Term::Cons (span, name, arg) => {
                let (data, s) = wissen.get_new_data (name)?;
                *subst = subst.append (s);
                if let Some (old_value) = against {
                    if let Some (
                        s
                    ) = subst.unify (&old_value, &data) {
                        *subst = subst.append (s);
                        value_dic_merge_arg (
                            data.value_dic () .unwrap (), arg,
                            wissen, subst, body, var_dic)?;
                        Ok (data)
                    } else {
                        return ErrorInCtx::new ()
                            .head ("Term::value")
                            .line ("on Term::Cons")
                            .line ("unification fail")
                            .line (&format! (
                                "old_value = {}",
                                old_value.to_string ()))
                            .line (&format! (
                                "data = {}",
                                data.to_string ()))
                            .span (span.clone ())
                            .wrap_in_err ()
                    }
                } else {
                    return ErrorInCtx::new ()
                        .head ("Term::value")
                        .line ("on Term::Cons")
                        .line (&format! ("name = {}", name))
                        .line ("no against")
                        .span (span.clone ())
                        .wrap_in_err ()
                }
            }
            Term::Prop (span, name, arg) => {
                let (prop, s) = wissen.get_prop (name)?;
                *subst = subst.append (s);
                if let Some (old_value) = against {
                    if let Some (
                        s
                    ) = subst.unify (&old_value, &prop) {
                        *subst = subst.append (s);
                    } else {
                        return ErrorInCtx::new ()
                            .head ("Term::value")
                            .line ("on Term::Prop")
                            .line ("unification fail")
                            .line (&format! (
                                "old_value = {}",
                                old_value.to_string ()))
                            .line (&format! (
                                "prop = {}",
                                prop.to_string ()))
                            .span (span.clone ())
                            .wrap_in_err ()
                    }
                }
                value_dic_merge_arg (
                    prop.value_dic () .unwrap (), arg,
                    wissen, subst, body, var_dic)?;
                Ok (prop)
            }
            Term::FieldRef (_span, name) => {
                let value = body.get (name) .unwrap ();
                Ok (value.clone ())
            }
            Term::TypeOfType (_span) => {
                Ok (Value::TypeOfType)
            }
        }
    }
}

fn value_dic_merge_arg (
    value_dic: &Dic <Value>,
    arg: &Arg,
    wissen: &Wissen,
    subst: &mut Subst,
    body: &Dic <Value>,
    var_dic: &mut Dic <Value>,
) -> Result <(), ErrorInCtx> {
    match arg {
        Arg::Vec (term_vec) => {
            for term in term_vec {
                let old_value = value_dic_next_value (
                    value_dic,
                    subst.clone ());
                let value = term.value (
                    wissen, subst, body, var_dic,
                    Some (&old_value))?;
            }
            Ok (())
        }
        Arg::Rec (binding_vec) => {
            for binding in binding_vec {
                match binding {
                    Binding::EqualTo (name, term) => {
                        if let Some (
                            old_value
                        ) = value_dic.get (name) {
                            let value = term.value (
                                wissen, subst, body, var_dic,
                                Some (old_value))?;
                        } else {
                            return ErrorInCtx::new ()
                                .head ("value_dic_merge_arg")
                                .line ("on Binding::EqualTo")
                                .line (&format! ("name = {}", name))
                                .span (term.span ())
                                .wrap_in_err ()
                        }
                    }
                    Binding::Inhabit (name, term) => {
                        return ErrorInCtx::new ()
                            .head ("value_dic_merge_arg")
                            .line ("on Binding::Inhabit")
                            .line (&format! ("name = {}", name))
                            .span (term.span ())
                            .wrap_in_err ()
                    }
                }
            }
            Ok (())
        }
    }
}

fn value_dic_next_value (
    value_dic: &Dic <Value>,
    subst: Subst,
) -> Value {
    for value in value_dic.values () {
        let value = subst.walk (value);
        if let Value::TypedVar (_) = value {
            return value;
        }
    }
    panic! ("value_dic_next_value")
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
            Arg::Rec (binding_vec) => binding_vec.is_empty (),
        }
    }
}

impl ToString for Arg {
    fn to_string (&self) -> String {
        match self {
            Arg::Vec (term_vec) => {
                format! (
                    "({})",
                    vec_to_string (term_vec, " "))
            }
            Arg::Rec (binding_vec) => {
                format! (
                    "{{ {} }}",
                    vec_to_string (binding_vec, ", "))
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

impl Binding {
    fn bind (
        &self,
        wissen: &Wissen,
        subst: &mut Subst,
        body: &mut Dic <Value>,
        var_dic: &mut Dic <Value>,
    ) -> Result <(), ErrorInCtx> {
        match self {
            Binding::EqualTo (name, term) => {
                let old_value = body.get (name) .unwrap ();
                let _value = term.value (
                    wissen, subst, body, var_dic,
                    Some (old_value))?;
                Ok (())
            }
            Binding::Inhabit (name, term) => {
                let value = term.value (
                    wissen, subst, body, var_dic,
                    None)?;
                let typed_var = new_typed_var (name, &value);
                if let Some (
                    old_value
                ) = body.get (name) {
                    if let Some (
                        new_subst
                    ) = subst.unify (&old_value, &typed_var) {
                        *subst = subst.append (new_subst);
                    } else {
                        return ErrorInCtx::new ()
                            .head ("Binding::bind")
                            .line ("on Binding::Inhabit")
                            .span (term.span ())
                            .wrap_in_err ()
                    }
                } else {
                    var_dic.ins (name, Some (typed_var.clone ()));
                    body.ins (name, Some (typed_var));
                }
                Ok (())
            }
        }
    }
}

fn new_typed_var (name: &str, value: &Value) -> Value {
    Value::TypedVar (TypedVar {
        id: Id::uuid (),
        name: name.to_string (),
        ty: box value.clone (),
    })
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Value {
    Var (Var),
    TypedVar (TypedVar),
    Disj (Disj),
    Conj (Conj),
    Data (Data),
    TypeOfType,
}

impl ToString for Value {
    fn to_string (&self) -> String {
        match self {
            Value::Var (var) => var.to_string (),
            Value::TypedVar (typed_var) => typed_var.to_string (),
            Value::Disj (disj) => disj.to_string (),
            Value::Conj (conj) => conj.to_string (),
            Value::Data (data) => data.to_string (),
            Value::TypeOfType => format! ("type"),
        }
    }
}

impl Value {
    fn value_dic (&self) -> Option <&Dic <Value>> {
        match self {
            Value::Disj (disj) => Some (&disj.body),
            Value::Conj (conj) => Some (&conj.body),
            Value::Data (data) => Some (&data.body),
            _ => None,
        }
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
            Id::Uuid (uuid) => {
                // uuid.to_string ()
                let s = uuid.to_string ();
                format! ("{}", &s[0 .. 3])
            }
            Id::Local (counter) => {
                format! ("{}", counter)
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub struct Var {
    id: Id,
    name: String,
}

impl Var {
    fn new (s: &str) -> Self {
        Var {
            id: Id::uuid (),
            name: s.to_string (),
        }
    }
}

impl Var {
    fn local (s: &str, counter: usize) -> Self {
        Var {
            id: Id::local (counter),
            name: s.to_string (),
        }
    }
}

impl ToString for Var {
    fn to_string (&self) -> String {
        format! (
            "{}#{}",
            self.name,
            self.id.to_string ())
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct TypedVar {
    id: Id,
    name: String,
    ty: Box <Value>,
}

impl ToString for TypedVar {
    fn to_string (&self) -> String {
        format! (
            "{}#{} : {}",
            self.name,
            self.id.to_string (),
            self.ty.to_string ())
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Disj {
    name: String,
    name_vec: Vec <String>,
    body: Dic <Value>,
}

impl ToString for Disj {
    fn to_string (&self) -> String {
        if self.body.is_empty () {
            format! (
                "{} ({}) {{}}",
                self.name,
                vec_to_string (&self.name_vec, ", "))
        } else {
            format! (
                "{} ({}) {{ {} }}",
                self.name,
                vec_to_string (&self.name_vec, ", "),
                dic_to_string (&self.body))
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Conj {
    name: String,
    body: Dic <Value>,
}

impl ToString for Conj {
    fn to_string (&self) -> String {
        if self.body.is_empty () {
            format! (
                "{} {{}}",
                self.name)
        } else {
            format! (
                "{} {{ {} }}",
                self.name,
                dic_to_string (&self.body))
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Data {
    name: String,
    body: Dic <Value>,
}

impl ToString for Data {
    fn to_string (&self) -> String {
        if self.body.is_empty () {
            format! (
                "{} {{}}",
                self.name)
        } else {
            format! (
                "{} {{ {} }}",
                self.name,
                dic_to_string (&self.body))
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Prop {
    Disj (Disj),
    Conj (Conj),
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Subst {
    Null,
    VarBinding (Var, Value, Arc <Subst>),
    TypedVarBinding (TypedVar, Value, Arc <Subst>),
}

impl Subst {
    fn new () -> Self {
        Subst::Null
    }
}

impl Subst {
    fn bind_var (
        &self,
        var: Var,
        value: Value,
    ) -> Self {
        Subst::VarBinding (
            var,
            value,
            Arc::new (self.clone ()))
    }
}

impl Subst {
    fn bind_typed_var (
        &self,
        typed_var: TypedVar,
        value: Value,
    ) -> Self {
        Subst::TypedVarBinding (
            typed_var,
            value,
            Arc::new (self.clone ()))
    }
}

impl Subst {
    pub fn find_var (
        &self,
        var: &Var,
    ) -> Option <&Value> {
        match self {
            Subst::Null => None,
            Subst::VarBinding (
                var1, value, next,
            ) => {
                if var1 == var {
                    Some (value)
                } else {
                    next.find_var (var)
                }
            }
            Subst::TypedVarBinding (
                _typed_var, _value, next,
            ) => {
                next.find_var (var)
            }
        }
    }
}

impl Subst {
    pub fn find_typed_var (
        &self,
        typed_var: &TypedVar,
    ) -> Option <&Value> {
        match self {
            Subst::Null => None,
            Subst::VarBinding (
                _var, _value, next,
            ) => {
                next.find_typed_var (typed_var)
            }
            Subst::TypedVarBinding (
                typed_var1, value, next,
            ) => {
                if typed_var1 == typed_var {
                    Some (value)
                } else {
                    next.find_typed_var (typed_var)
                }
            }
        }
    }
}

impl Subst {
    pub fn walk (&self, value: &Value) -> Value {
        match value {
            Value::Var (var) => {
                if let Some (
                    new_value
                ) = self.find_var (var) {
                    self.walk (new_value)
                } else {
                    value.clone ()
                }
            }
            Value::TypedVar (typed_var) => {
                if let Some (
                    new_value
                ) = self.find_typed_var (typed_var) {
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
            (Value::TypedVar (u),
             Value::TypedVar (v),
            ) if u == v => {
                Some (self.clone ())
            }
            (Value::Var (u), v) => {
                if self.var_occur_p (&u, &v) {
                    None
                } else {
                    Some (self.bind_var (u, v))
                }
            }
            (u, Value::Var (v)) => {
                if self.var_occur_p (&v, &u) {
                    None
                } else {
                    Some (self.bind_var (v, u))
                }
            }
            (Value::TypedVar (u), v) => {
                if self.typed_var_occur_p (&u, &v) {
                    None
                } else if let Some (
                    subst
                ) = self.unify_type_to_value (&u.ty, &v) {
                    Some (subst.bind_typed_var (u, v))
                } else {
                    None
                }
            }
            (u, Value::TypedVar (v)) => {
                if self.typed_var_occur_p (&v, &u) {
                    None
                } else if let Some (
                    subst
                ) = self.unify_type_to_value (&v.ty, &u) {
                    Some (subst.bind_typed_var (v, u))
                } else {
                    None
                }
            }
            (Value::Data (u),
             Value::Data (v),
            ) => {
                if u.name != v.name {
                    return None;
                }
                self.unify_dic (&u.body, &v.body)
            }
            // ><><><
            // Value::Disj Value::Disj
            // ><><><
            // Value::Conj Value::Conj
            // ><><><
            // Value::Conj Value::Disj
            // ><><><
            // Value::Disj Value::Conj
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
    pub fn unify_type_to_value (
        &self,
        t: &Value,
        v: &Value,
    ) -> Option <Subst> {
        let t = self.walk (t);
        let v = self.walk (v);
        match (t, v) {
            (Value::Conj (conj), Value::Data (data)) => {
                let prop_name = cons_name_to_prop_name (
                    &data.name);
                if conj.name != prop_name {
                    None
                } else {
                    self.cover_dic (
                        &data.body,
                        &conj.body)
                }
            }
            (Value::Disj (disj), Value::Data (data)) => {
                let prop_name = cons_name_to_prop_name (
                    &data.name);
                let name_set: HashSet <String> = disj.name_vec
                    .clone ()
                    .into_iter ()
                    .collect ();
                if name_set.contains (&prop_name) {
                    self.cover_dic (
                        &data.body,
                        &disj.body)
                } else {
                    None
                }
            }
            (Value::TypeOfType, Value::Disj (..)) => {
                Some (self.clone ())
            }
            (Value::TypeOfType, Value::Conj (..)) => {
                Some (self.clone ())
            }
            (t, Value::TypedVar (v)) => {
                self.unify (&t, &v.ty)
            }
            _ => {
                None
            }
        }
    }
}

impl Subst {
    pub fn cover_dic (
        &self,
        large_dic: &Dic <Value>,
        small_dic: &Dic <Value>,
    ) -> Option <Subst> {
        let mut subst = self.clone ();
        for (name, v) in small_dic.iter () {
            if let Some (v1) = large_dic.get (name) {
                subst.unify (v1, v)?;
            } else {
                return None;
            }
        }
        Some (subst)
    }
}

impl Subst {
    pub fn unify_dic (
        &self,
        u_dic: &Dic <Value>,
        v_dic: &Dic <Value>,
    ) -> Option <Subst> {
        let mut subst = self.clone ();
        let zip = u_dic.entries () .zip (v_dic.entries ());
        for (u_entry, v_entry) in zip {
            if u_entry.name != v_entry.name {
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
            Value::Data (data) => {
                for value in data.body.values () {
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
    pub fn typed_var_occur_p (
        &self,
        typed_var: &TypedVar,
        value: &Value,
    ) -> bool {
        let value = self.walk (value);
        match value {
            Value::TypedVar (typed_var1) => {
                typed_var == &typed_var1
            }
            Value::Data (data) => {
                for value in data.body.values () {
                    if self.typed_var_occur_p (typed_var, value) {
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
        loop {
            match subst {
                Subst::Null => break,
                Subst::VarBinding (
                    _var, _value, next
                ) => {
                    len += 1;
                    subst = &next;
                }
                Subst::TypedVarBinding (
                    _typed_var, _value, next
                ) => {
                    len += 1;
                    subst = &next;
                }
            }
        }
        len
    }
}

impl Subst {
    pub fn append (&self, subst: Subst) -> Subst {
        match self {
            Subst::Null => { subst }
            Subst::VarBinding (var, value, next) => {
                Subst::VarBinding (
                    var.clone (),
                    value.clone (),
                    Arc::new (next.append (subst)))
            }
            Subst::TypedVarBinding (typed_var, value, next) => {
                Subst::TypedVarBinding (
                    typed_var.clone (),
                    value.clone (),
                    Arc::new (next.append (subst)))
            }
        }
    }
}

impl ToString for Subst {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        let mut subst = self;
        loop {
            match subst {
                Subst::Null => break,
                Subst::VarBinding (
                    var, value, next
                ) => {
                    s += &var.to_string ();
                    s += " = ";
                    s += &value.to_string ();
                    s += "\n";
                    subst = &next;
                }
                Subst::TypedVarBinding (
                    typed_var, value, next
                ) => {
                    s += &typed_var.to_string ();
                    s += " = ";
                    s += &value.to_string ();
                    s += "\n";
                    subst = &next;
                }
            }
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
            Den::Disj (name_vec, binding_vec) => {
                if binding_vec.is_empty () {
                    format! (
                        "disj ({}) {{}}",
                        vec_to_string (name_vec, " "))
                } else {
                    format! (
                        "disj ({}) {{ {} }}",
                        vec_to_string (name_vec, " "),
                        vec_to_string (binding_vec, ", "))
                }
            }
            Den::Conj (binding_vec) => {
                if binding_vec.is_empty () {
                    format! ("conj {{}}")
                } else {
                    format! (
                        "conj {{ {} }}",
                        vec_to_string (binding_vec, ", "))
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
        for statement in &statement_vec {
            if let Statement::Prove (
                counter, binding_vec
            ) = statement {
                let mut proving = self.proving (binding_vec);
                let qed_vec = proving.take_qed (*counter);
                output_vec.push (WissenOutput {
                    qed_vec,
                });
            }
        }
        Ok (output_vec)
    }
}

impl ToString for Wissen {
    fn to_string (&self) -> String {
        add_tag ("<wissen>", dic_to_lines (&self.den_dic))
    }
}

impl Wissen {
    fn get_prop (
        &self,
        name: &str,
    ) -> Result <(Value, Subst), ErrorInCtx> {
        let den = self.den_dic.get (name) .unwrap ();
        match den {
            Den::Disj (name_vec, binding_vec) => {
                let (body, subst) = new_value_dic (
                    self, binding_vec)?;
                let disj = Value::Disj (Disj {
                    name: name.to_string (),
                    name_vec: name_vec.clone (),
                    body,
                });
                Ok ((disj, subst))
            }
            Den::Conj (binding_vec) => {
                let (body, subst) = new_value_dic (
                    self, binding_vec)?;
                let conj = Value::Conj (Conj {
                    name: name.to_string (),
                    body,
                });
                Ok ((conj, subst))
            }
        }
    }
}

fn cons_name_to_prop_name (cons_name: &str) -> String {
    let base_name = &cons_name[.. cons_name.len () - 2];
    format! ("{}-t", base_name)
}

impl Wissen {
    fn get_new_data (
        &self,
        name: &str,
    ) -> Result <(Value, Subst), ErrorInCtx> {
        let prop_name = &cons_name_to_prop_name (name);
        let (prop, subst) = self.get_prop (prop_name)?;
        let value_dic = prop.value_dic () .unwrap ();
        let data = Value::Data (Data {
            name: name.to_string (),
            body: value_dic.clone (),
        });
        Ok ((data, subst))
    }
}

fn new_value_dic (
    wissen: &Wissen,
    binding_vec: &Vec <Binding>,
) -> Result <(Dic <Value>, Subst), ErrorInCtx> {
    let mut subst = Subst::new ();
    let mut body = Dic::new ();
    let mut var_dic = Dic::new ();
    for binding in binding_vec {
        binding.bind (
            wissen,
            &mut subst,
            &mut body,
            &mut var_dic)?;
    }
    Ok ((body, subst))
}

fn value_dic_to_prop_queue (
    subst: &Subst,
    value_dic: &Dic <Value>
) -> VecDeque <(String, Prop)> {
    let mut prop_queue = VecDeque::new ();
    for (name, value) in value_dic.iter () {
        let value = subst.walk (value);        
        let prop = match value {
            Value::Disj (disj) => {
                prop_queue.push_back ((
                    name.to_string (),
                    Prop::Disj (disj)
                ));
            }
            Value::Conj (conj) => {
                prop_queue.push_back ((
                    name.to_string (),
                    Prop::Conj (conj)
                ));
            }
            _ => {}
        };
    }
    prop_queue
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct WissenOutput {
    qed_vec: Vec <Qed>,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Statement {
    Den (String, Den),
    Prove (usize, Vec <Binding>),
}

impl Wissen {
    pub fn proving <'a> (
        &'a self,
        binding_vec: &Vec <Binding>,
    ) -> Proving <'a> {
        let (value_dic, subst) = new_value_dic (
            self, binding_vec) .unwrap ();
        let prop_queue = value_dic_to_prop_queue (
            &subst,
            &value_dic);
        let root_data = Data {
            name: "root-c".to_string (),
            body: value_dic,
        };
        let proof = Proof {
            wissen: self,
            subst: subst,
            data_stack: vec! [
                ("root".to_string (), root_data),
            ],
            prop_queue: prop_queue,
        };
        Proving {
            proof_queue: vec! [proof] .into (),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Proving <'a> {
    proof_queue: VecDeque <Proof <'a>>,
}

impl <'a> Proving <'a> {
    pub fn next_qed (&mut self) -> Option <Qed> {
        while let Some (
            mut proof
        ) = self.proof_queue.pop_front () {
            match proof.step () {
                ProofStep::One => {
                    if let Some (
                        (_name, data)
                    ) = proof.data_stack.pop () {
                        return Some (Qed {
                            subst: proof.subst,
                            body: data.body,
                        });
                    } else {
                        panic! ("Proving::next_qed");
                    }
                }
                ProofStep::More (proof_queue) => {
                    for proof in proof_queue {
                        //// about searching
                        // push back  |   depth first
                        // push front | breadth first
                        self.proof_queue.push_back (proof);
                    }
                }
                ProofStep::Fail => {}
            }
        }
        return None;
    }
}

impl <'a> Proving <'a> {
    pub fn take_qed (&mut self, n: usize) -> Vec <Qed> {
        let mut vec = Vec::new ();
        for _ in 0..n {
            if let Some (qed) = self.next_qed () {
                vec.push (qed)
            }
        }
        vec
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Proof <'a> {
    wissen: &'a Wissen,
    subst: Subst,
    data_stack: Vec <(String, Data)>,
    prop_queue: VecDeque <(String, Prop)>,
}

impl <'a> Proof <'a> {
    fn step (&mut self) -> ProofStep <'a> {
        if let Some (
            (name, prop),
        ) = self.prop_queue.pop_front () {
            unimplemented! ()
        } else {
            ProofStep::One
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum ProofStep <'a> {
    One,
    More (VecDeque <Proof <'a>>),
    Fail,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Qed {
    subst: Subst,
    body: Dic <Value>,
}

const GRAMMAR: &'static str = r#"
Statement::Den = { prop-name? "=" Den }
Statement::Prove = { "prove" '(' num? ')' Arg::Rec }
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
    symbol.starts_with (":")
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
                symbol.to_string ()))
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
        Ok (Den::Conj (
            mexp_vec_to_binding_vec (body)?))
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

fn mexp_to_prove_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "prove",
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
        if let [
            Mexp::Sym { symbol, .. }
        ] = &body1 [..] {
            let result = symbol.parse::<usize> ();
            if result.is_err () {
                return ErrorInCtx::new ()
                    .line ("fail to parse usize num in `prove`")
                    .line (&format! ("symbol = {}", symbol))
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ();
            }
            Ok (Statement::Prove (
                result.unwrap (),
                mexp_vec_to_binding_vec (body2)?))
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse prop's first arg")
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
        .or (mexp_to_prove_statement (mexp))
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

const PRELUDE: &'static str =
    include_str! ("prelude.cic");

#[test]
fn test_unify () {
    let u = Value::Var (Var::new ("u"));
    let v = Value::Var (Var::new ("v"));
    let subst = Subst::new () .unify (
        &Value::Data (Data {
            name: "cons-c" .to_string (),
            body: vec! [
                ("car", u.clone ()),
                ("cdr", v.clone ()),
            ] .into ()
        }),
        &Value::Data (Data {
            name: "cons-c" .to_string (),
            body: vec! [
                ("car", v.clone ()),
                ("cdr", Value::Data (Data {
                    name: "unit-c" .to_string (),
                    body: Dic::new (),
                })),
            ] .into ()
        }))
        .unwrap ();
    println! ("{}", subst.to_string ());
    assert_eq! (subst.len (), 2);
}

#[test]
fn test_wis () {
    let mut wissen = Wissen::new ();
    let input = PRELUDE;
    let ctx = ErrorCtx::new () .body (input);
    match wissen.wis (input) {
        Ok (mut output_vec) => {
            println! ("{}", wissen.to_string ());
        }
        Err (error) => {
            error.print (ctx.clone ());
        }
    }
    for name in wissen.den_dic.keys () {
        match wissen.get_prop (name) {
            Ok ((prop, subst)) => {
                println! (
                    "<prop>\n{}\n</prop>",
                    prop.to_string ());
                println! ("{}", subst.to_string ());
            }
            Err (error) => {
                println! ("- fail on name = {}", name);
                error.print (ctx.clone ());
            }
        }
    }
}
