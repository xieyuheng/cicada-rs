#![feature (uniform_paths)]
#![feature (bind_by_move_pattern_guards)]
#![feature (box_patterns)]
#![feature (box_syntax)]

#![allow (dead_code)]

use std::fmt;
use std::sync::Arc;
use std::collections::VecDeque;
use std::collections::HashMap;
use uuid::Uuid;
use dic::Dic;
use error_report::{
    // Span,
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
    Arg,
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

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub enum Term {
    Var   (VarTerm),
    Tuple (TupleTerm),
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub struct VarTerm {
    name: String,
    id: Option <Id>,
}

impl ToString for VarTerm {
    fn to_string (&self) -> String {
        if let Some (id) = &self.id {
            // format! (
            //     "{}#{}",
            //     self.name,
            //     id.to_string ())
            format! (
                "{}#{}",
                self.name,
                &id.to_string () [0..2])
        } else {
            format! ("{}", self.name)
        }
    }
}

#[derive (Clone)]
#[derive (PartialEq, Eq, Hash)]
pub struct Id (uuid::adapter::Hyphenated);

impl Id {
    fn new () -> Self {
        Id (Uuid::new_v4 () .to_hyphenated ())
    }
}

impl fmt::Debug for Id {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "{}", self.0)
    }
}

impl ToString for Id {
    fn to_string (&self) -> String {
        format! ("{}", self.0)
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub struct TupleTerm {
    head: String,
    body: Vec <Term>,
}

impl ToString for TupleTerm {
    fn to_string (&self) -> String {
        if self.body.len () == 0 {
            format! ("{}", self.head)
        } else {
            format! (
                "{} ({})",
                self.head,
                vec_to_string (&self.body, " "))
        }
    }
}

impl Term {
    fn var (s: &str) -> Term {
        Term::Var (VarTerm {
            name: s.to_string (),
            id: Some (Id::new ()),
        })
    }
}

impl Term {
    fn var_no_id (s: &str) -> Term {
        Term::Var (VarTerm {
            name: s.to_string (),
            id: None,
        })
    }
}

impl Term {
    fn tuple (h: &str, vec: Vec <Term>) -> Term {
        Term::Tuple (TupleTerm {
            head: h.to_string (),
            body: vec,
        })
    }
}

impl Term {
    fn duplicate (
        &self,
        var_map: &mut HashMap <VarTerm, VarTerm>,
    ) -> Term {
        match self {
            Term::Var (var) => {
                if let Some (dup_var) = var_map.get (var) {
                    Term::Var (dup_var.clone ())
                } else {
                    let new_var = VarTerm {
                        name: var.name.clone (),
                        id: Some (Id::new ()),
                    };
                    var_map.insert (
                        var.clone (),
                        new_var.clone ());
                    Term::Var (new_var)
                }
            }
            Term::Tuple (tuple) => {
                let mut new_tuple = TupleTerm {
                    head: tuple.head.clone (),
                    body: Vec::new (),
                };
                for term in &tuple.body {
                    new_tuple.body.push (
                        term.duplicate (var_map));
                }
                Term::Tuple (new_tuple)
            }
        }
    }
}

impl Term {
    fn fresh (&self) -> Term {
        let mut var_map = HashMap::new ();
        self.duplicate (&mut var_map)
    }
}

impl ToString for Term {
    fn to_string (&self) -> String {
        match self {
            Term::Var (var) => var.to_string (),
            Term::Tuple (tuple) => tuple.to_string (),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub enum Subst {
    Null,
    Cons {
       var: VarTerm,
       term: Term,
       next: Arc <Subst>,
    },
}

impl Subst {
    fn new () -> Self {
        Subst::Null
    }
}

impl Subst {
    fn extend (&self, var: VarTerm, term: Term) -> Self {
        Subst::Cons {
            var, term,
            next: Arc::new (self.clone ()),
        }
    }
}

impl Subst {
    pub fn find (&self, var: &VarTerm) -> Option <&Term> {
        match self {
            Subst::Null => None,
            Subst::Cons {
                var: first, term, next,
            } => {
                if first == var {
                    Some (term)
                } else {
                    next.find (var)
                }
            }
        }
    }
}

impl Subst {
    pub fn walk (&self, term: &Term) -> Term {
        match term {
            Term::Var (var) => {
                if let Some (new_term) = self.find (var) {
                    self.walk (new_term)
                } else {
                    term.clone ()
                }
            }
            _ => term.clone ()
        }
    }
}

impl Subst {
    pub fn unify (
        &self,
        u: &Term,
        v: &Term,
    ) -> Option <Subst> {
        let u = self.walk (u);
        let v = self.walk (v);
        match (u, v) {
            (Term::Var (u),
             Term::Var (v),
            ) if u == v => {
                Some (self.clone ())
            }
            (Term::Var (u), v) => {
                Some (self.extend (u, v))
            }
            (u, Term::Var (v)) => {
                Some (self.extend (v, u))
            }
            (Term::Tuple (ut),
             Term::Tuple (vt),
            ) => {
                if ut.head != vt.head {
                    return None;
                }
                if ut.body.len () != vt.body.len () {
                    return None;
                }
                let mut subst = self.clone ();
                let zip = ut.body.iter () .zip (vt.body.iter ());
                for (u, v) in zip {
                    subst = subst.unify (u, v)?;
                }
                Some (subst)
            }
        }
    }
}

impl ToString for Subst {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        let mut subst = self.clone ();
        while let Subst::Cons {
            var,
            term,
            next,
        } = subst {
            s += &var.to_string ();
            s += " = ";
            s += &term.to_string ();
            s += "\n";
            subst = (*next) .clone ();
        }
        s
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub enum Prop {
    Disj (Vec <String>),
    Conj (Vec <Term>, Vec <Query>),
}

impl Prop {
    fn duplicate (
        &self,
        var_map: &mut HashMap <VarTerm, VarTerm>,
    ) -> Prop {
        match self {
            Prop::Disj (_name_vec) => {
                self.clone ()
            }
            Prop::Conj (args, query_vec) => {
                let mut new_args = Vec::new ();
                for arg in args {
                    new_args.push (
                        arg.duplicate (var_map))
                }
                let mut new_query_vec = Vec::new ();
                for query in query_vec {
                    new_query_vec.push (
                        query.duplicate (var_map))
                }
                Prop::Conj (new_args, new_query_vec)
            }
        }
    }
}

impl Prop {
    fn fresh (&self) -> Prop {
        let mut var_map = HashMap::new ();
        self.duplicate (&mut var_map)
    }
}

impl Prop {
    fn apply (
        self,
        args: &Vec <Term>,
        mut subst: Subst,
    ) -> Option <(Vec <Vec <Arc <Query>>>, Subst)> {
        match self {
            Prop::Disj (name_vec) => {
                let mut query_matrix = Vec::new ();
                for name in name_vec {
                    let query = Arc::new (Query {
                        name: name,
                        args: args.clone (),
                    });
                    query_matrix.push (vec! [query]);
                }
                Some ((query_matrix, subst))
            }
            Prop::Conj (terms, query_vec) => {
                if args.len () != terms.len () {
                    eprintln! ("- [warning] Prop::apply");
                    eprintln! ("  arity mismatch");
                    return None;
                }
                let zip = args.iter () .zip (terms.iter ());
                for (u, v) in zip {
                    if let Some (
                        new_subst
                    ) = subst.unify (u, v) {
                        subst = new_subst;
                    } else {
                        return None;
                    }
                }
                let query_matrix = vec! [
                    query_vec
                        .into_iter ()
                        .map (|x| Arc::new (x))
                        .collect ()
                ];
                Some ((query_matrix, subst))
            }
        }
    }
}

impl ToString for Prop {
    fn to_string (&self) -> String {
        match self {
            Prop::Disj (name_vec) => {
                format! (
                    "disj ({})",
                    vec_to_string (&name_vec, " "))
            }
            Prop::Conj (terms, query_vec) => {
                format! (
                    "conj ({}) {}",
                    vec_to_string (&terms, " "),
                    if query_vec.len () == 0 {
                        format! ("{{}}")
                    } else {
                        format! (
                            "{{ {} }}",
                            vec_to_string (&query_vec, " "))
                    })
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub struct Query {
    name: String,
    args: Vec <Term>,
}

impl Query {
    fn duplicate (
        &self,
        var_map: &mut HashMap <VarTerm, VarTerm>,
    ) -> Query {
        let mut new_args = Vec::new ();
        for arg in &self.args {
            new_args.push (
                arg.duplicate (var_map));
        }
        Query {
            name: self.name.clone (),
            args: new_args,
        }
    }
}

impl ToString for Query {
    fn to_string (&self) -> String {
        format! (
            "{} ({})",
            self.name,
            vec_to_string (&self.args, " "))
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Wissen {
    prop_dic: Dic <Prop>,
}

impl Wissen {
    fn new () -> Self {
        Wissen {
            prop_dic: Dic::new (),
        }
    }
}

impl Wissen {
    fn find_prop (&self, name: &str) -> Option <Prop> {
        if let Some (
            prop
        ) = self.prop_dic.get (name) {
            Some (prop.fresh ())
        } else {
            None
        }
    }
}

impl Wissen {
    fn prove <'a> (
        &'a self,
        query: &Query,
    ) -> Proving <'a> {
        let proof = Proof {
            wissen: self,
            subst: Subst::new (),
            query_queue: vec! [Arc::new (query.clone ())] .into (),
        };
        Proving {
            proof_queue: vec! [proof] .into (),
        }
    }
}

impl ToString for Wissen {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        for (name, prop) in self.prop_dic.iter () {
            s += name;
            s += " = ";
            s += &prop.to_string ();
            s += "\n";
        }
        format! ("<wissen>\n{}</wissen>\n", s)
    }
}

impl Wissen {
    fn define_prop (&mut self, name: &str, prop: &Prop) {
       self.prop_dic.ins (name, Some (prop.clone ()));
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Statement {
    DefineProp (String, Prop),
    Query (Query),
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Proving <'a> {
    proof_queue: VecDeque <Proof <'a>>,
}

impl <'a> Proving <'a> {
    fn next_subst (&mut self) -> Option <Subst> {
        while let Some (
            proof
        ) = self.proof_queue.pop_front () {
            // println! (
            //     "- Proving::next_subst = {}",
            //     proof.to_string ());
            match proof.step () {
                ProofStep::Finished (subst) => {
                    return Some (subst);
                }
                ProofStep::MoreTodo (proof_queue) => {
                    for proof in proof_queue {
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
    fn take_subst (&mut self, n: usize) -> Vec <Subst> {
        let mut subst_vec = Vec::new ();
        for _ in 0..n {
            if let Some (subst) = self.next_subst () {
                subst_vec.push (subst)
            }
        }
        subst_vec
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Proof <'a> {
    wissen: &'a Wissen,
    subst: Subst,
    query_queue: VecDeque <Arc <Query>>,
}

impl <'a> Proof <'a> {
    fn step (mut self) -> ProofStep <'a> {
        if let Some (query) = self.query_queue.pop_front () {
            if let Some (
                prop
            ) = self.wissen.find_prop (&query.name) {
                let mut proof_queue = VecDeque::new ();
                if let Some (
                    (query_matrix, new_subst)
                ) = prop.apply (&query.args, self.subst.clone ()) {
                    for query_vec in query_matrix {
                        let mut proof = self.clone ();
                        proof.subst = new_subst.clone ();
                        let rev = query_vec.into_iter () .rev ();
                        for query in rev {
                            proof.query_queue.push_front (query);
                        }
                        proof_queue.push_back (proof);
                    }
                    ProofStep::MoreTodo (proof_queue)
                } else {
                    ProofStep::Fail
                }
            } else {
                eprintln! ("- [warning] Proof::step");
                eprintln! ("  undefined prop : {}", query.name);
                ProofStep::Fail
            }
        } else {
            ProofStep::Finished (self.subst)
        }
    }
}

impl <'a> ToString for Proof <'a> {
    fn to_string (&self) -> String {
        let query_vec: &Vec <Query> = &self.query_queue
            .iter ()
            .map (|x| (**x) .clone ())
            .collect ();
        format! (
            "<proof>\n\
            <query_queue>\n\
            {}</query_queue>\n\
            <subst>\n\
            {}</subst>\n\
            </proof>\n",
            vec_to_lines (query_vec),
            self.subst.to_string ())
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum ProofStep <'a> {
    Finished (Subst),
    MoreTodo (VecDeque <Proof <'a>>),
    Fail,
}

// Statement::DefineProp = { prop-name? "=" Prop }
// Statement::Query = { Query }

// Prop::Disj = {
//     "disj" '('
//         list (prop-name?)
//    ')'
// }
// Prop::Conj = {
//     "conj" '('
//         list (Term)
//    ')' '{'
//         list (Query)
//    '}'
// }

// Term::Var = {
//     unique-var-name?
// }
// Term::Tuple = {
//     tuple-name? '('
//         list (Term)
//     ')'
// }

// Query::Tuple = {
//     prop-name? '('
//         list (Term)
//     ')'
// }

fn note_about_mexp_syntax_of_prop () -> ErrorMsg {
    ErrorMsg::new ()
        .head (r#"grammar of Prop"#)
}

fn mexp_to_prop_name <'a> (
    mexp: &Mexp <'a>,
) -> Result <String, ErrorInCtx> {
    if let Mexp::Sym {
        symbol,
        ..
    } = mexp {
        if symbol.ends_with ("-t") {
            Ok (symbol.to_string ())
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name")
                .line ("which must end with `-t`")
                .line (&format! ("but found : {}", symbol))
                .span (mexp.span ())
                .note (note_about_mexp_syntax_of_prop ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .line ("expecting prop name")
            .line (&format! ("but found : {}", mexp.to_string ()))
            .span (mexp.span ())
            .wrap_in_err ()
    }
}

fn mexp_to_disj_prop <'a> (
    mexp: &Mexp <'a>,
) -> Result <Prop, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol: "disj",
            ..
        },
        arg: Arg::Tuple {
            body,
            ..
        },
        ..
    } = mexp {
        Ok (Prop::Disj (mexp_vec_to_prop_name_vec (body)?))
    } else {
        ErrorInCtx::new ()
            .head ("unknown mexp")
            .span (mexp.span ())
            .note (note_about_mexp_syntax_of_prop ())
            .wrap_in_err ()
    }
}

fn mexp_to_query <'a> (
    mexp: &Mexp <'a>,
) -> Result <Query, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol,
            ..
        },
        arg: Arg::Tuple {
            body,
            ..
        },
        ..
    } = mexp {
        if symbol.ends_with ("-t") {
            Ok (Query {
                name: symbol.to_string (),
                args: mexp_vec_to_term_vec (body)?,
            })
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name")
                .line ("which must end with `-t`")
                .line (&format! ("but found : {}", symbol))
                .span (mexp.span ())
                .note (note_about_mexp_syntax_of_prop ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("unknown mexp")
            .span (mexp.span ())
            .note (note_about_mexp_syntax_of_prop ())
            .wrap_in_err ()
    }
}

fn mexp_to_term <'a> (
    mexp: &Mexp <'a>,
) -> Result <Term, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol,
            ..
        },
        arg: Arg::Tuple {
            body,
            ..
        },
        ..
    } = mexp {
        if symbol.ends_with ("-c") {
            Ok (Term::tuple (
                symbol,
                mexp_vec_to_term_vec (body)?))
        } else {
            ErrorInCtx::new ()
                .line ("expecting tuple name")
                .line ("which must end with `-c`")
                .line (&format! ("but found : {}", symbol))
                .span (mexp.span ())
                .note (note_about_mexp_syntax_of_prop ())
                .wrap_in_err ()
        }
    } else if let Mexp::Sym {
        symbol,
        ..
    } = mexp {
        if symbol.ends_with ("-c") {
            Ok (Term::tuple (symbol, vec! []))
        } else if symbol.ends_with ("-t") {
            ErrorInCtx::new ()
                .line ("expecting tuple name or var")
                .line ("but found prop name which end with `-t`")
                .line (&format! ("prop name : {}", symbol))
                .span (mexp.span ())
                .note (note_about_mexp_syntax_of_prop ())
                .wrap_in_err ()
        } else {
            Ok (Term::var_no_id (symbol))
        }
    } else {
        ErrorInCtx::new ()
            .head ("unknown mexp")
            .span (mexp.span ())
            .note (note_about_mexp_syntax_of_prop ())
            .wrap_in_err ()
    }
}

fn mexp_to_conj_prop <'a> (
    mexp: &Mexp <'a>,
) -> Result <Prop, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "conj",
                ..
            },
            arg: Arg::Tuple {
                body: body1,
                ..
            },
            ..
        },
        arg: Arg::Block {
            body: body2,
            ..
        },
        ..
    } = mexp {
        Ok (Prop::Conj (mexp_vec_to_term_vec (body1)?,
                        mexp_vec_to_query_vec (body2)?))
    } else {
        ErrorInCtx::new ()
            .head ("unknown mexp")
            .span (mexp.span ())
            .note (note_about_mexp_syntax_of_prop ())
            .wrap_in_err ()
    }
}

fn mexp_to_prop <'a> (
    mexp: &Mexp <'a>,
) -> Result <Prop, ErrorInCtx> {
    mexp_to_disj_prop (mexp)
        .or (mexp_to_conj_prop (mexp))
}

fn mexp_to_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    if let Ok (query) = mexp_to_query (mexp) {
        Ok (Statement::Query (query))
    } else if let Mexp::Infix {
        op: "=",
        lhs: box Mexp::Sym {
            symbol,
            ..
        },
        rhs,
        ..
    } = mexp {
        if symbol.ends_with ("-t") {
            Ok (Statement::DefineProp (
                symbol.to_string (),
                mexp_to_prop (rhs)?))
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name")
                .line ("which must end with `-t`")
                .line (&format! ("but found : {}", symbol))
                .span (mexp.span ())
                .note (note_about_mexp_syntax_of_prop ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("unknown mexp")
            .span (mexp.span ())
            .note (note_about_mexp_syntax_of_prop ())
            .wrap_in_err ()
    }
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

fn mexp_vec_to_query_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <Query>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_query (&mexp)?);
    }
    Ok (vec)
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

fn mexp_vec_to_statement_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <Statement>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_statement (&mexp)?);
    }
    Ok (vec)
}

#[test]
fn test_unify () {
    let u = Term::var ("u");
    let v = Term::var ("v");
    let subst = Subst::new () .unify (
        &Term::tuple ("pair-c", vec! [
            u.clone (),
            v.clone (),
        ]),
        &Term::tuple ("pair-c", vec! [
            v.clone (),
            Term::tuple ("hi-c", vec! []),
        ])) .unwrap ();
    println! ("- unify : \n{}", subst.to_string ());
}

#[test]
fn test_love () {
    let mut wissen = Wissen::new ();
    let prop = Prop::Conj (
        vec! [Term::tuple ("you-c", vec! [])],
        vec! []);
    wissen.define_prop ("love-t", &prop);
    let query = Query {
        name: "love-t".to_string (),
        args: vec! [Term::var ("u")],
    };
    let mut proving = wissen.prove (&query);
    while let Some (subst) = proving.next_subst () {
        println! ("- love : \n{}", subst.to_string ());
    }
}

#[cfg (test)]
const LIST_APPEND_EXAMPLE: &'static str = "
list-append-t = disj (
    zero-append-t
    succ-append-t
)
zero-append-t = conj (null-c succ succ) {}
succ-append-t = conj (
    cons-c (car cdr)
    succ
    cons-c (car o-cdr)
) {
    list-append-t (cdr succ o-cdr)
}

list-append-t (x y z)
";

#[test]
fn test_list_append () {
    let mut wissen = Wissen::new ();
    let list_append_t = Prop::Disj (
        vec! [
            "zero-append-t".to_string (),
            "succ-append-t".to_string (),
        ]);
    wissen.define_prop ("list-append-t", &list_append_t);
    let succ = Term::var ("succ");
    let zero_append_t = Prop::Conj (
        vec! [
            Term::tuple ("null-c", vec! []),
            succ.clone (),
            succ
        ],
        vec! []);
    wissen.define_prop ("zero-append-t", &zero_append_t);
    let car = Term::var ("car");
    let cdr = Term::var ("cdr");
    let succ = Term::var ("succ");
    let o_cdr = Term::var ("o-cdr");
    let succ_append_t = Prop::Conj (
        vec! [
            Term::tuple ("cons-c",
                         vec! [car.clone (),
                               cdr.clone ()]),
            succ.clone (),
            Term::tuple ("cons-c",
                         vec! [car,
                               o_cdr.clone ()]),
        ],
        vec! [
            Query {
                name: "list-append-t".to_string (),
                args: vec! [
                    cdr,
                    succ,
                    o_cdr,
                ],
            }
        ]);
    wissen.define_prop ("succ-append-t", &succ_append_t);
    let query = Query {
        name: "list-append-t".to_string (),
        args: vec! [Term::var ("x"),
                    Term::var ("y"),
                    Term::var ("z")],
    };
    let mut proving = wissen.prove (&query);
    for subst in proving.take_subst (6) {
        println! ("- append : \n{}", subst.to_string ());
    }
}

#[test]
fn test_mexp () -> Result <(), ErrorInCtx> {
    let mut wissen = Wissen::new ();
    let input = LIST_APPEND_EXAMPLE;
    let syntax_table = SyntaxTable::default ();
    let mexp_vec = syntax_table.parse (input)?;
    let statement_vec = mexp_vec_to_statement_vec (&mexp_vec)?;
    for statement in &statement_vec {
        if let Statement::DefineProp (name, prop) = statement {
            wissen.define_prop (name, prop);
        }
    }
    for statement in &statement_vec {
        if let Statement::Query (query) = statement {
            let mut proving = wissen.prove (query);
            for subst in proving.take_subst (6) {
                println! ("- mexp =\n{}", subst.to_string ());
            }
        }
    }
    Ok (())
}
