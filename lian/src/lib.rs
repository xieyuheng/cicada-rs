#![feature (uniform_paths)]
#![feature (bind_by_move_pattern_guards)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]
#![allow (non_camel_case_types)]

use std::vec;
use std::sync::Arc;
use uuid::Uuid;

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Term {
    Var   (VarTerm),
    Tuple (TupleTerm),
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct VarTerm {
    name: String,
    id: Uuid,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct TupleTerm {
    head: String,
    body: Vec <Term>,
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
                Some (Subst::Cons {
                    var: u,
                    term: v,
                    next: Arc::new (self.clone ()),
                })
            }
            (u, Term::Var (v)) => {
                Some (Subst::Cons {
                    var: v,
                    term: u,
                    next: Arc::new (self.clone ()),
                })
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
#[derive (PartialEq)]
pub enum Goal {
    Eqo  {
        u: Term,
        v: Term,
    },
    Disj {
        g1: Arc <Goal>,
        g2: Arc <Goal>,
    },
    Conj {
        g1: Arc <Goal>,
        g2: Arc <Goal>,
    },
}

impl Goal {
    pub fn apply (&self, subst: Subst) -> Stream {
        match self {
            Goal::Eqo { u, v } => {
                if let Some (
                    new_subst
                ) = subst.unify (&u, &v) {
                    unit (new_subst)
                } else {
                    mzero ()
                }
            }
            Goal::Disj { g1, g2 } => {
                mplus (
                    g1.apply (subst.clone ()),
                    g2.apply (subst))
            }
            Goal::Conj { g1, g2 } => {
                bind (g1.apply (subst), g2)
            }
        }
    }
}

type Stream = Box <Iterator <Item = Subst>>;

fn mzero () -> Stream {
    Box::new (Vec::new () .into_iter ())
}

fn unit (subst: Subst) -> Stream {
    Box::new (vec! [subst] .into_iter ())
}

fn mplus (mut s1: Stream, s2: Stream) -> Stream {
    if let Some (subst) = s1.next () {
        Box::new (unit (subst) .chain (s2) .chain (s1))
    } else {
        s2
    }
}

fn bind (mut s: Stream, g: &Goal) -> Stream {
    if let Some (subst) = s.next () {
        mplus (g.apply (subst), bind (s, g))
    } else {
        mzero ()
    }
}

fn var (s: &str) -> Term {
    Term::Var (VarTerm {
        name: s.to_string (),
        id: Uuid::new_v4 (),
    })
}

fn tuple (h: &str, vec: Vec <Term>) -> Term {
    Term::Tuple (TupleTerm {
        head: h.to_string (),
        body: vec,
    })
}

fn eqo (u: Term, v: Term) -> Arc <Goal> {
    Arc::new (Goal::Eqo { u, v })
}

fn disj (g1: Arc <Goal>, g2: Arc <Goal>) -> Arc <Goal> {
    Arc::new (Goal::Disj { g1, g2 })
}

fn conj (g1: Arc <Goal>, g2: Arc <Goal>) -> Arc <Goal> {
    Arc::new (Goal::Conj { g1, g2 })
}

#[test]
fn test_unify () {
    let u = var ("u");
    let v = var ("v");
    let subst = Subst::new () .unify (
        &tuple ("tuple", vec! [u.clone (), v.clone ()]),
        &tuple ("tuple", vec! [v.clone (), tuple ("hi", vec! [])]));
    println! ("{:?}", subst.unwrap ());
}

#[test]
fn test_goal () {
    let g = conj (
        eqo (var ("u"), tuple ("love", vec! [])),
        disj (eqo (var ("v"), tuple ("bye", vec! [])),
              eqo (var ("w"), tuple ("hi", vec! [])))
    );
    for subst in g.apply (Subst::new ()) {
        println! ("- {:?}", subst);
    }
}

#[test]
fn test_mexp () {

}
