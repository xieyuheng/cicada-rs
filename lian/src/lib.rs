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
    pub fn apply (&self, subst: Subst) -> Searching {
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

type Searching = Box <Iterator <Item = Subst>>;

fn mzero () -> Searching {
    Box::new (Vec::new () .into_iter ())
}

fn unit (subst: Subst) -> Searching {
    Box::new (vec! [subst] .into_iter ())
}

fn mplus (mut s1: Searching, s2: Searching) -> Searching {
    if let Some (subst) = s1.next () {
        Box::new (unit (subst) .chain (s2) .chain (s1))
    } else {
        s2
    }
}

fn bind (mut s: Searching, g: &Goal) -> Searching {
    if let Some (subst) = s.next () {
        mplus (g.apply (subst), bind (s, g))
    } else {
        mzero ()
    }
}

#[test]
fn test_unify () {
    let subst = Subst::Null;
    let u = Term::Var (VarTerm {
        name: "u".to_string (),
        id: Uuid::new_v4 (),
    });
    let v = Term::Var (VarTerm {
        name: "v".to_string (),
        id: Uuid::new_v4 (),
    });
    let subst = subst.unify (&u, &v);
    println! ("{:?}", subst);
}

#[test]
fn test_goal () {
    let subst = Subst::Null;
    let g = Goal::Conj {
        g1: Arc::new (Goal::Eqo {
            u: Term::Var (VarTerm {
                name: "u".to_string (),
                id: Uuid::new_v4 (),
            }),
            v: Term::Tuple (TupleTerm {
                head: "love".to_string (),
                body: Vec::new (),
            }),
        }),
        g2: Arc::new (Goal::Disj {
            g1: Arc::new (Goal::Eqo {
                u: Term::Var (VarTerm {
                    name: "v".to_string (),
                    id: Uuid::new_v4 (),
                }),
                v: Term::Tuple (TupleTerm {
                    head: "bye".to_string (),
                    body: Vec::new (),
                }),
            }),
            g2: Arc::new (Goal::Eqo {
                u: Term::Var (VarTerm {
                    name: "w".to_string (),
                    id: Uuid::new_v4 (),
                }),
                v: Term::Tuple (TupleTerm {
                    head: "hi".to_string (),
                    body: Vec::new (),
                }),
            }),
        })
    };
    for subst in g.apply (subst) {
        println! ("{:?}", subst);
    }
}
