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
pub struct Env {
    pub relation_dic: RelationDic,
    pub subst: Subst,
}

impl Env {
    fn new () -> Self {
        Env {
            relation_dic: RelationDic::Null,
            subst: Subst::new (),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum RelationDic {
    Null,
    Cons {
        relation_name: String,
        relation: Relation,
        next: Arc <RelationDic>,
    },
}

impl RelationDic {
    fn new () -> Self {
        RelationDic::Null
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Relation {

}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Choice {
    body: Vec <Goal>,
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
    fn cons (&self, var: VarTerm, term: Term) -> Self {
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
                Some (self.cons (u, v))
            }
            (u, Term::Var (v)) => {
                Some (self.cons (v, u))
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
    pub fn apply (&self, env: Env) -> Stream {
        match self {
            Goal::Eqo { u, v } => {
                if let Some (
                    subst
                ) = env.subst.unify (&u, &v) {
                    unit (Env {
                        subst,
                        ..env
                    })
                } else {
                    mzero ()
                }
            }
            Goal::Disj { g1, g2 } => {
                mplus (
                    g1.apply (env.clone ()),
                    g2.apply (env))
            }
            Goal::Conj { g1, g2 } => {
                bind (g1.apply (env), g2)
            }
        }
    }
}

type Stream = Box <Iterator <Item = Env>>;

fn mzero () -> Stream {
    Box::new (Vec::new () .into_iter ())
}

fn unit (env: Env) -> Stream {
    Box::new (vec! [env] .into_iter ())
}

fn mplus (mut s1: Stream, s2: Stream) -> Stream {
    if let Some (env) = s1.next () {
        Box::new (unit (env) .chain (s2) .chain (s1))
    } else {
        s2
    }
}

fn bind (mut s: Stream, g: &Goal) -> Stream {
    if let Some (env) = s.next () {
        mplus (g.apply (env), bind (s, g))
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
    let env = Env::new () .subst.unify (
        &tuple ("tuple", vec! [u.clone (), v.clone ()]),
        &tuple ("tuple", vec! [v.clone (), tuple ("hi", vec! [])]));
    println! ("{:?}", env.unwrap ());
}

#[test]
fn test_goal () {
    let g = conj (
        eqo (var ("u"), tuple ("love", vec! [])),
        disj (eqo (var ("v"), tuple ("bye", vec! [])),
              eqo (var ("w"), tuple ("hi", vec! [])))
    );
    for env in g.apply (Env::new ()) {
        println! ("- {:?}", env);
    }
}

#[test]
fn test_mexp () {

}
