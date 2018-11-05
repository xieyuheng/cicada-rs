#![feature (uniform_paths)]
#![feature (bind_by_move_pattern_guards)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]
#![allow (non_camel_case_types)]

use std::fmt;
use std::sync::Arc;
use std::collections::VecDeque;
use std::collections::HashMap;
use uuid::Uuid;
use dic::Dic;

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
    id: Id,
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

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub struct TupleTerm {
    head: String,
    body: Vec <Term>,
}

impl Term {
    fn var (s: &str) -> Term {
        Term::Var (VarTerm {
            name: s.to_string (),
            id: Id::new (),
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
                        id: Id::new (),
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

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Wissen {
    prop_dic: Dic <Prop>,
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
        query: Query,
    ) -> Proving <'a> {
        let proof = Proof {
            wissen: self,
            subst: Subst::new (),
            query_queue: vec! [Arc::new (query)] .into (),
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
    fn next_subst (&mut self) -> Option <Subst> {
        while let Some (
            proof
        ) = self.proof_queue.pop_front () {
            // println! ("proof = {:?}", proof);
            // println! ("self = {:?}", self);
            match proof.step () {
                ProofStep::Finished (subst) => {
                    return Some (subst);
                }
                ProofStep::MoreTodo (proof_queue) => {
                    for proof in proof_queue {
                        // self.proof_queue.push_front (proof);
                        self.proof_queue.push_back (proof);
                    }
                }
                ProofStep::Fail => {}
            }
        }
        return None;
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

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum ProofStep <'a> {
    Finished (Subst),
    MoreTodo (VecDeque <Proof <'a>>),
    Fail,
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
            Term::tuple ("hi", vec! []),
        ]));
    println! ("- unify : {:#?}", subst.unwrap ());
}

#[test]
fn test_love () {
    let mut wissen = Wissen {
        prop_dic: Dic::new (),
    };
    let prop = Prop::Conj (
        vec! [Term::tuple ("you", vec! [])],
        vec! []);
    wissen.prop_dic.ins ("love-t", Some (prop));
    let query = Query {
        name: "love-t".to_string (),
        args: vec! [Term::var ("u")],
    };
    let mut proving = wissen.prove (query);
    while let Some (subst) = proving.next_subst () {
        println! ("- love : {:#?}", subst);
    }
}

// list-append-t = disj (
//     zero-append-t
//     succ-append-t
// ) {} []
// zero-append-t = conj (null-c succ succ) {}
// succ-append-t = conj (
//     cons-c (car cdr)
//     succ
//     cons-c (car o-cdr)
// ) {
//     list-append-t (cdr succ o-cdr)
// }

#[test]
fn test_list_append () {
    let mut wissen = Wissen {
        prop_dic: Dic::new (),
    };
    let list_append_t = Prop::Disj (
        vec! [
            "zero-append-t".to_string (),
            "succ-append-t".to_string (),
        ]);
    wissen.prop_dic.ins ("list-append-t", Some (list_append_t));
    let succ = Term::var ("succ");
    let zero_append_t = Prop::Conj (
        vec! [
            Term::tuple ("null-c", vec! []),
            succ.clone (),
            succ
        ],
        vec! []);
    wissen.prop_dic.ins ("zero-append-t", Some (zero_append_t));
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
    wissen.prop_dic.ins ("succ-append-t", Some (succ_append_t));
    let query = Query {
        name: "list-append-t".to_string (),
        args: vec! [Term::var ("x"),
                    Term::var ("y"),
                    Term::var ("z")],
    };
    let mut proving = wissen.prove (query);
    let mut counter = 10;
    while let Some (subst) = proving.next_subst () {
        counter -= 1;
        if counter > 0 {
            println! ("- append : {:#?}", subst);
        } else {
            break;
        }
    }
}

#[test]
fn test_mexp () {

}
