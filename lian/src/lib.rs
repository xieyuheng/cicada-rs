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
#[derive (PartialEq)]
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
    Disj (Vec <Prop>),
    Conj (Vec <Term>, Vec <Query>),
}

impl Prop {
    fn duplicate (
        &self,
        var_map: &mut HashMap <VarTerm, VarTerm>,
    ) -> Prop {
        match self {
            Prop::Disj (prop_vec) => {
                let mut new_prop_vec = Vec::new ();
                for prop in prop_vec {
                    new_prop_vec.push (
                        prop.duplicate (var_map));
                }
                Prop::Disj (new_prop_vec)
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
#[derive (PartialEq)]
pub struct Wissen {
    prop_dic: Dic <Prop>,
}

impl Wissen {
    fn prove <'a> (
        &'a self,
        query: Query,
    ) -> Proving <'a> {
        let trace = Trace {
            subst: Subst::new (),
            query_vec: vec! [Arc::new (query)],
        };
        let mut queue = VecDeque::new ();
        queue.push_back (trace);
        Proving {
            wissen: self,
            queue,
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Proving <'a> {
    wissen: &'a Wissen,
    queue: VecDeque <Trace>,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Trace {
    subst: Subst,
    query_vec: Vec <Arc <Query>>,
}

impl <'a> Proving <'a> {
    fn next_subst (&mut self) -> Option <Subst> {
        if let Some (frame) = self.queue.pop_front () {
            unimplemented!()
        } else {
            None
        }
    }
}

#[test]
fn test_unify () {
    let u = Term::var ("u");
    let v = Term::var ("v");
    let subst = Subst::new () .unify (
        &Term::tuple ("tuple", vec! [
            u.clone (),
            v.clone (),
        ]),
        &Term::tuple ("tuple", vec! [
            v.clone (),
            Term::tuple ("hi", vec! []),
        ]));
    println! ("{:#?}", subst.unwrap ());
}

#[test]
fn test_mexp () {

}
