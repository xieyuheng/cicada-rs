#![feature (uniform_paths)]
#![feature (bind_by_move_pattern_guards)]
#![feature (box_patterns)]
#![feature (box_syntax)]

#![allow (dead_code)]

use std::fmt;
use std::sync::Arc;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;
use dic::Dic;
use error_report::{
    // Span,
    ErrorMsg,
    ErrorInCtx,
};
#[cfg (test)]
use error_report::{
    ErrorCtx,
};
use mexp::{
    SyntaxTable,
    Mexp,
    MexpArg,
};

fn vec_to_string <T> (vec: &Vec <T>, delimiter: &str) -> String
where T : ToString {
    let mut s = String::new ();
    for x in vec {
        s += &x.to_string ();
        s += delimiter;
    }
    for _ in 0 .. delimiter.len () {
        s.pop ();
    }
    s
}

fn vec_to_lines <T> (vec: &Vec <T>) -> String
where T : ToString {
    let mut s = vec_to_string (vec, "\n");
    if ! s.is_empty () {
        s += "\n";
    }
    s
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Term {
    Var (Var),
    Cons (Cons),
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Cons {
    head: String,
    body: Vec <Term>,
}

impl ToString for Cons {
    fn to_string (&self) -> String {
        if self.body.len () == 0 {
            format! ("{}", self.head)
        } else {
            format! (
                "{} ({})",
                self.head,
                vec_to_string (&self.body, " "))
        }
    }
}

impl Term {
    fn var (s: &str) -> Term {
        Term::Var (Var {
            name: s.to_string (),
            id: Some (Id::uuid ()),
        })
    }
}

impl Term {
    fn var_no_id (s: &str) -> Term {
        Term::Var (Var {
            name: s.to_string (),
            id: None,
        })
    }
}

impl Term {
    fn var_local (s: &str, counter: usize) -> Term {
        Term::Var (Var {
            name: s.to_string (),
            id: Some (Id::local (counter)),
        })
    }
}

impl Term {
    fn cons (h: &str, vec: Vec <Term>) -> Term {
        Term::Cons (Cons {
            head: h.to_string (),
            body: vec,
        })
    }
}

impl Term {
    fn duplicate (
        &self,
        var_map: &mut HashMap <Var, Var>,
    ) -> Term {
        match self {
            Term::Var (var) => {
                if let Some (dup_var) = var_map.get (var) {
                    Term::Var (dup_var.clone ())
                } else {
                    let new_var = Var {
                        name: var.name.clone (),
                        id: Some (Id::uuid ()),
                    };
                    var_map.insert (
                        var.clone (),
                        new_var.clone ());
                    Term::Var (new_var)
                }
            }
            Term::Cons (cons) => {
                let mut new_cons = Cons {
                    head: cons.head.clone (),
                    body: Vec::new (),
                };
                for term in &cons.body {
                    new_cons.body.push (
                        term.duplicate (var_map));
                }
                Term::Cons (new_cons)
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

impl ToString for Term {
    fn to_string (&self) -> String {
        match self {
            Term::Var (var) => var.to_string (),
            Term::Cons (cons) => cons.to_string (),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub struct Var {
    name: String,
    id: Option <Id>,
}

impl ToString for Var {
    fn to_string (&self) -> String {
        let mut s = format! ("{}", self.name);
        if let Some (id) = &self.id {
            s += &format! ("#{}", id.to_string ());
        }
        s
    }
}

#[derive (Clone)]
#[derive (PartialEq, Eq, Hash)]
pub enum Id {
    Uuid (uuid::adapter::Hyphenated),
    Local (usize),
}

impl Id {
    fn uuid () -> Self {
        Id::Uuid (Uuid::new_v4 () .to_hyphenated ())
    }
}

impl Id {
    fn local (counter: usize) -> Self {
        Id::Local (counter)
    }
}

impl fmt::Debug for Id {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Id::Uuid (uuid) => write! (f, "{}", uuid),
            Id::Local (counter) => write! (f, "{}", counter),
        }
    }
}

impl ToString for Id {
    fn to_string (&self) -> String {
        match self {
            Id::Uuid (uuid) => format! ("{}", uuid),
            Id::Local (counter) => format! ("{}", counter),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Subst {
    Null,
    Cons {
       var: Var,
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
    fn extend (&self, var: Var, term: Term) -> Self {
        Subst::Cons {
            var, term,
            next: Arc::new (self.clone ()),
        }
    }
}

impl Subst {
    pub fn find (&self, var: &Var) -> Option <&Term> {
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
                if self.var_occur_p (&u, &v) {
                    None
                } else {
                    Some (self.extend (u, v))
                }
            }
            (u, Term::Var (v)) => {
                if self.var_occur_p (&v, &u) {
                    None
                } else {
                    Some (self.extend (v, u))
                }
            }
            (Term::Cons (ut),
             Term::Cons (vt),
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

impl Subst {
    pub fn null_p (&self) -> bool {
        &Subst::Null == self
    }
}

impl Subst {
    pub fn cons_p (&self) -> bool {
        &Subst::Null != self
    }
}

impl Subst {
    pub fn len (&self) -> usize {
        let mut len = 0;
        let mut subst = self;
        while let Subst::Cons { next, .. } = subst {
            len += 1;
            subst = &next;
        }
        len
    }
}

impl ToString for Subst {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        let mut subst = self.clone ();
        while let Subst::Cons {
            var,
            term,
            next,
        } = subst {
            s += &var.to_string ();
            s += " = ";
            s += &term.to_string ();
            s += "\n";
            subst = (*next) .clone ();
        }
        s
    }
}

impl Subst {
    pub fn deep_walk (&self, term: &Term) -> Term {
        let term = self.walk (term);
        match term {
            Term::Var (_) => term,
            Term::Cons (Cons {
                head,
                body,
            }) => {
                let body = body.iter ()
                    .map (|x| self.deep_walk (x))
                    .collect ();
                Term::Cons (Cons {
                    head,
                    body,
                })
            }
        }
    }
}

impl Subst {
    pub fn localize_by_term (&self, term: &Term) -> Subst {
        let term = self.walk (term);
        match term {
            Term::Var (var) => {
                self.extend (
                    var.clone (),
                    Term::var_local (
                        &var.name,
                        self.len ()))
            }
            Term::Cons (Cons {
                body, ..
            }) => {
                let mut subst = self.clone ();
                for term in &body {
                    subst = subst.localize_by_term (term);
                }
                subst
            }
        }
    }
}

impl Subst {
    pub fn reify_var (&self, var: &Var) -> Term {
        let term = Term::Var (var.clone ());
        let term = self.deep_walk (&term);
        let new_subst = Subst::new ();
        let local_subst = new_subst.localize_by_term (&term);
        local_subst.deep_walk (&term)
    }
}

impl Subst {
    pub fn var_occur_p (
        &self,
        var: &Var,
        term: &Term,
    ) -> bool {
        let term = self.walk (term);
        match term {
            Term::Var (var1) => {
                var == &var1
            }
            Term::Cons (cons) => {
                for term in &cons.body {
                    if self.var_occur_p (var, term) {
                        return true;
                    }
                }
                return false;
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Den {
    Disj (Vec <String>),
    Conj (Vec <Term>, Vec <Prop>),
}

impl Den {
    fn duplicate (
        &self,
        var_map: &mut HashMap <Var, Var>,
    ) -> Den {
        match self {
            Den::Disj (_name_vec) => {
                self.clone ()
            }
            Den::Conj (args, prop_vec) => {
                let mut new_args = Vec::new ();
                for arg in args {
                    new_args.push (
                        arg.duplicate (var_map))
                }
                let mut new_prop_vec = Vec::new ();
                for prop in prop_vec {
                    new_prop_vec.push (
                        prop.duplicate (var_map))
                }
                Den::Conj (new_args, new_prop_vec)
            }
        }
    }
}

impl Den {
    fn fresh (&self) -> Den {
        let mut var_map = HashMap::new ();
        self.duplicate (&mut var_map)
    }
}

impl Den {
    fn apply (
        self,
        args: &Vec <Term>,
        mut subst: Subst,
    ) -> Option <(Vec <Vec <Arc <Prop>>>, Subst)> {
        match self {
            Den::Disj (name_vec) => {
                let mut prop_matrix = Vec::new ();
                for name in name_vec {
                    let prop = Arc::new (Prop {
                        name: name,
                        args: args.clone (),
                    });
                    prop_matrix.push (vec! [prop]);
                }
                Some ((prop_matrix, subst))
            }
            Den::Conj (terms, prop_vec) => {
                if args.len () != terms.len () {
                    eprintln! ("- [warning] Den::apply");
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
                let prop_matrix = vec! [
                    prop_vec
                        .into_iter ()
                        .map (|x| Arc::new (x))
                        .collect ()
                ];
                Some ((prop_matrix, subst))
            }
        }
    }
}

impl ToString for Den {
    fn to_string (&self) -> String {
        match self {
            Den::Disj (name_vec) => {
                format! (
                    "disj ({})",
                    vec_to_string (&name_vec, " "))
            }
            Den::Conj (terms, prop_vec) => {
                format! (
                    "conj ({}) {}",
                    vec_to_string (&terms, " "),
                    if prop_vec.len () == 0 {
                        format! ("{{}}")
                    } else {
                        format! (
                            "{{ {} }}",
                            vec_to_string (&prop_vec, " "))
                    })
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Prop {
    name: String,
    args: Vec <Term>,
}

impl Prop {
    fn duplicate (
        &self,
        var_map: &mut HashMap <Var, Var>,
    ) -> Prop {
        let mut new_args = Vec::new ();
        for arg in &self.args {
            new_args.push (
                arg.duplicate (var_map));
        }
        Prop {
            name: self.name.clone (),
            args: new_args,
        }
    }
}

impl ToString for Prop {
    fn to_string (&self) -> String {
        format! (
            "{} ({})",
            self.name,
            vec_to_string (&self.args, " "))
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Wissen {
    den_dic: Dic <Den>,
}

impl Wissen {
    pub fn new () -> Self {
        Wissen {
            den_dic: Dic::new (),
        }
    }
}

impl Wissen {
    fn find_den (&self, name: &str) -> Option <Den> {
        if let Some (
            den
        ) = self.den_dic.get (name) {
            Some (den.fresh ())
        } else {
            None
        }
    }
}

impl Wissen {
    pub fn proving <'a> (
        &'a self,
        prop_vec: &Vec <Prop>,
    ) -> Proving <'a> {
        let mut prop_queue = VecDeque::new ();
        for prop in prop_vec {
            prop_queue.push_back (Arc::new (prop.clone ()))
        }
        let proof = Proof {
            wissen: self,
            tree_stack: vec! [DeductionTree {
               conj_name: "root".to_string (),
               arity: prop_vec.len (),
               body: Vec::new (),
            }],
            subst: Subst::new (),
            prop_queue,
        };
        Proving {
            proof_queue: vec! [proof] .into (),
        }
    }
}

impl ToString for Wissen {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        for (name, den) in self.den_dic.iter () {
            s += name;
            s += " = ";
            s += &den.to_string ();
            s += "\n";
        }
        format! ("<wissen>\n{}</wissen>\n", s)
    }
}

impl Wissen {
    pub fn den (&mut self, name: &str, den: &Den) {
       self.den_dic.ins (name, Some (den.clone ()));
    }
}

impl Wissen {
    pub fn wis <'a> (
        &'a mut self,
        input: &str,
    ) -> Result <Vec <WissenOutput>, ErrorInCtx> {
        let syntax_table = SyntaxTable::default ();
        let mexp_vec = syntax_table.parse (input)?;
        let statement_vec = mexp_vec_to_statement_vec (&mexp_vec)?;
        for statement in &statement_vec {
            if let Statement::Den (
                name, den
            ) = statement {
                self.den (name, den);
            }
        }
        let mut output_vec = Vec::new ();
        for statement in &statement_vec {
            if let Statement::Query (
                counter, prop_vec
            ) = statement {
                let mut proving = self.proving (prop_vec);
                let subst_vec = proving.take_subst (*counter);
                output_vec.push (WissenOutput::Query {
                    prop_vec: prop_vec.clone (),
                    subst_vec,
                });
            }
            if let Statement::Prove (
                counter, prop_vec
            ) = statement {
                let mut proving = self.proving (prop_vec);
                let qed_vec = proving.take_qed (*counter);
                output_vec.push (WissenOutput::Prove {
                    prop_vec: prop_vec.clone (),
                    qed_vec,
                });
            }
        }
        Ok (output_vec)
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum WissenOutput {
    Query {
        prop_vec: Vec <Prop>,
        subst_vec: Vec <Subst>,
    },
    Prove {
        prop_vec: Vec <Prop>,
        qed_vec: Vec <Qed>,
    },
}

fn collect_var_from_prop_vec (
    prop_vec: &Vec <Prop>
) -> HashSet <Var> {
    let mut var_set = HashSet::new ();
    for prop in prop_vec {
        for var in collect_var_from_term_vec (&prop.args) {
            var_set.insert (var);
        }
    }
    var_set
}

fn collect_var_from_term (
    term: &Term
) -> HashSet <Var> {
    match term {
        Term::Var (var) => {
            let mut var_set = HashSet::new ();
            var_set.insert (var.clone ());
            var_set
        }
        Term::Cons (cons) => {
            collect_var_from_term_vec (&cons.body)
        }
    }
}

fn collect_var_from_term_vec (
    term_vec: &Vec <Term>
) -> HashSet <Var> {
    let mut var_set = HashSet::new ();
    for term in term_vec {
        for var in collect_var_from_term (term) {
            var_set.insert (var);
        }
    }
    var_set
}

impl ToString for WissenOutput {
    fn to_string (&self) -> String {
        match self {
            WissenOutput::Query {
                prop_vec,
                subst_vec,
            } => {
                let mut s = String::new ();
                s += "<query-output>\n";
                s += "\n";
                let var_set = collect_var_from_prop_vec (
                    prop_vec);
                for subst in subst_vec {
                    s += "<subst>\n";
                    for var in &var_set {
                        s += &var.to_string ();
                        s += " = ";
                        s += &subst.reify_var (var) .to_string ();
                        s += "\n";
                    }
                    s += "</subst>\n";
                    s += "\n";
                }
                s += "</query-output>";
                s
            }
            WissenOutput::Prove {
                prop_vec,
                qed_vec,
            } => {
                let mut s = String::new ();
                s += "<prove-output>\n";
                s += "\n";
                let var_set = collect_var_from_prop_vec (
                    prop_vec);
                for qed in qed_vec {
                    s += "<deduction-tree>\n";
                    s += &qed.deduction_tree.to_string ();
                    s += "\n";
                    s += "</deduction-tree>\n";
                    s += "<subst>\n";
                    for var in &var_set {
                        s += &var.to_string ();
                        s += " = ";
                        s += &qed.subst.reify_var (var) .to_string ();
                        s += "\n";
                    }
                    s += "</subst>\n";
                    s += "\n";
                }
                s += "</prove-output>";
                s
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Statement {
    Den (String, Den),
    Query (usize, Vec <Prop>),
    Prove (usize, Vec <Prop>),
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Proving <'a> {
    proof_queue: VecDeque <Proof <'a>>,
}

impl <'a> Proving <'a> {
    pub fn next_qed (&mut self) -> Option <Qed> {
        while let Some (
            mut proof
        ) = self.proof_queue.pop_front () {
            match proof.step () {
                ProofStep::One (qed) => {
                    return Some (qed);
                }
                ProofStep::More (proof_queue) => {
                    for proof in proof_queue {
                        //// about searching
                        // push back  |   depth first
                        // push front | breadth first
                        self.proof_queue.push_back (proof);
                    }
                }
                ProofStep::Fail => {}
            }
        }
        return None;
    }
}

impl <'a> Proving <'a> {
    pub fn take_subst (&mut self, n: usize) -> Vec <Subst> {
        let mut vec = Vec::new ();
        for _ in 0..n {
            if let Some (qed) = self.next_qed () {
                vec.push (qed.subst)
            }
        }
        vec
    }
}

impl <'a> Proving <'a> {
    pub fn take_qed (&mut self, n: usize) -> Vec <Qed> {
        let mut vec = Vec::new ();
        for _ in 0..n {
            if let Some (qed) = self.next_qed () {
                vec.push (qed)
            }
        }
        vec
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Proof <'a> {
    wissen: &'a Wissen,
    subst: Subst,
    tree_stack: Vec <DeductionTree>,
    prop_queue: VecDeque <Arc <Prop>>,
}

impl <'a> Proof <'a> {
    fn step (&mut self) -> ProofStep <'a> {
        if let Some (
            prop
        ) = self.prop_queue.pop_front () {
            if let Some (
                den
            ) = self.wissen.find_den (&prop.name) {
                let mut proof_queue = VecDeque::new ();
                let backup_den = den.clone ();
                if let Some (
                    (prop_matrix, new_subst)
                ) = den.apply (&prop.args, self.subst.clone ()) {
                    for prop_vec in prop_matrix {
                        let mut proof = self.clone ();
                        proof.subst = new_subst.clone ();
                        proof.record_deduction_step (
                            &prop,
                            &backup_den);
                        let rev = prop_vec.into_iter () .rev ();
                        for prop in rev {
                            // the order must be kept
                            //   to record_deduction_step
                            proof.prop_queue.push_front (prop);
                        }
                        proof_queue.push_back (proof);
                    }
                    ProofStep::More (proof_queue)
                } else {
                    ProofStep::Fail
                }
            } else {
                eprintln! ("- [warning] Proof::step");
                eprintln! ("  undefined den = {}", prop.name);
                ProofStep::Fail
            }
        } else {
            let deduction_tree = self.tree_stack.pop () .unwrap ();
            ProofStep::One (Qed {
                subst: self.subst.clone (),
                deduction_tree,
            })
        }
    }
}

impl <'a> ToString for Proof <'a> {
    fn to_string (&self) -> String {
        let prop_vec: &Vec <Prop> = &self.prop_queue
            .iter ()
            .map (|x| (**x) .clone ())
            .collect ();
        format! (
            "<proof>\n\
            <prop_queue>\n\
            {}</prop_queue>\n\
            <subst>\n\
            {}</subst>\n\
            </proof>\n",
            vec_to_lines (prop_vec),
            self.subst.to_string ())
    }
}

impl <'a> Proof <'a> {
    fn record_deduction_step (
        &mut self,
        prop: &Prop,
        den: &Den,
    ) {
        if let Den::Conj (
            _, prop_vec
        ) = den {
            self.tree_stack.push (DeductionTree {
                conj_name: prop.name.clone (),
                arity: prop_vec.len (),
                body: Vec::new (),
            });
            self.converge_deduction_tree ();
        }
    }
}

impl <'a> Proof <'a> {
    fn converge_deduction_tree (&mut self) {
        let mut last = self.tree_stack.pop () .unwrap ();
        loop {
            if let Some (mut next) = self.tree_stack.pop () {
                if last.full_p () {
                    next.body.push (last);
                    last = next;
                } else {
                    self.tree_stack.push (next);
                    break;
                }
            } else {
                break;
            }
        }
        self.tree_stack.push (last);
    }
}

// impl <'a> Proof <'a> {
//     fn converge_deduction_tree (&mut self) {
//         loop {
//             let last = self.tree_stack.pop () .unwrap ();
//             if last.full_p () && ! self.tree_stack.is_empty () {
//                 let mut next = self.tree_stack.pop () .unwrap ();
//                 next.body.push (last);
//                 self.tree_stack.push (next);
//             } else {
//                 self.tree_stack.push (last);
//                 return;
//             }
//         }
//     }
// }

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum ProofStep <'a> {
    One (Qed),
    More (VecDeque <Proof <'a>>),
    Fail,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct DeductionTree {
    // there are no position for Disj in the DeductionTree
    //   because Disj is not constructive -- sort of ~
    conj_name: String,
    arity: usize,
    body: Vec <DeductionTree>,
}

impl DeductionTree {
    fn full_p (&self) -> bool {
        self.body.len () == self.arity
    }
}

impl ToString for DeductionTree {
    fn to_string (&self) -> String {
        format! (
            "{} {{ {} }}",
            self.conj_name,
            vec_to_string (&self.body, " "))
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct Qed {
    subst: Subst,
    deduction_tree: DeductionTree,
}

const GRAMMAR: &'static str = r#"
Statement::Den = { den-name? "=" Den }
Statement::Query = { "query" '(' num? ')' '{' list (Prop) '}' }
Statement::Prove = { "prove" '(' num? ')' '{' list (Prop) '}' }
Den::Disj = { "disj" '(' list (den-name?) ')' }
Den::Conj = { "conj" '(' list (Term) ')' '{' list (Prop) '}' }
Term::Var = { unique-var-name? }
Term::Cons = { cons-name? '(' list (Term) ')' }
Prop::Cons = { den-name? '(' list (Term) ')' }
"#;

fn note_about_grammar () -> ErrorMsg {
    ErrorMsg::new ()
        .head ("grammar :")
        .lines (GRAMMAR)
}

fn mexp_to_prop_name <'a> (
    mexp: &Mexp <'a>,
) -> Result <String, ErrorInCtx> {
    if let Mexp::Sym {
        symbol,
        ..
    } = mexp {
        if symbol.ends_with ("-t") {
            Ok (symbol.to_string ())
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name")
                .line ("which must end with `-t`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .line ("expecting prop name")
            .line (&format! ("mexp = {}", mexp.to_string ()))
            .span (mexp.span ())
            .wrap_in_err ()
    }
}

fn mexp_to_prop <'a> (
    mexp: &Mexp <'a>,
) -> Result <Prop, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol,
            ..
        },
        arg: MexpArg::Tuple {
            body,
            ..
        },
        ..
    } = mexp {
        if symbol.ends_with ("-t") {
            Ok (Prop {
                name: symbol.to_string (),
                args: mexp_vec_to_term_vec (body)?,
            })
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name")
                .line ("which must end with `-t`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_term <'a> (
    mexp: &Mexp <'a>,
) -> Result <Term, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol,
            ..
        },
        arg: MexpArg::Tuple {
            body,
            ..
        },
        ..
    } = mexp {
        if symbol.ends_with ("-c") {
            Ok (Term::cons (
                symbol,
                mexp_vec_to_term_vec (body)?))
        } else {
            ErrorInCtx::new ()
                .line ("expecting cons name")
                .line ("which must end with `-c`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else if let Mexp::Sym {
        symbol,
        ..
    } = mexp {
        if symbol.ends_with ("-c") {
            Ok (Term::cons (symbol, vec! []))
        } else if symbol.ends_with ("-t") {
            ErrorInCtx::new ()
                .line ("expecting cons name or var")
                .line ("but found prop name which end with `-t`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        } else {
            Ok (Term::var_no_id (symbol))
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_disj_den <'a> (
    mexp: &Mexp <'a>,
) -> Result <Den, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol: "disj",
            ..
        },
        arg: MexpArg::Tuple {
            body,
            ..
        },
        ..
    } = mexp {
        Ok (Den::Disj (mexp_vec_to_prop_name_vec (body)?))
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_conj_den <'a> (
    mexp: &Mexp <'a>,
) -> Result <Den, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "conj",
                ..
            },
            arg: MexpArg::Tuple {
                body: body1,
                ..
            },
            ..
        },
        arg: MexpArg::Block {
            body: body2,
            ..
        },
        ..
    } = mexp {
        Ok (Den::Conj (mexp_vec_to_term_vec (body1)?,
                        mexp_vec_to_prop_vec (body2)?))
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_den <'a> (
    mexp: &Mexp <'a>,
) -> Result <Den, ErrorInCtx> {
    mexp_to_disj_den (mexp)
        .or (mexp_to_conj_den (mexp))
}

fn mexp_to_den_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    if let Mexp::Infix {
        op: "=",
        lhs: box Mexp::Sym {
            symbol,
            ..
        },
        rhs,
        ..
    } = mexp {
        if symbol.ends_with ("-t") {
            Ok (Statement::Den (
                symbol.to_string (),
                mexp_to_den (rhs)?))
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name")
                .line ("which must end with `-t`")
                .line (&format! ("symbol = {}", symbol))
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_prop_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "query",
                ..
            },
            arg: MexpArg::Tuple {
                body: body1,
                ..
            },
            ..
        },
        arg: MexpArg::Block {
            body: body2,
            ..
        },
        ..
    } = mexp {
        if let [
            Mexp::Sym { symbol, .. }
        ] = &body1 [..] {
            let result = symbol.parse::<usize> ();
            if result.is_err () {
                return ErrorInCtx::new ()
                    .line ("fail to parse usize num in `prop`")
                    .line (&format! ("symbol = {}", symbol))
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ();
            }
            Ok (Statement::Query (
                result.unwrap (),
                mexp_vec_to_prop_vec (body2)?))
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse prop's first arg")
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_prove_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "prove",
                ..
            },
            arg: MexpArg::Tuple {
                body: body1,
                ..
            },
            ..
        },
        arg: MexpArg::Block {
            body: body2,
            ..
        },
        ..
    } = mexp {
        if let [
            Mexp::Sym { symbol, .. }
        ] = &body1 [..] {
            let result = symbol.parse::<usize> ();
            if result.is_err () {
                return ErrorInCtx::new ()
                    .line ("fail to parse usize num in `prove`")
                    .line (&format! ("symbol = {}", symbol))
                    .span (mexp.span ())
                    .note (note_about_grammar ())
                    .wrap_in_err ();
            }
            Ok (Statement::Prove (
                result.unwrap (),
                mexp_vec_to_prop_vec (body2)?))
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse prop's first arg")
                .span (mexp.span ())
                .note (note_about_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    mexp_to_den_statement (mexp)
        .or (mexp_to_prop_statement (mexp))
        .or (mexp_to_prove_statement (mexp))
}

fn mexp_vec_to_prop_name_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <String>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_prop_name (&mexp)?);
    }
    Ok (vec)
}

fn mexp_vec_to_prop_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <Prop>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_prop (&mexp)?);
    }
    Ok (vec)
}

fn mexp_vec_to_term_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <Term>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_term (&mexp)?);
    }
    Ok (vec)
}

fn mexp_vec_to_statement_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <Statement>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_statement (&mexp)?);
    }
    Ok (vec)
}

#[test]
fn test_unify () {
    let u = Term::var ("u");
    let v = Term::var ("v");
    let subst = Subst::new () .unify (
        &Term::cons ("pair-c", vec! [
            u.clone (),
            v.clone (),
        ]),
        &Term::cons ("pair-c", vec! [
            v.clone (),
            Term::cons ("hi-c", vec! []),
        ])) .unwrap ();
    assert_eq! (subst.len (), 2);
}

#[test]
fn test_love () {
    let mut wissen = Wissen::new ();
    let den = Den::Conj (
        vec! [Term::cons ("you-c", vec! [])],
        vec! []);
    wissen.den ("love-t", &den);
    let prop = Prop {
        name: "love-t".to_string (),
        args: vec! [Term::var ("u")],
    };
    let mut proving = wissen.proving (&vec! [prop]);
    assert! (proving.next_qed () .is_some ());
    assert! (proving.next_qed () .is_none ());
}

#[test]
fn test_list_append () {
    let mut wissen = Wissen::new ();
    let list_append_t = Den::Disj (
        vec! [
            "zero-append-t".to_string (),
            "succ-append-t".to_string (),
        ]);
    wissen.den ("list-append-t", &list_append_t);
    let succ = Term::var ("succ");
    let zero_append_t = Den::Conj (
        vec! [
            Term::cons ("null-c", vec! []),
            succ.clone (),
            succ
        ],
        vec! []);
    wissen.den ("zero-append-t", &zero_append_t);
    let car = Term::var ("car");
    let cdr = Term::var ("cdr");
    let succ = Term::var ("succ");
    let o_cdr = Term::var ("o-cdr");
    let succ_append_t = Den::Conj (
        vec! [
            Term::cons ("cons-c",
                         vec! [car.clone (),
                               cdr.clone ()]),
            succ.clone (),
            Term::cons ("cons-c",
                         vec! [car,
                               o_cdr.clone ()]),
        ],
        vec! [
            Prop {
                name: "list-append-t".to_string (),
                args: vec! [
                    cdr,
                    succ,
                    o_cdr,
                ],
            }
        ]);
    wissen.den ("succ-append-t", &succ_append_t);
    let prop = Prop {
        name: "list-append-t".to_string (),
        args: vec! [Term::var ("x"),
                    Term::var ("y"),
                    Term::var ("z")],
    };
    let mut proving = wissen.proving (&vec! [prop]);
    let subst_vec = proving.take_subst (100);
    assert_eq! (subst_vec.len (), 100);
    for subst in subst_vec {
        assert! (subst.cons_p ());
    }
}

#[cfg (test)]
const LIST_APPEND_EXAMPLE: &'static str = "
list-append-t = disj (
    zero-append-t
    succ-append-t
)

zero-append-t = conj (null-c succ succ) {}

succ-append-t = conj (
    cons-c (car cdr)
    succ
    cons-c (car o-cdr)
) {
    list-append-t (cdr succ o-cdr)
}

query (10) { list-append-t (x y z) }
";

#[test]
fn test_mexp () -> Result <(), ErrorInCtx> {
    let mut wissen = Wissen::new ();
    let input = LIST_APPEND_EXAMPLE;
    match wissen.wis (input) {
        Ok (mut output_vec) => {
            assert_eq! (output_vec.len (), 1);
            if let WissenOutput::Query {
                subst_vec, ..
            } = output_vec.pop () .unwrap () {
                assert_eq! (subst_vec.len (), 10);
                for subst in subst_vec {
                    assert! (subst.cons_p ());
                }
            } else {
                panic! ()
            }
        }
        Err (error) => {
            let ctx = ErrorCtx::new ()
                .body (input);
            error.print (ctx);
        }
    }
    Ok (())
}
