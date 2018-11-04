#![feature (uniform_paths)]
#![feature (bind_by_move_pattern_guards)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]
#![allow (non_camel_case_types)]

use std::sync::Arc;
use uuid::Uuid;
use dic::Dic;

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

impl Term {
    fn var (s: &str) -> Term {
        Term::Var (VarTerm {
            name: s.to_string (),
            id: Uuid::new_v4 (),
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
#[derive (PartialEq)]
pub struct Disj {
    conj_dic: Dic <Conj>,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Conj {
    head: Vec <Term>,
    body: Vec <Call>,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Call {
    Disj {
        disj_name: String,
        args: Vec <Term>,
    },
    Conj {
        disj_name: String,
        conj_name: String,
        args: Vec <Term>,
    },
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Wissen {
    disj_dic: Dic <Disj>,
}

impl Wissen {
    fn query <'a> (
        &'a self,
        disj_name: &str,
        args: Vec <Term>,
    ) -> Solving <'a> {
        let disj = self.disj_dic.get (disj_name) .unwrap ();
        let frame = Frame {
            disj_name: disj_name.to_string (),
            disj: disj.clone (),
            args: args,
            index: 0,
            backup_subst: Subst::new (),
        };
        Solving {
            wissen: self,
            trace: vec! [frame],
            subst: Subst::new (),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Solving <'a> {
    wissen: &'a Wissen,
    trace: Vec <Frame>,
    subst: Subst,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Frame {
    disj_name: String,
    disj: Disj,
    args: Vec <Term>,
    index: usize,
    backup_subst: Subst,
}

impl <'a> Solving <'a> {
    fn next_subst (&mut self) -> Option <Subst> {
        unimplemented! ()
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
    println! ("{:?}", subst.unwrap ());
}

#[test]
fn test_mexp () {

}
