#![feature (uniform_paths)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

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
pub struct Substitution {
    map: HashMap <Uuid, Term>,
}

impl Substitution {
    fn new () -> Self {
        Substitution {
            map: HashMap::new (),
        }
    }
}

impl Substitution {
    fn extend (&self, var: &Uuid, term: Term) -> Substitution {
        let mut map = self.map.clone ();
        map.insert (var.clone (), term);
        Substitution { map }
    }
}

impl Substitution {
    fn find (&self, var: &Uuid) -> Option <&Term> {
        self.map.get (var)
    }
}

impl Substitution {
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

impl Substitution {
    fn unify (
        &self,
        u: &Term,
        v: &Term,
    ) -> Option <Substitution> {
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
pub struct State {
    subst: Substitution,
}

impl State {
    fn new () -> Self {
        State {
            subst: Substitution::new (),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Stream {
    Null,
    More {
        state: State,
        next: Box <Stream>,
    },
}

impl Stream {
    fn mzero () -> Stream {
        Stream::Null
    }
}

impl Stream {
    fn unit (state: State) -> Stream {
        let next = Stream::mzero ();
        Stream::More {
            state,
            next: Box::new (next),
        }
    }
}

fn eqo (
    u: Term,
    v: Term,
) -> impl Fn (State) -> Stream {
    move |state| {
        if let Some (subst) = state.subst.unify (&u, &v) {
            let new_state = State {
                subst,
            };
            Stream::unit (new_state)
        } else {
            Stream::mzero ()
        }
    }
}

fn disj (
    g1: fn (State) -> Stream,
    g2: fn (State) -> Stream,
) -> impl Fn (State) -> Stream {
    move |state| {
        mplus (g1 (state.clone ()), g2 (state))
    }
}

fn conj (
    g1: fn (State) -> Stream,
    g2: fn (State) -> Stream,
) -> impl Fn (State) -> Stream {
    move |state| {
        let s1 = g1 (state);
        bind (s1, g2)
    }
}

fn mplus (s1: Stream, s2: Stream) -> Stream {
    if let Stream::More {
        state,
        next,
    } = s1 {
        let next = mplus (s2, *next);
        Stream::More {
            state,
            next: Box::new (next),
        }
    } else {
        s2
    }
}

fn bind (
    s: Stream,
    g: fn (State) -> Stream,
) -> Stream {
    if let Stream::More {
        state,
        next,
    } = s {
        mplus (g (state), bind (*next, g))
    } else {
        Stream::mzero ()
    }
}

#[test]
fn test_unify () {
    let subst = Substitution::new ();
    let v = Term::var ();
    let u = Term::var ();
    subst.unify (&v, &u) .unwrap ();
    subst.unify (&u, &u) .unwrap ();
    subst.unify (&v, &v) .unwrap ();
    let bye = Term::str ("bye");
    let world = Term::str ("world");
    let vec1 = Term::Vec (vec! [
        v.clone (),
        bye.clone (),
        u.clone (),
    ]);
    let vec2 = Term::Vec (vec! [
        u.clone (),
        bye.clone (),
        world.clone (),
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
    let world = Term::str ("world");
    let a = eqo (x.clone (), hi.clone ());
    // let b1 = eqo (y.clone (), bye.clone ());
    // let b2 = eqo (y.clone (), world.clone ());
    //// closure in rust is hopeless
    // let b = disj (b1, b2);
    // let g = conj (a, b);
    println! ("{:?}", a (State::new ()));
}
