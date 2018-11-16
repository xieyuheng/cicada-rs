#![feature (uniform_paths)]
#![feature (bind_by_move_pattern_guards)]
#![feature (box_patterns)]
#![feature (box_syntax)]

#![allow (dead_code)]

use std::fmt;
use std::sync::Arc;
use std::collections::VecDeque;
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
            Term::Var (span, name) => {
                if let Some (value) = var_dic.get (name) {
                    unify_against (value, against, subst, span)?;
                    Ok (value.clone ())
                } else {
                    let new_var = Value::Var (Var::new (name));
                    var_dic.ins (name, Some (new_var.clone ()));
                    unify_against (&new_var, against, subst, span)?;
                    Ok (new_var)
                }
            }
            Term::Cons (span, name, arg) => {
                let (data, new_subst) = wissen.get_new_data (name)?;
                let data = Value::Data (data);
                *subst = new_subst.union (subst);
                if against.is_none () {
                    return ErrorInCtx::new ()
                        .head ("Term::value")
                        .line ("on Term::Cons")
                        .line (&format! ("name = {}", name))
                        .line ("no against")
                        .span (span.clone ())
                        .wrap_in_err ()
                }
                unify_against (&data, against, subst, span)?;
                value_dic_merge_arg (
                    data.value_dic () .unwrap (), arg,
                    wissen, subst, body, var_dic)?;
                Ok (data)
            }
            Term::Prop (span, name, arg) => {
                let (prop, new_subst) = wissen.get_prop (name)?;
                *subst = new_subst.union (subst);
                unify_against (&prop, against, subst, span)?;
                value_dic_merge_arg (
                    prop.value_dic () .unwrap (), arg,
                    wissen, subst, body, var_dic)?;
                Ok (prop)
            }
            Term::FieldRef (span, name) => {
                let value = body.get (name) .unwrap () .clone ();
                unify_against (&value, against, subst, span)?;
                Ok (value)
            }
            Term::TypeOfType (span) => {
                let value = Value::TypeOfType;
                unify_against (&value, against, subst, span)?;
                Ok (value)
            }
        }
    }
}

fn unify_against (
    value: &Value,
    against: Option <&Value>,
    subst: &mut Subst,
    span: &Span,
) -> Result <(), ErrorInCtx> {
    if let Some (old_value) = against {
        if let Some (
            new_subst
        ) = subst.unify (&old_value, &value) {
            *subst = new_subst.union (subst);
            Ok (())
        } else {
            return ErrorInCtx::new ()
                .head ("unify_against fail")
                .line (&format! ("old = {}",
                                 old_value
                                 .to_string ()))
                .line (&format! (">>> = {}",
                                 subst
                                 .deep_walk (old_value)
                                 .to_string ()))
                .line (&format! ("new = {}",
                                 value
                                 .to_string ()))
                .line (&format! (">>> = {}",
                                 subst
                                 .deep_walk (value)
                                 .to_string ()))
                .span (span.clone ())
                .wrap_in_err ()
        }
    } else {
        Ok (())
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
            let mut queue: VecDeque <Value> = VecDeque::new ();
            for value in value_dic.values () {
                let value = subst.walk (value);
                if let Value::TypedVar (_) = value {
                    queue.push_back (value);
                }
            }
            for term in term_vec {
                if let Some (old_value) = queue.pop_front () {
                    term.value (
                        wissen, subst, body, var_dic,
                        Some (&old_value))?;
                } else {
                    term.value (
                        wissen, subst, body, var_dic,
                        None)?;
                }
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
                            term.value (
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
                let tv = Value::TypedVar (new_tv (name, &value));
                if let Some (
                    old_value
                ) = body.get (name) {
                    if let Some (
                        new_subst
                    ) = subst.unify (&old_value, &tv) {
                        *subst = new_subst.union (subst);
                    } else {
                        return ErrorInCtx::new ()
                            .head ("Binding::bind")
                            .line ("on Binding::Inhabit")
                            .span (term.span ())
                            .wrap_in_err ()
                    }
                } else {
                    var_dic.ins (name, Some (tv.clone ()));
                    body.ins (name, Some (tv));
                }
                Ok (())
            }
        }
    }
}

fn new_tv (name: &str, value: &Value) -> TypedVar {
    TypedVar {
        id: Id::uuid (),
        name: name.to_string (),
        ty: box value.clone (),
    }
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
            Value::TypedVar (tv) => tv.to_string (),
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

impl TypedVar {
    fn fulfill (
        &self,
        wissen: &Wissen,
        subst: &Subst,
    ) -> Vec <(Vec <TypedVar>, Subst)> {
        let ty = subst.deep_walk (&self.ty);
        match ty {
            Value::Disj (disj) => {
                let mut tv_matrix = Vec::new ();
                for name in &disj.name_vec {
                    let (conj, new_subst) = wissen.get_prop (name) .unwrap ();
                    // ><><><
                    // can the above prop be disj too ?
                    let subst = new_subst.union (subst);
                    let new_tv = new_tv (&self.name, &conj);
                    if let Some (subst) = subst.unify (
                        &Value::TypedVar (self.clone ()),
                        &Value::TypedVar (new_tv.clone ())
                    ) {
                        tv_matrix.push ((vec! [new_tv], subst));
                    }
                }
                tv_matrix
            }
            Value::Conj (conj) => {
                let mut tv_matrix = Vec::new ();
                let (data, new_subst) = wissen.get_new_data (&conj.name) .unwrap ();
                let subst = new_subst.union (subst);
                if let Some (subst) = subst.unify (
                    &Value::TypedVar (self.clone ()),
                    &Value::Data (data.clone ()),
                ) {
                    let tv_vec = value_dic_to_tv_vec (
                        &subst,
                        &data.body);
                    tv_matrix.push ((tv_vec, subst));
                }
                tv_matrix
            }
            _ => {
                panic! ("TypedVar::fulfill");
            }
        }
    }
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
pub enum Subst {
    Null,
    VarBinding (Id, String, Value, Arc <Subst>),
    TypedVarBinding (Id, String, Value, Arc <Subst>),
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
            var.id.clone (),
            var.name.clone (),
            value,
            Arc::new (self.clone ()))
    }
}

impl Subst {
    fn bind_tv (
        &self,
        tv: TypedVar,
        value: Value,
    ) -> Self {
        Subst::TypedVarBinding (
            tv.id.clone (),
            tv.name.clone (),
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
                id1, name1, value, next,
            ) => {
                if id1 == &var.id && name1 == &var.name {
                    Some (value)
                } else {
                    next.find_var (var)
                }
            }
            Subst::TypedVarBinding (
                _id, _name , _value, next,
            ) => {
                next.find_var (var)
            }
        }
    }
}

impl Subst {
    pub fn find_tv (
        &self,
        tv: &TypedVar,
    ) -> Option <&Value> {
        match self {
            Subst::Null => None,
            Subst::VarBinding (
                _id, _name, _value, next,
            ) => {
                next.find_tv (tv)
            }
            Subst::TypedVarBinding (
                id1, name1, value, next,
            ) => {
                if id1 == &tv.id && name1 == &tv.name {
                    Some (value)
                } else {
                    next.find_tv (tv)
                }
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
                    _id, _name, _value, next
                ) => {
                    len += 1;
                    subst = &next;
                }
                Subst::TypedVarBinding (
                    _id, _name, _value, next
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
    fn reverse_union (&self, subst: &Subst) -> Subst {
        let mut subst = subst.clone ();
        let mut ante = self;
        loop {
            match ante {
                Subst::Null => {
                    return subst;
                }
                Subst::VarBinding (id, name, value, next) => {
                    subst = Subst::VarBinding (
                        id.clone (),
                        name.clone (),
                        value.clone (),
                        Arc::new (subst));
                    ante = next;
                }
                Subst::TypedVarBinding (id, name, value, next) => {
                    subst = Subst::TypedVarBinding (
                        id.clone (),
                        name.clone (),
                        value.clone (),
                        Arc::new (subst));
                    ante = next;
                }
            }
        }
    }
}

impl Subst {
    fn reverse (&self) -> Subst {
        self.reverse_union (&Subst::new ())
    }
}

impl Subst {
    pub fn union (&self, subst: &Subst) -> Subst {
        let ante = self.reverse ();
        ante.reverse_union (subst)
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
                    id, name, value, next
                ) => {
                    s += &name;
                    s += "#";
                    s += &id.to_string ();
                    s += " = ";
                    s += &value.to_string ();
                    s += "\n";
                    subst = &next;
                }
                Subst::TypedVarBinding (
                    id, name, value, next
                ) => {
                    s += &name;
                    s += "#";
                    s += &id.to_string ();
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
            Value::TypedVar (tv) => {
                if let Some (
                    new_value
                ) = self.find_tv (tv) {
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
            (Value::TypedVar (tv), v) |
            (v, Value::TypedVar (tv)) => {
                if self.tv_occur_p (&tv, &v) {
                    None
                } else if let Some (
                    subst
                ) = self.unify_type_to_value (&tv.ty, &v) {
                    // println! ("- here");
                    // println! ("  tv = {}", tv.to_string ());
                    // println! ("  v = {}", v.to_string ());
                    Some (subst.bind_tv (tv, v))
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
            (Value::Disj (u),
             Value::Disj (v),
            ) => {
                if u.name != v.name {
                    return None;
                }
                if u.name_vec != v.name_vec {
                    return None;
                }
                self.unify_dic (&u.body, &v.body)
            }
            (Value::Conj (u),
             Value::Conj (v),
            ) => {
                if u.name != v.name {
                    return None;
                }
                self.unify_dic (&u.body, &v.body)
            }
            (Value::Disj (disj), Value::Conj (conj)) |
            (Value::Conj (conj), Value::Disj (disj)) => {
                let name_set: HashSet <String> = disj.name_vec
                    .clone ()
                    .into_iter ()
                    .collect ();
                if ! name_set.contains (&conj.name) {
                    return None;
                }
                self.cover_dic (
                    &conj.body,
                    &disj.body)
            }
            (Value::TypeOfType, Value::TypeOfType) => {
                Some (self.clone ())
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
                subst = subst.unify (v1, v)?;
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
    pub fn tv_occur_p (
        &self,
        tv: &TypedVar,
        value: &Value,
    ) -> bool {
        let value = self.walk (value);
        match value {
            Value::TypedVar (tv1) => {
                tv == &tv1
            }
            Value::Data (data) => {
                for value in data.body.values () {
                    if self.tv_occur_p (tv, value) {
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
    pub fn deep_walk (&self, value: &Value) -> Value {
        let value = self.walk (value);
        match value {
            Value::Var (_) => value,
            Value::TypedVar (tv) => {
                Value::TypedVar (TypedVar {
                    ty: box self.deep_walk (&tv.ty),
                    ..tv
                })
            }
            Value::Disj (disj) => {
                Value::Disj (Disj {
                    body: self.deep_walk_dic (&disj.body),
                    ..disj
                })
            }
            Value::Conj (conj) => {
                Value::Conj (Conj {
                    body: self.deep_walk_dic (&conj.body),
                    ..conj
                })
            }
            Value::Data (data) => {
                Value::Data (Data {
                    body: self.deep_walk_dic (&data.body),
                    ..data
                })
            }
            Value::TypeOfType => Value::TypeOfType,
        }
    }
}

impl Subst {
    fn deep_walk_dic (
        &self,
        old_dic: &Dic <Value>,
    ) -> Dic <Value> {
        let mut new_dic = Dic::new ();
        for (name, value) in old_dic.iter () {
            new_dic.ins (name, Some (self.deep_walk (value)));
        }
        new_dic
    }
}

impl Subst {
    pub fn reify (&self, value: &Value) -> Value {
        // let value = self.deep_walk (&value);
        // let new_subst = Subst::new ();
        // let local_subst = new_subst.localize_by_value (&value);
        // local_subst.deep_walk (&value)
        self.deep_walk (&value)
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
        let mut output_counter = 0;
        for statement in &statement_vec {
            if let Statement::Prove (
                counter, prop_term
            ) = statement {
                output_counter += 1;
                let mut proving = self.proving (prop_term)?;
                output_vec.push (WissenOutput {
                    name: "#".to_string () + &output_counter.to_string (),
                    qed_vec: proving.take_qed (*counter),
                });
            }
            if let Statement::NamedProve (
                name, counter, prop_term
            ) = statement {
                let mut proving = self.proving (prop_term)?;
                output_vec.push (WissenOutput {
                    name: name.to_string (),
                    qed_vec: proving.take_qed (*counter),
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

fn prop_name_to_cons_name (cons_name: &str) -> String {
    let base_name = &cons_name[.. cons_name.len () - 2];
    format! ("{}-c", base_name)
}

impl Wissen {
    fn get_new_data (
        &self,
        name: &str,
    ) -> Result <(Data, Subst), ErrorInCtx> {
        let prop_name = &cons_name_to_prop_name (name);
        let (prop, subst) = self.get_prop (prop_name)?;
        let value_dic = prop.value_dic () .unwrap ();
        let data = Data {
            name: name.to_string (),
            body: value_dic.clone (),
        };
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

fn new_value (
    wissen: &Wissen,
    term: &Term,
) -> Result <(Value, Subst), ErrorInCtx> {
    let mut subst = Subst::new ();
    let mut body = Dic::new ();
    let mut var_dic = Dic::new ();
    let value = term.value (
            wissen,
            &mut subst,
            &mut body,
            &mut var_dic,
            None)?;
    Ok ((value, subst))
}

fn value_dic_to_tv_vec (
    subst: &Subst,
    value_dic: &Dic <Value>
) -> Vec <TypedVar> {
    let mut vec = Vec::new ();
    for value in value_dic.values () {
        let value = subst.walk (value);
        match value {
            Value::TypedVar (tv) => {
                let ty = subst.walk (&tv.ty);
                match ty {
                    Value::Disj (_) |
                    Value::Conj (_) => {
                        vec.push (tv);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    vec
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct WissenOutput {
    name: String,
    qed_vec: Vec <Qed>,
}

impl ToString for WissenOutput {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        s += &self.name;
        s += " = ";
        if self.qed_vec.len () == 0 {
            s += "<proofs></proofs>\n";
        } else {
            s += "<proofs>\n";
            for qed in &self.qed_vec {
                s += &qed.to_string ();
            }
            s += "</proofs>\n";
        }
        s 
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Statement {
    Den (String, Den),
    Prove (usize, Term),
    NamedProve (String, usize, Term),
}

impl Wissen {
    pub fn proving <'a> (
        &'a self,
        prop_term: &Term,
    ) -> Result <Proving <'a>, ErrorInCtx> {
        let (value, subst) = new_value (
            self, prop_term)?;
        let mut tv_queue = VecDeque::new ();
        let root_tv = new_tv ("root", &value);
        tv_queue.push_back (root_tv.clone ());
        let proof = Proof {
            wissen: self,
            subst: subst,
            root: Value::TypedVar (root_tv),
            tv_queue,
        };
        Ok (Proving {
            proof_queue: vec! [proof] .into (),
        })
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
                ProofStep::Qed (qed) => {
                    return Some (qed);
                }
                ProofStep::More (proof_queue) => {
                    for proof in proof_queue {
                        //// about searching
                        // push back  |   depth first
                        // push front | breadth first
                        self.proof_queue.push_back (proof);
                    }
                }
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
    root: Value,
    tv_queue: VecDeque <TypedVar>,
}

impl <'a> Proof <'a> {
    fn step (&mut self) -> ProofStep <'a> {
        if let Some (
            tv,
        ) = self.tv_queue.pop_front () {
            let tv_matrix = tv.fulfill (&self.wissen, &self.subst);
            let mut proof_queue = VecDeque::new ();
            for (tv_vec, new_subst) in tv_matrix {
                let mut proof = self.clone ();
                proof.subst = new_subst;
                for tv in tv_vec.into_iter () .rev () {
                    proof.tv_queue.push_front (tv);
                }
                proof_queue.push_back (proof)
            }
            ProofStep::More (proof_queue)
        } else {
            ProofStep::Qed (Qed {
                subst: self.subst.clone (),
                root: self.root.clone (),
            })
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum ProofStep <'a> {
    Qed (Qed),
    More (VecDeque <Proof <'a>>),
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Qed {
    subst: Subst,
    root: Value,
}

impl ToString for Qed {
    fn to_string (&self) -> String {
        Mexp::prettify (
            &self.subst.deep_walk (&self.root)
                .to_string ())
            .unwrap ()
    }
}

const GRAMMAR: &'static str = r#"
Statement::Den = { prop-name? "=" Den }
Statement::Prove = { "prove" '(' num? ')' Term::Prop }
Statement::NamedProve = { field-name? "=" "prove" '(' num? ')' Term::Prop }
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
            if let [
                prop_mexp
            ] = &body2 [..] {
                Ok (Statement::Prove (
                    result.unwrap (),
                    mexp_to_prop_term (prop_mexp)?))
            } else {
                ErrorInCtx::new ()
                    .line ("fail to parse `prove`'s body arg")
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ()
            }
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse `prove`'s first arg")
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


fn mexp_to_named_prove_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    if let Mexp::Infix {
        op: "=",
        lhs: box Mexp::Sym {
            symbol: name,
            ..
        },
        rhs: box Mexp::Apply {
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
            if let [
                prop_mexp
            ] = &body2 [..] {
                Ok (Statement::NamedProve (
                    name.to_string (),
                    result.unwrap (),
                    mexp_to_prop_term (prop_mexp)?))
            } else {
                ErrorInCtx::new ()
                    .line ("fail to parse `prove`'s body arg")
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ()
            }
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse `prove`'s first arg")
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
        .or (mexp_to_named_prove_statement (mexp))
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
    include_str! ("../examples/prelude.cic");

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
    // println! ("{}", subst.to_string ());
    assert_eq! (subst.len (), 2);
}

#[test]
fn test_wissen_get_prop () {
    let mut wissen = Wissen::new ();
    let input = PRELUDE;
    let ctx = ErrorCtx::new () .body (input);
    match wissen.wis (input) {
        Ok (_output_vec) => {}
        Err (error) => {
            error.print (ctx.clone ());
        }
    }
    for name in wissen.den_dic.keys () {
        match wissen.get_prop (name) {
            Ok ((_prop, _subst)) => {}
            // Ok ((prop, _subst)) => {
            //     println! (
            //         "<prop>\n{}\n</prop>",
            //         prop.to_string ());
            //     // println! ("{}", subst.to_string ());
            // }
            Err (error) => {
                println! ("- fail on name = {}", name);
                error.print (ctx.clone ());
                panic! ("test_wissen_get_prop");
            }
        }
    }
}

#[test]
fn test_wissen_output () {
    let mut wissen = Wissen::new ();
    let input = PRELUDE;
    let ctx = ErrorCtx::new () .body (input);
    match wissen.wis (input) {
        Ok (output_vec) => {
            for output in output_vec {
                println! ("{}", output.to_string ());
            }
        }
        Err (error) => {
            error.print (ctx.clone ());
        }
    }
}
