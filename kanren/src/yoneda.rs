use std::vec;
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
    fn ext (&self, var: VarTerm, term: Term) -> Self {
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
                Some (self.ext (u, v))
            }
            (u, Term::Var (v)) => {
                Some (self.ext (v, u))
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
pub enum Relation {
    Disj (Vec <ConjRelation>),
    Conj (ConjRelation),
}

impl Relation {
    fn apply (&self, term_vec: &Vec <Term>) -> Arc <Goal> {
        match self {
            Relation::Disj (body) => {
                let mut goal = Goal::zero ();
                for conj_relation in body {
                    goal = Goal::disj (
                        goal,
                        conj_relation.apply (term_vec))
                }
                goal
            }
            Relation::Conj (conj_relation) => {
                conj_relation.apply (term_vec)
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct ConjRelation {
    head: Vec <Term>,
    body: Vec <TupleTerm>,
}

impl ConjRelation {
    fn apply (&self, term_vec: &Vec <Term>) -> Arc <Goal> {
        let mut goal = Goal::unit ();
        let zip = self.head.iter () .zip (term_vec.iter ());
        for (u, v) in zip {
            let g = Goal::eqo (u.clone (), v.clone ());
            goal = Goal::conj (goal, g)
        }
        for term in &self.body {
            // println! ("- term = {:?}", term);
            // println! ("- self = {:?}", self);
            let g = Arc::new (Goal::Call (term.clone ()));
            goal = Goal::conj (goal, g)
        }
        goal
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Goal {
    Zero,
    Unit,
    Eqo (Term, Term),
    Disj (Arc <Goal>, Arc <Goal>),
    Conj (Arc <Goal>, Arc <Goal>),
    Call (TupleTerm),
}

impl Goal {
    pub fn apply (
        &self,
        relation_dic: &Dic <Relation>,
        subst: Subst,
    ) -> Stream {
        match self {
            Goal::Zero => {
                mzero ()
            }
            Goal::Unit => {
                unit (subst)
            }
            Goal::Eqo (u, v) => {
                if let Some (
                    new_subst
                ) = subst.unify (&u, &v) {
                    unit (new_subst)
                } else {
                    mzero ()
                }
            }
            Goal::Disj (g1, g2) => {
                mplus (
                    g1.apply (relation_dic, subst.clone ()),
                    g2.apply (relation_dic, subst))
            }
            Goal::Conj (g1, g2) => {
                bind (
                    g1.apply (relation_dic, subst),
                    g2,
                    relation_dic)
            }
            Goal::Call (TupleTerm {
                head,
                body,
            }) => {
                if let Some (
                    relation
                ) = relation_dic.get (&head) {
                    let goal = relation.apply (body);
                    goal.apply (relation_dic, subst)
                } else {
                    mzero ()
                }
            }
        }
    }
}

impl Goal {
    fn zero () -> Arc <Goal> {
        Arc::new (Goal::Zero)
    }
}

impl Goal {
    fn unit () -> Arc <Goal> {
        Arc::new (Goal::Unit)
    }
}

impl Goal {
    fn eqo (u: Term, v: Term) -> Arc <Goal> {
        Arc::new (Goal::Eqo (u, v))
    }
}

impl Goal {
    fn disj (g1: Arc <Goal>, g2: Arc <Goal>) -> Arc <Goal> {
        Arc::new (Goal::Disj (g1, g2))
    }
}

impl Goal {
    fn conj (g1: Arc <Goal>, g2: Arc <Goal>) -> Arc <Goal> {
        Arc::new (Goal::Conj (g1, g2))
    }
}

impl Goal {
    fn call (name: &str, body: Vec <Term>) -> Arc <Goal> {
        Arc::new (Goal::Call (TupleTerm {
            head: name.to_string (),
            body,
        }))
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

fn bind (
    mut stream: Stream,
    goal: &Goal,
    relation_dic: &Dic <Relation>,
) -> Stream {
    if let Some (subst) = stream.next () {
        mplus (goal.apply (relation_dic, subst),
               bind (stream, goal, relation_dic))
    } else {
        mzero ()
    }
}

fn query (
    relation_dic: &Dic <Relation>,
    relation: &Relation,
    term_vec: &Vec <Term>,
) -> Stream {
    let goal = relation.apply (term_vec);
    goal.apply (relation_dic, Subst::new ())
}

#[test]
fn test_unify () {
    let u = Term::var ("u");
    let v = Term::var ("v");
    let _subst = Subst::new () .unify (
        &Term::tuple ("tuple", vec! [
            u.clone (),
            v.clone (),
        ]),
        &Term::tuple ("tuple", vec! [
            v.clone (),
            Term::tuple ("hi", vec! []),
        ]));
    // println! ("{:?}", subst.unwrap ());
}

#[test]
fn test_goal () {
    let goal = Goal::conj (
        Goal::eqo (
            Term::var ("u"),
            Term::tuple ("love", vec! [])),
        Goal::disj (
            Goal::eqo (
                Term::var ("v"),
                Term::tuple ("bye", vec! [])),
            Goal::eqo (
                Term::var ("w"),
                Term::tuple ("hi", vec! []))));
    let relation_dic = Dic::new ();
    for _subst in goal.apply (&relation_dic, Subst::new ()) {
        // println! ("- {:?}", subst);
    }
}

#[test]
fn test_query () {
    // fives-t:five (five-c)
    // fives-t:more (x) { fives-t (x) }
    let x = Term::var ("x");
    let relation = Relation::Disj (
        vec! [
            ConjRelation {
                head: vec! [Term::tuple ("five-c", vec! [])],
                body: vec! [],
            },
            ConjRelation {
                head: vec! [x.clone ()],
                body: vec! [
                    TupleTerm {
                        head: "fives-t".to_string (),
                        body: vec! [x.clone ()],
                    },
                ],
            },
        ]);
    let _relation = Relation::Conj (
        ConjRelation {
            head: vec! [Term::tuple ("five-c", vec! [])],
            body: vec! [],
        });
    let mut relation_dic = Dic::new ();
    relation_dic.ins ("fives-t", Some (relation.clone ()));
    let term_vec = vec! [Term::var ("u")];
    // let stream = query (
    //     &relation_dic,
    //     &relation,
    //     &term_vec);
    // for subst in stream {
    //     println! ("- {:?}", subst);
    // }

    let _goal = relation.apply (&term_vec);
    // println! ("- goal = {:?}", goal);
    // goal.apply (&relation_dic, Subst::new ());
}

#[test]
fn test_mexp () {

}
