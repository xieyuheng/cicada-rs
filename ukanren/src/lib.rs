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
pub struct World {
    map: HashMap <Uuid, Term>,
}

impl World {
    fn new () -> Self {
        World {
            map: HashMap::new (),
        }
    }
}

impl World {
    fn extend (&self, var: &Uuid, term: Term) -> World {
        let mut map = self.map.clone ();
        map.insert (var.clone (), term);
        World { map }
    }
}

impl World {
    fn find (&self, var: &Uuid) -> Option <&Term> {
        self.map.get (var)
    }
}

impl World {
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

impl World {
    fn unify (
        &self,
        u: &Term,
        v: &Term,
    ) -> Option <World> {
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
                let mut world = self.clone ();
                for (u, v) in us.iter () .zip (vs.iter ()) {
                    world = world.unify (u, v)?;
                }
                Some (world)
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
        world: World,
        next: Box <Stream>,
    },
}

impl Stream {
    fn mzero () -> Stream {
        Stream::Null
    }
}

impl Stream {
    fn unit (world: World) -> Stream {
        let next = Stream::mzero ();
        Stream::More {
            world,
            next: Box::new (next),
        }
    }
}

fn eqo (
    u: Term,
    v: Term,
) -> impl Fn (World) -> Stream {
    move |init_world| {
        if let Some (world) = init_world.unify (&u, &v) {
            Stream::unit (world)
        } else {
            Stream::mzero ()
        }
    }
}

fn disj (
    g1: fn (World) -> Stream,
    g2: fn (World) -> Stream,
) -> impl Fn (World) -> Stream {
    move |world| {
        mplus (g1 (world.clone ()), g2 (world))
    }
}

fn conj (
    g1: fn (World) -> Stream,
    g2: fn (World) -> Stream,
) -> impl Fn (World) -> Stream {
    move |world| {
        let s1 = g1 (world);
        bind (s1, g2)
    }
}

fn mplus (s1: Stream, s2: Stream) -> Stream {
    if let Stream::More {
        world,
        next,
    } = s1 {
        let next = mplus (s2, *next);
        Stream::More {
            world,
            next: Box::new (next),
        }
    } else {
        s2
    }
}

fn bind (
    s: Stream,
    g: fn (World) -> Stream,
) -> Stream {
    if let Stream::More {
        world,
        next,
    } = s {
        mplus (g (world), bind (*next, g))
    } else {
        Stream::mzero ()
    }
}

#[test]
fn test_unify () {
    let world = World::new ();
    let v = Term::var ();
    let u = Term::var ();
    world.unify (&v, &u) .unwrap ();
    world.unify (&u, &u) .unwrap ();
    world.unify (&v, &v) .unwrap ();
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
    let world = world.unify (&vec1, &vec2) .unwrap ();
    assert_eq! (2, world.map.len ());
}

#[test]
fn test_goal () {
    let x = Term::var ();
    let hi = Term::str ("hi");
    let a = eqo (x.clone (), hi.clone ());
    // let y = Term::var ();
    // let bye = Term::str ("bye");
    // let love = Term::str ("love");
    // let b1 = eqo (y.clone (), bye.clone ());
    // let b2 = eqo (y.clone (), love.clone ());
    //// closure in rust is hopeless
    // let b = disj (b1, b2);
    // let g = conj (a, b);
    println! ("{:?}", a (World::new ()));
}
