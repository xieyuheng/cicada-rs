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
    ErrorCtx,
    ErrorInCtx,
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

fn format_mexp_block (head: &str, input: String) -> String {
    if input.is_empty () {
        format! ("{} {{}}", head)
    } else {
        format! ("{} {{ {} }}", head, input)
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
        module: &Module,
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
                let (data, new_subst) = module.get_new_data (name)?;
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
                    module, subst, body, var_dic)?;
                Ok (data)
            }
            Term::Prop (span, name, arg) => {
                let (prop, new_subst) = module.get_prop (name)?;
                *subst = new_subst.union (subst);
                unify_against (&prop, against, subst, span)?;
                value_dic_merge_arg (
                    prop.value_dic () .unwrap (), arg,
                    module, subst, body, var_dic)?;
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
    module: &Module,
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
                        module, subst, body, var_dic,
                        Some (&old_value))?;
                } else {
                    term.value (
                        module, subst, body, var_dic,
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
                                module, subst, body, var_dic,
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
        module: &Module,
        subst: &mut Subst,
        body: &mut Dic <Value>,
        var_dic: &mut Dic <Value>,
    ) -> Result <(), ErrorInCtx> {
        match self {
            Binding::EqualTo (name, term) => {
                let old_value = body.get (name) .unwrap ();
                let _value = term.value (
                    module, subst, body, var_dic,
                    Some (old_value))?;
                Ok (())
            }
            Binding::Inhabit (name, term) => {
                let value = term.value (
                    module, subst, body, var_dic,
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
        module: &Module,
        subst: &Subst,
    ) -> Vec <(Vec <TypedVar>, Subst)> {
        let ty = subst.deep_walk (&self.ty);
        match ty {
            Value::Disj (disj) => {
                let mut tv_matrix = Vec::new ();
                for name in &disj.name_vec {
                    let (conj, new_subst) = module.get_prop (name) .unwrap ();
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
                let (data, new_subst) = module.get_new_data (&conj.name) .unwrap ();
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
    fn bind_tv (
        &self,
        tv: TypedVar,
        value: Value,
    ) -> Self {
        Subst::TypedVarBinding (
            tv,
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
                _tv, _value, next,
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
                _var, _value, next,
            ) => {
                next.find_tv (tv)
            }
            Subst::TypedVarBinding (
                tv1, value, next,
            ) => {
                if tv1 == tv {
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
                    _var, _value, next
                ) => {
                    len += 1;
                    subst = &next;
                }
                Subst::TypedVarBinding (
                    _tv, _value, next
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
                Subst::VarBinding (var, value, next) => {
                    subst = Subst::VarBinding (
                        var.clone (),
                        value.clone (),
                        Arc::new (subst));
                    ante = next;
                }
                Subst::TypedVarBinding (tv, value, next) => {
                    subst = Subst::TypedVarBinding (
                        tv.clone (),
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
                    var, value, next
                ) => {
                    s += &var.to_string ();
                    s += " = ";
                    s += &value.to_string ();
                    s += "\n";
                    subst = &next;
                }
                Subst::TypedVarBinding (
                    tv, value, next
                ) => {
                    s += &tv.to_string ();
                    s += " = ";
                    s += &value.to_string ();
                    s += "\n";
                    subst = &next;
                }
            }
        }
        format_mexp_block ("$raw-subst-c", s)
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
                let prop_name = name_to_prop_name (
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
                let prop_name = name_to_prop_name (
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
    pub fn localize_by_value (&self, value: &Value) -> Subst {
        let value = self.walk (value);
        match value {
            Value::Var (var) => {
                self.bind_var (
                    var.clone (),
                    Value::Var (Var {
                        id: Id::local (self.len ()),
                        name: var.name.clone (),
                    }))
            }
            Value::TypedVar (tv) => {
                self.bind_tv (
                    tv.clone (),
                    Value::TypedVar (TypedVar {
                        id: Id::local (self.len ()),
                        name: tv.name.clone (),
                        ty: tv.ty.clone (),
                    }))
            }
            Value::Disj (disj) => {
                let mut subst = self.clone ();
                for value in disj.body.values () {
                    subst = subst.localize_by_value (value);
                }
                subst
            }
            Value::Conj (conj) => {
                let mut subst = self.clone ();
                for value in conj.body.values () {
                    subst = subst.localize_by_value (value);
                }
                subst
            }
            Value::Data (data) => {
                let mut subst = self.clone ();
                for value in data.body.values () {
                    subst = subst.localize_by_value (value);
                }
                subst
            }
            Value::TypeOfType => {
                self.clone ()
            }
        }
    }
}

impl Subst {
    pub fn reify (&self, value: &Value) -> Value {
        let value = self.deep_walk (&value);
        let new_subst = Subst::new ();
        let local_subst = new_subst.localize_by_value (&value);
        local_subst.deep_walk (&value)
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Obj {
    Disj (Vec <String>, Vec <Binding>),
    Conj (Vec <Binding>),
    Module (Module),
    SearchRes (SearchRes),
    QueryRes (QueryRes),
    StepRes (StepRes),
}

impl ToString for Obj {
    fn to_string (&self) -> String {
        match self {
            Obj::Disj (name_vec, binding_vec) => {
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
            Obj::Conj (binding_vec) => {
                if binding_vec.is_empty () {
                    format! ("conj {{}}")
                } else {
                    format! (
                        "conj {{ {} }}",
                        vec_to_string (binding_vec, ", "))
                }
            }
            Obj::Module (module) => {
                module.to_string ()
            }
            Obj::SearchRes (res) => {
                res.to_string ()
            }
            Obj::QueryRes (res) => {
                res.to_string ()
            }
            Obj::StepRes (res) => {
                res.to_string ()
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct SearchRes {
    qed_vec: Vec <Qed>,
}

impl ToString for SearchRes {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        if self.qed_vec.len () == 0 {
            s += "$search-res-c {}\n";
        } else {
            s += "$search-res-c {\n";
            for qed in &self.qed_vec {
                s += &qed.to_string ();
            }
            s += "}\n";
        }
        match Mexp::prettify (&s) {
            Ok (output) => output,
            Err (error) => {
                let ctx = ErrorCtx::new ()
                    .body (&s);
                error.print (ctx);
                panic! ("SearchRes::to_string")
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct QueryRes {
    var_vec: Vec <Var>,
    subst_vec: Vec <Subst>,
}

impl ToString for QueryRes {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        if self.subst_vec.len () == 0 {
            s += "$query-res-c {}\n";
        } else {
            s += "$query-res-c {\n";
            for subst in &self.subst_vec {
                s += &report_subst (&self.var_vec, subst);
            }
            s += "}\n";
        }
        match Mexp::prettify (&s) {
            Ok (output) => output,
            Err (error) => {
                let ctx = ErrorCtx::new ()
                    .body (&s);
                error.print (ctx);
                panic! ("QueryRes::to_string")
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct StepRes {
    step: Option <DeductionStep>,
    deduction_queue: VecDeque <Deduction>,
}

impl ToString for StepRes {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        s += "$step-res-c {\n";
        if let Some (step) = &self.step {
            s += "step = ";
            s += &step.to_string ();
        }
        let mut vec: Vec <Deduction> = Vec::new ();
        for x in &self.deduction_queue {
            vec.push (x.clone ());
        }
        s += &format! (
            "queue = [ {} ]",
            &vec_to_string (&vec, " "));
        s += "}\n";
        match Mexp::prettify (&s) {
            Ok (output) => output,
            Err (error) => {
                let ctx = ErrorCtx::new ()
                    .body (&s);
                error.print (ctx);
                panic! ("StepRes::to_string")
            }
        }
    }
}

fn report_subst (
    var_vec: &Vec <Var>,
    subst: &Subst,
) -> String {
    let mut s = String::new ();
    for var in var_vec {
        let value = Value::Var (var.clone ());
        s += &var.name;
        s += " = ";
        s += &subst.reify (&value) .to_string ();
        s += "\n";
    }
    format_mexp_block ("$subst-c", s)
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Module {
    obj_dic: Dic <Obj>,
    // source
}

impl Module {
    pub fn new () -> Self {
        Module {
            obj_dic: Dic::new (),
        }
    }
}

impl Module {
    pub fn define (
        &mut self,
        name: &str,
        obj: &Obj,
    ) -> Result <(), ErrorInCtx> {
        if self.obj_dic.has_name (name) {
            ErrorInCtx::new ()
                .head ("Module::define")
                .line ("name already defined")
                .line (&format! ("name = {}", name))
                .wrap_in_err ()
        } else {
            self.obj_dic.ins (name, Some (obj.clone ()));
            Ok (())
        }
    }
}

impl Module {
    pub fn exe_def <'a> (
        &'a mut self,
        index: usize,
        def: &Def,
    ) -> Result <Option <(String, Obj)>, ErrorInCtx> {
        match def {
            Def::Obj (
                name, obj
            ) => {
                let name = name.to_string ();
                self.define (&name, obj)?;
                Ok (Some ((name, obj.clone ())))
            }
            Def::Import (
                name, url
            ) => {
                let name = name.to_string ();
                let module = Module::load (url)?;
                let obj = Obj::Module (module);
                self.define (&name, &obj)?;
                Ok (Some ((name, obj)))
            }
            Def::Search (
                name, counter, prop_term
            ) => {
                let name = name.to_string ();
                if let Ok ((
                    value, subst
                )) = new_value (self, prop_term) {
                    let mut proving = self.proving (
                        &value, &subst);
                    let qed_vec = proving.take_qed (*counter);
                    let obj = Obj::SearchRes (SearchRes {
                        qed_vec
                    });
                    self.define (&name, &obj)?;
                    Ok (Some ((name, obj)))
                } else {
                    let obj = Obj::SearchRes (SearchRes {
                        qed_vec: Vec::new (),
                    });
                    self.define (&name, &obj)?;
                    Ok (Some ((name, obj)))
                }
            }
            Def::NamelessSearch (
                counter, prop_term
            ) => {
                let name = "#".to_string () +
                    &index.to_string ();
                if let Ok ((
                    value, subst
                )) = new_value (self, prop_term) {
                    let mut proving = self.proving (
                        &value, &subst);
                    let qed_vec = proving.take_qed (*counter);
                    let obj = Obj::SearchRes (SearchRes {
                        qed_vec,
                    });
                    self.define (&name, &obj)?;
                    Ok (Some ((name, obj)))
                } else {
                    let obj = Obj::SearchRes (SearchRes {
                        qed_vec: Vec::new (),
                    });
                    self.define (&name, &obj)?;
                    Ok (Some ((name, obj)))
                }
            }
            Def::Query (
                name, counter, prop_term
            ) => {
                let name = name.to_string ();
                if let Ok ((
                    value, subst, var_vec
                )) = new_value_and_var_vec (self, prop_term) {
                    let mut proving = self.proving (
                        &value, &subst);
                    let subst_vec = proving.take_subst (*counter);
                    let obj = Obj::QueryRes (QueryRes {
                        var_vec,
                        subst_vec,
                    });
                    self.define (&name, &obj)?;
                    Ok (Some ((name, obj)))
                } else {
                    let obj = Obj::QueryRes (QueryRes {
                        var_vec: Vec::new (),
                        subst_vec: Vec::new (),
                    });
                    self.define (&name, &obj)?;
                    Ok (Some ((name, obj)))
                }
            }
            Def::NamelessQuery (
                counter, prop_term
            ) => {
                let name = "#".to_string () +
                    &index.to_string ();
                if let Ok ((
                    value, subst, var_vec
                )) = new_value_and_var_vec (self, prop_term) {
                    let mut proving = self.proving (
                        &value, &subst);
                    let subst_vec = proving.take_subst (*counter);
                    let obj = Obj::QueryRes (QueryRes {
                        var_vec,
                        subst_vec,
                    });
                    self.define (&name, &obj)?;
                    Ok (Some ((name, obj)))
                } else {
                    let obj = Obj::QueryRes (QueryRes {
                        var_vec: Vec::new (),
                        subst_vec: Vec::new (),
                    });
                    self.define (&name, &obj)?;
                    Ok (Some ((name, obj)))
                }
            }
            Def::Assert (
                prop_term
            ) => {
                if let Ok ((
                    value, subst
                )) = new_value (self, prop_term) {
                    let mut proving = self.proving (
                        &value, &subst);
                    if let None = proving.next_qed () {
                        ErrorInCtx::new ()
                            .head ("assert! fail")
                            .span (prop_term.span ())
                            .wrap_in_err ()
                    } else {
                        Ok (None)
                    }
                } else {
                    ErrorInCtx::new ()
                        .head ("assert! fail")
                        .line ("can not get value from term")
                        .span (prop_term.span ())
                        .wrap_in_err ()
                }
            }
            Def::AssertNot (
                prop_term
            ) => {
                if let Ok ((
                    value, subst
                )) = new_value (self, prop_term) {
                    let mut proving = self.proving (
                        &value, &subst);
                    if let Some (_) = proving.next_qed () {
                        ErrorInCtx::new ()
                            .head ("assert-not! fail")
                            .span (prop_term.span ())
                            .wrap_in_err ()
                    } else {
                        Ok (None)
                    }
                } else {
                    Ok (None)
                }
            }
            Def::Note => {
                Ok (None)
            }
            Def::Step (counter, prop_term) => {
                let name = "#".to_string () +
                    &index.to_string ();
                if let Ok ((
                    value, subst
                )) = new_value (self, prop_term) {
                    let mut proving = self.proving (
                        &value, &subst);
                    let step = proving.step_many (*counter);
                    let obj = Obj::StepRes (StepRes {
                        step,
                        deduction_queue: proving.deduction_queue,
                    });
                    self.define (&name, &obj)?;
                    Ok (Some ((name, obj)))
                } else {
                    Ok (None)
                }
            }
        }
    }
}

impl Module {
    pub fn prop_check (&self) -> Result <(), ErrorInCtx> {
        for name in self.obj_dic.keys () {
            if prop_name_symbol_p (name) {
                let span = Span {
                    hi: 0,
                    lo: 0,
                };
                let arg = Arg::Rec (Vec::new ());
                let prop_term = Term::Prop (
                    span, name.to_string (), arg);
                let (value, subst) = new_value (self, &prop_term)?;
                let mut proving = self.proving (&value, &subst);
                if let None = proving.next_qed () {
                    return ErrorInCtx::new ()
                        .head ("Module::prop_check")
                        .line (&format! ("fail on prop : {}", name))
                        .wrap_in_err ()
                }
            }
        }
        Ok (())
    }
}

impl Module {
    pub fn run (
        &mut self,
        input: &str,
    ) -> Result <Dic <Obj>, ErrorInCtx> {
        let syntax_table = SyntaxTable::default ();
        let mexp_vec = syntax_table.parse (input)?;
        let def_vec = mexp_vec_to_def_vec (&mexp_vec)?;
        let len = self.obj_dic.len ();
        let mut obj_dic = Dic::new ();
        for (index, def) in def_vec.iter () .enumerate () {
            if let Some ((
                name, obj
            )) = self.exe_def (len + index, def)? {
                obj_dic.ins (&name, Some (obj));
            }
        }
        self.prop_check ()?;
        Ok (obj_dic)
    }
}

use std::path::Path;
use std::fs;

impl Module {
    pub fn load (
        url: &str,
    ) -> Result <Module, ErrorInCtx> {
        let path = Path::new (url);
        let mut module = Module::new ();
        if path.is_file () {
            let input = fs::read_to_string (path) .unwrap ();
            module.run (&input)?;
            Ok (module)
        } else {
            ErrorInCtx::new ()
                .head ("Module::load")
                .line (&format! ("url = {}", url))
                .wrap_in_err ()
        }
    }
}

impl ToString for Module {
    fn to_string (&self) -> String {
        format_mexp_block (
            "$module-c",
            dic_to_lines (&self.obj_dic))
    }
}

pub fn report_obj_dic (obj_dic: &Dic <Obj>) -> String {
    let mut s = String::new ();
    for (name, obj) in obj_dic.iter () {
        match obj {
            Obj::SearchRes (_) => {
                s += name;
                s += " = ";
                s += &obj.to_string ();
                s += "\n";
            }
            Obj::QueryRes (_) => {
                s += name;
                s += " = ";
                s += &obj.to_string ();
                s += "\n";
            }
            Obj::StepRes (_) => {
                s += name;
                s += " = ";
                s += &obj.to_string ();
                s += "\n";
            }
            _ => {}
        }
    }
    s
}

impl Module {
    pub fn report (&self) -> String {
        report_obj_dic (&self.obj_dic)
    }
}

impl Module {
    fn get_prop (
        &self,
        name: &str,
    ) -> Result <(Value, Subst), ErrorInCtx> {
        match self.obj_dic.get (name) {
            Some (Obj::Disj (name_vec, binding_vec)) => {
                let (body, subst) = new_value_dic (
                    self, binding_vec)?;
                let disj = Value::Disj (Disj {
                    name: name.to_string (),
                    name_vec: name_vec.clone (),
                    body,
                });
                Ok ((disj, subst))
            }
            Some (Obj::Conj (binding_vec)) => {
                let (body, subst) = new_value_dic (
                    self, binding_vec)?;
                let conj = Value::Conj (Conj {
                    name: name.to_string (),
                    body,
                });
                Ok ((conj, subst))
            }
            Some (_) => {
                ErrorInCtx::new ()
                    .head ("Module::get_prop")
                    .line ("name is not bound to Disj or Conj")
                    .line (&format! ("name = {}", name))
                    .wrap_in_err ()
            }
            None => {
                ErrorInCtx::new ()
                    .head ("Module::get_prop")
                    .line (&format! ("undefined name = {}", name))
                    .wrap_in_err ()
            }
        }
    }
}

fn name_to_prop_name (cons_name: &str) -> String {
    let base_name = &cons_name[.. cons_name.len () - 2];
    format! ("{}-t", base_name)
}

fn name_to_cons_name (cons_name: &str) -> String {
    let base_name = &cons_name[.. cons_name.len () - 2];
    format! ("{}-c", base_name)
}

impl Module {
    fn get_new_data (
        &self,
        name: &str,
    ) -> Result <(Data, Subst), ErrorInCtx> {
        let prop_name = &name_to_prop_name (name);
        let (prop, subst) = self.get_prop (prop_name)?;
        let value_dic = prop.value_dic () .unwrap ();
        let data = Data {
            name: name_to_cons_name (name),
            body: value_dic.clone (),
        };
        Ok ((data, subst))
    }
}

fn new_value_dic (
    module: &Module,
    binding_vec: &Vec <Binding>,
) -> Result <(Dic <Value>, Subst), ErrorInCtx> {
    let mut subst = Subst::new ();
    let mut body = Dic::new ();
    let mut var_dic = Dic::new ();
    for binding in binding_vec {
        binding.bind (
            module,
            &mut subst,
            &mut body,
            &mut var_dic)?;
    }
    Ok ((body, subst))
}

fn new_value (
    module: &Module,
    term: &Term,
) -> Result <(Value, Subst), ErrorInCtx> {
    let mut subst = Subst::new ();
    let mut body = Dic::new ();
    let mut var_dic = Dic::new ();
    let value = term.value (
        module,
        &mut subst,
        &mut body,
        &mut var_dic,
        None)?;
    Ok ((value, subst))
}

fn new_value_and_var_vec (
    module: &Module,
    term: &Term,
) -> Result <(Value, Subst, Vec <Var>), ErrorInCtx> {
    let mut subst = Subst::new ();
    let mut body = Dic::new ();
    let mut var_dic = Dic::new ();
    let mut var_name_vec = Vec::new ();
    collect_var_name_vec_from_term (
        &mut var_name_vec, term);
    let mut var_vec = Vec::new ();
    for var_name in var_name_vec {
        let var = Var::new (&var_name);
        var_dic.ins (&var_name, Some (Value::Var (var.clone ())));
        var_vec.push (var);
    }
    let value = term.value (
        module,
        &mut subst,
        &mut body,
        &mut var_dic,
        None)?;
    Ok ((value, subst, var_vec))
}

fn collect_var_name_vec_from_term (
    var_name_vec: &mut Vec <String>,
    term: &Term,
) {
    match term {
        Term::Var (_span, name) => {
            if ! var_name_vec.contains (name) {
                var_name_vec.push (name.to_string ());
            }
        }
        Term::Cons (_span, _name, arg) => {
            collect_var_name_vec_from_arg (
                var_name_vec, arg);
        }
        Term::Prop (_span, _name, arg) => {
            collect_var_name_vec_from_arg (
                var_name_vec, arg);
        }
        Term::FieldRef (_span, _name) => {}
        Term::TypeOfType (_span) => {}
    }
}

fn collect_var_name_vec_from_arg (
    var_name_vec: &mut Vec <String>,
    arg: &Arg,
) {
    match arg {
        Arg::Vec (term_vec) => {
            for term in term_vec {
                collect_var_name_vec_from_term (
                    var_name_vec, term);
            }
        }
        Arg::Rec (binding_vec) => {
            for binding in binding_vec {
                match binding {
                    Binding::EqualTo (_name, term) |
                    Binding::Inhabit (_name, term) => {
                        collect_var_name_vec_from_term (
                            var_name_vec, term);
                    }
                }
            }
        }
    }
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
pub enum Def {
    Obj (String, Obj),
    Import (String, String),
    Search (String, usize, Term),
    NamelessSearch (usize, Term),
    Query (String, usize, Term),
    NamelessQuery (usize, Term),
    Assert (Term),
    AssertNot (Term),
    Note,
    Step (usize, Term),
    // StepBetween (usize, usize, Term),
}

impl Module {
    pub fn proving <'a> (
        &'a self,
        value: &Value,
        subst: &Subst,
    ) -> Proving <'a> {
        let mut tv_queue = VecDeque::new ();
        let root_tv = new_tv ("root", value);
        tv_queue.push_back (root_tv.clone ());
        let deduction = Deduction {
            subst: subst.clone (),
            root: Value::TypedVar (root_tv),
            tv_queue,
        };
        Proving {
            module: self,
            deduction_queue: vec! [deduction] .into (),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Proving <'a> {
    module: &'a Module,
    deduction_queue: VecDeque <Deduction>,
}

impl <'a> Proving <'a> {
    pub fn step (&mut self) -> Option <DeductionStep> {
        if let Some (
            mut deduction
        ) = self.deduction_queue.pop_front () {
            let deduction_step = deduction.step (self.module);
            match deduction_step.clone () {
                DeductionStep::Qed (_qed) => {}
                DeductionStep::More (deduction_queue) => {
                    for deduction in deduction_queue {
                        //// about searching
                        // push back  |   depth first
                        // push front | breadth first
                        self.deduction_queue.push_back (deduction);
                    }
                }
            }
            Some (deduction_step)
        } else {
            None
        }
    }
}

impl <'a> Proving <'a> {
    pub fn step_many (
        &mut self,
        mut counter: usize,
    ) -> Option <DeductionStep> {
        while counter > 0 {
            self.step ();
            counter -= 1;
        }
        self.step ()
    }
}

impl <'a> Proving <'a> {
    pub fn next_qed (&mut self) -> Option <Qed> {
        while let Some (
            mut deduction
        ) = self.deduction_queue.pop_front () {
            match deduction.step (self.module) {
                DeductionStep::Qed (qed) => {
                    return Some (qed);
                }
                DeductionStep::More (deduction_queue) => {
                    for deduction in deduction_queue {
                        //// about searching
                        // push back  |   depth first
                        // push front | breadth first
                        self.deduction_queue.push_back (deduction);
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

impl <'a> Proving <'a> {
    pub fn take_subst (&mut self, n: usize) -> Vec <Subst> {
        let mut vec = Vec::new ();
        for _ in 0..n {
            if let Some (qed) = self.next_qed () {
                vec.push (qed.subst)
            }
        }
        vec
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Deduction {
    subst: Subst,
    root: Value,
    tv_queue: VecDeque <TypedVar>,
}

impl Deduction {
    fn step (&mut self, module: &Module) -> DeductionStep {
        if let Some (
            tv,
        ) = self.tv_queue.pop_front () {
            let tv_matrix = tv.fulfill (module, &self.subst);
            let mut deduction_queue = VecDeque::new ();
            for (tv_vec, new_subst) in tv_matrix {
                let mut deduction = self.clone ();
                deduction.subst = new_subst;
                //// about searching again
                // we have two level of searching
                // we need to use breadth first again
                for tv in tv_vec.into_iter () {
                    deduction.tv_queue.push_back (tv);
                }
                // for tv in tv_vec.into_iter () .rev () {
                //     deduction.tv_queue.push_front (tv);
                // }
                deduction_queue.push_back (deduction)
            }
            DeductionStep::More (deduction_queue)
        } else {
            DeductionStep::Qed (Qed {
                subst: self.subst.clone (),
                root: self.root.clone (),
            })
        }
    }
}

impl ToString for Deduction {
    fn to_string (&self) -> String {
        let mut tv_vec: Vec <TypedVar> = Vec::new ();
        let mut reified_tv_vec: Vec <Value> = Vec::new ();
        for tv in &self.tv_queue {
            tv_vec.push (tv.clone ());
            reified_tv_vec.push (
                self.subst.reify (&Value::TypedVar (
                    tv.clone ())));
        }
        format! (
            "$deduction-c {{
                 root = {}
                 tv-queue = [ {} ]
                 reified-tv-queue = [ {} ] }}",
            self.subst.reify (&self.root) .to_string (),
            vec_to_string (&tv_vec, " "),
            vec_to_string (&reified_tv_vec, " "))

    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum DeductionStep {
    Qed (Qed),
    More (VecDeque <Deduction>),
}

impl ToString for DeductionStep {
    fn to_string (&self) -> String {
        match self {
            DeductionStep::Qed (qed) => {
                format! (
                    "$deduction-step:qed-c {{ root = {} }}",
                    qed.to_string ())
            }
            DeductionStep::More (queue) => {
                let mut vec: Vec <Deduction> = Vec::new ();
                for x in queue {
                    vec.push (x.clone ());
                }
                format! (
                    "$deduction-step:queue-c {{ queue = [ {} ] }}",
                    vec_to_string (&vec, " "))
            }
        }
    }
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
        self.subst.reify (&self.root) .to_string ()
    }
}

const GRAMMAR: &'static str = r#"
Def::Obj = { name? "=" Obj }
Def::Import = { name? "=" "import!" '(' url? ')' }
Def::Search = { name? "=" "search!" '(' nat-num? ')' Term::Prop }
Def::NamelessSearch = { "search!" '(' nat-num? ')' Term::Prop }
Def::Query = { name? "=" "query!" '(' nat-num? ')' Term::Prop }
Def::NamelessQuery = { "query!" '(' nat-num? ')' Term::Prop }
Def::Assert = { "assert!" Term::Prop }
Def::AssertNot = { "assert-not!" Term::Prop }
Def::Note = { "note!" '{' list (mexp) '}' }
Def::Step = { "step!" (nat-num?) Term::Prop }
Def::StepBetween = { "step-between!" (nat-num? nat-num?) Term::Prop }

Obj::Disj = { "disj" '(' list (prop-name?) ')' Arg::Rec }
Obj::Conj = { "conj" Arg::Rec }
[TODO] Obj::Module = { "module" '{' list (Def) '}' }

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

fn mexp_to_disj_obj <'a> (
    mexp: &Mexp <'a>,
) -> Result <Obj, ErrorInCtx> {
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
        Ok (Obj::Disj (
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

fn mexp_to_conj_obj <'a> (
    mexp: &Mexp <'a>,
) -> Result <Obj, ErrorInCtx> {
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
        Ok (Obj::Conj (
            mexp_vec_to_binding_vec (body)?))
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_prop_obj <'a> (
    mexp: &Mexp <'a>,
) -> Result <Obj, ErrorInCtx> {
    mexp_to_disj_obj (mexp)
        .or (mexp_to_conj_obj (mexp))
}

fn mexp_to_obj_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
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
            Ok (Def::Obj (
                symbol.to_string (),
                mexp_to_prop_obj (rhs)?))
        } else {
            ErrorInCtx::new ()
                .head ("mexp_to_obj_def")
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

fn mexp_to_search_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
    if let Mexp::Infix {
        op: "=",
        lhs: box Mexp::Sym {
            symbol: name,
            ..
        },
        rhs: box Mexp::Apply {
            head: box Mexp::Apply {
                head: box Mexp::Sym {
                    symbol: "search!",
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
                    .line ("fail to parse usize num in `search!`")
                    .line (&format! ("symbol = {}", symbol))
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ();
            }
            if let [
                prop_mexp
            ] = &body2 [..] {
                Ok (Def::Search (
                    name.to_string (),
                    result.unwrap (),
                    mexp_to_prop_term (prop_mexp)?))
            } else {
                ErrorInCtx::new ()
                    .line ("fail to parse `search!`'s body arg")
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ()
            }
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse `search!`'s first arg")
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

fn mexp_to_nameless_search_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "search!",
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
                    .line ("fail to parse usize num in `search!`")
                    .line (&format! ("symbol = {}", symbol))
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ();
            }
            if let [
                prop_mexp
            ] = &body2 [..] {
                Ok (Def::NamelessSearch (
                    result.unwrap (),
                    mexp_to_prop_term (prop_mexp)?))
            } else {
                ErrorInCtx::new ()
                    .line ("fail to parse `search!`'s body arg")
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ()
            }
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse `search!`'s first arg")
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


fn mexp_to_query_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
    if let Mexp::Infix {
        op: "=",
        lhs: box Mexp::Sym {
            symbol: name,
            ..
        },
        rhs: box Mexp::Apply {
            head: box Mexp::Apply {
                head: box Mexp::Sym {
                    symbol: "query!",
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
                    .line ("fail to parse usize num in `query!`")
                    .line (&format! ("symbol = {}", symbol))
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ();
            }
            if let [
                prop_mexp
            ] = &body2 [..] {
                Ok (Def::Query (
                    name.to_string (),
                    result.unwrap (),
                    mexp_to_prop_term (prop_mexp)?))
            } else {
                ErrorInCtx::new ()
                    .line ("fail to parse `query!`'s body arg")
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ()
            }
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse `query!`'s first arg")
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

fn mexp_to_nameless_query_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "query!",
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
                    .line ("fail to parse usize num in `query!`")
                    .line (&format! ("symbol = {}", symbol))
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ();
            }
            if let [
                prop_mexp
            ] = &body2 [..] {
                Ok (Def::NamelessQuery (
                    result.unwrap (),
                    mexp_to_prop_term (prop_mexp)?))
            } else {
                ErrorInCtx::new ()
                    .line ("fail to parse `query!`'s body arg")
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ()
            }
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse `query!`'s first arg")
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


fn mexp_to_import_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
    if let Mexp::Infix {
        op: "=",
        lhs: box Mexp::Sym {
            symbol: name,
            ..
        },
        rhs: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "import!",
                ..
            },
            arg: MexpArg::Tuple {
                body,
                ..
            },
            ..
        },
        ..
    } = mexp {
        if let [
            Mexp::Str { string, .. }
        ] = &body [..] {
            Ok (Def::Import (
                name.to_string (),
                string.to_string ()))
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse `import!`'s arg")
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

fn mexp_to_assert_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol: "assert!",
            ..
        },
        arg: MexpArg::Block {
            body,
            ..
        },
        ..
    } = mexp {
        if let [
            prop_mexp
        ] = &body [..] {
            Ok (Def::Assert (
                mexp_to_prop_term (prop_mexp)?))
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse `assert!`'s body arg")
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

fn mexp_to_assert_not_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol: "assert-not!",
            ..
        },
        arg: MexpArg::Block {
            body,
            ..
        },
        ..
    } = mexp {
        if let [
            prop_mexp
        ] = &body [..] {
            Ok (Def::AssertNot (
                mexp_to_prop_term (prop_mexp)?))
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse `assert!`'s body arg")
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

fn mexp_to_note_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol: "note",
            ..
        },
        arg: MexpArg::Block {
            ..
        },
        ..
    } = mexp {
        Ok (Def::Note)
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_step_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "step!",
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
                    .line ("fail to parse usize num in `step!`")
                    .line (&format! ("symbol = {}", symbol))
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ();
            }
            if let [
                prop_mexp
            ] = &body2 [..] {
                Ok (Def::Step (
                    result.unwrap (),
                    mexp_to_prop_term (prop_mexp)?))
            } else {
                ErrorInCtx::new ()
                    .line ("fail to parse `step!`'s body arg")
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ()
            }
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse `step!`'s first arg")
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


fn mexp_to_def <'a> (
    mexp: &Mexp <'a>,
) -> Result <Def, ErrorInCtx> {
    mexp_to_obj_def (mexp)
        .or (mexp_to_import_def (mexp))
        .or (mexp_to_search_def (mexp))
        .or (mexp_to_nameless_search_def (mexp))
        .or (mexp_to_query_def (mexp))
        .or (mexp_to_nameless_query_def (mexp))
        .or (mexp_to_assert_def (mexp))
        .or (mexp_to_assert_not_def (mexp))
        .or (mexp_to_note_def (mexp))
        .or (mexp_to_step_def (mexp))
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

fn mexp_vec_to_def_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <Def>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_def (&mexp)?);
    }
    Ok (vec)
}

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

const NAT_EXAMPLE: &'static str = r#"
nat-t = disj (
    zero-t
    succ-t
) {}

zero-t = conj {}

succ-t = conj {
    prev : nat-t
}

nat-add-t = disj (
    zero-add-t
    succ-add-t
) {
    a : nat-t
    b : nat-t
    c : nat-t
}

zero-add-t = conj {
    a : nat-t
    b : nat-t
    c : nat-t
    a = zero-c
    c = b
}

succ-add-t = conj {
    a : nat-t
    b : nat-t
    c : nat-t
    a = succ-c (:a)
    c = succ-c (:c)
    prev : nat-add-t (:a b :c)
}

assert! {
    nat-add-t (zero-c zero-c zero-c)
}

assert! {
    nat-add-t (zero-c succ-c (zero-c) succ-c (zero-c))
}

assert! {
    nat-add-t (succ-c (zero-c) zero-c succ-c (zero-c))
}
"#;

#[test]
fn test_module_get_prop () {
    let mut module = Module::new ();
    let input = NAT_EXAMPLE;
    let ctx = ErrorCtx::new () .body (input);
    match module.run (input) {
        Ok (_obj_dic) => {}
        Err (error) => {
            error.print (ctx.clone ());
        }
    }
    for name in module.obj_dic.keys () {
        if name.ends_with ("-t") {
            match module.get_prop (name) {
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
                    panic! ("test_module_get_prop");
                }
            }
        }
    }
}

#[test]
fn test_module_output () {
    let mut module = Module::new ();
    let input = NAT_EXAMPLE;
    let ctx = ErrorCtx::new () .body (input);
    match module.run (input) {
        Ok (_obj_dic) => {
            println! ("{}", module.report ());
        }
        Err (error) => {
            error.print (ctx.clone ());
        }
    }
}
