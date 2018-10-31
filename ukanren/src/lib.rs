#![feature (uniform_paths)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Term {
    Var (Uuid),
    Str (String),
    Vec (Vec <Term>),
}

impl Term {
    fn var () -> Term {
        Term::Var (Uuid::new_v4 ())
    }

    fn str (s: &str) -> Term {
        Term::Str (s.to_string ())
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Subst {
    map: HashMap <Uuid, Term>,
}

impl Subst {
    fn new () -> Self {
        Subst {
            map: HashMap::new (),
        }
    }
}

impl Subst {
    fn extend (&self, var: &Uuid, term: Term) -> Subst {
        let mut map = self.map.clone ();
        map.insert (var.clone (), term);
        Subst { map }
    }
}

impl Subst {
    fn find (&self, var: &Uuid) -> Option <&Term> {
        self.map.get (var)
    }
}

impl Subst {
    fn walk (&self, term: &Term) -> Term {
        match term {
            Term::Var (var) => {
                if let Some (new_term) = self.find (&var) {
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
    fn unify (
        &self,
        u: &Term,
        v: &Term,
    ) -> Option <Subst> {
        let u = self.walk (u);
        let v = self.walk (v);
        match (u, v) {
            (Term::Var (u), Term::Var (v)) if u == v => {
                Some (self.clone ())
            }
            (Term::Var (u), v) => {
                Some (self.extend (&u, v))
            }
            (u, Term::Var (v)) => {
                Some (self.extend (&v, u))
            }
            (Term::Vec (us), Term::Vec (vs)) => {
                if us.len () != vs.len () {
                    return None;
                }
                let mut subst = self.clone ();
                for (u, v) in us.iter () .zip (vs.iter ()) {
                    subst = subst.unify (u, v)?;
                }
                Some (subst)
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

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Stream {
    Null,
    More {
        subst: Subst,
        next: Box <Stream>,
    },
}

impl Stream {
    fn mzero () -> Stream {
        Stream::Null
    }
}

impl Stream {
    fn unit (subst: Subst) -> Stream {
        let next = Stream::mzero ();
        Stream::More {
            subst,
            next: Box::new (next),
        }
    }
}

fn eqo (
    u: Term,
    v: Term,
) -> Arc <Fn (Subst) -> Stream> {
    Arc::new (move |init_subst| {
        if let Some (subst) = init_subst.unify (&u, &v) {
            Stream::unit (subst)
        } else {
            Stream::mzero ()
        }
    })
}

fn disj (
    g1: Arc <Fn (Subst) -> Stream>,
    g2: Arc <Fn (Subst) -> Stream>,
) -> Arc <Fn (Subst) -> Stream> {
    Arc::new (move |subst| {
        mplus (g1 (subst.clone ()), g2 (subst))
    })
}

fn conj (
    g1: Arc <Fn (Subst) -> Stream>,
    g2: Arc <Fn (Subst) -> Stream>,
) -> Arc <Fn (Subst) -> Stream> {
    Arc::new (move |subst| {
        let s1 = g1 (subst);
        bind (s1, g2.clone ())
    })
}

fn mplus (s1: Stream, s2: Stream) -> Stream {
    if let Stream::More {
        subst,
        next,
    } = s1 {
        let next = mplus (s2, *next);
        Stream::More {
            subst,
            next: Box::new (next),
        }
    } else {
        s2
    }
}

fn bind (
    s: Stream,
    g: Arc <Fn (Subst) -> Stream>,
) -> Stream {
    if let Stream::More {
        subst,
        next,
    } = s {
        mplus (g (subst), bind (*next, g))
    } else {
        Stream::mzero ()
    }
}

#[test]
fn test_unify () {
    let subst = Subst::new ();
    let v = Term::var ();
    let u = Term::var ();
    subst.unify (&v, &u) .unwrap ();
    subst.unify (&u, &u) .unwrap ();
    subst.unify (&v, &v) .unwrap ();
    let bye = Term::str ("bye");
    let love = Term::str ("love");
    let vec1 = Term::Vec (vec! [
        v.clone (),
        bye.clone (),
        u.clone (),
    ]);
    let vec2 = Term::Vec (vec! [
        u.clone (),
        bye.clone (),
        love.clone (),
    ]);
    let subst = subst.unify (&vec1, &vec2) .unwrap ();
    assert_eq! (2, subst.map.len ());
}

#[test]
fn test_goal () {
    let x = Term::var ();
    let y = Term::var ();
    let hi = Term::str ("hi");
    let bye = Term::str ("bye");
    let love = Term::str ("love");
    let g = conj (eqo (x.clone (), hi.clone ()),
                  disj (eqo (y.clone (), bye.clone ()),
                        eqo (y.clone (), love.clone ())));
    println! ("{:?}", g (Subst::new ()));
}
