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
    Arg,
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
    id: Option <Id>,
}

impl ToString for VarTerm {
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
#[derive (PartialEq, Eq, Hash)]
pub struct TupleTerm {
    head: String,
    body: Vec <Term>,
}

impl ToString for TupleTerm {
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
        Term::Var (VarTerm {
            name: s.to_string (),
            id: Some (Id::uuid ()),
        })
    }
}

impl Term {
    fn var_no_id (s: &str) -> Term {
        Term::Var (VarTerm {
            name: s.to_string (),
            id: None,
        })
    }
}

impl Term {
    fn var_local (s: &str, counter: usize) -> Term {
        Term::Var (VarTerm {
            name: s.to_string (),
            id: Some (Id::local (counter)),
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
                        id: Some (Id::uuid ()),
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

impl ToString for Term {
    fn to_string (&self) -> String {
        match self {
            Term::Var (var) => var.to_string (),
            Term::Tuple (tuple) => tuple.to_string (),
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
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
                if self.occur_check (&u, &v) {
                    None
                } else {
                    Some (self.extend (u, v))
                }
            }
            (u, Term::Var (v)) => {
                if self.occur_check (&v, &u) {
                    None
                } else {
                    Some (self.extend (v, u))
                }
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
    pub fn apply (&self, term: &Term) -> Term {
        let term = self.walk (term);
        match term {
            Term::Var (_) => term,
            Term::Tuple (TupleTerm {
                head,
                body,
            }) => {
                let body = body.iter ()
                    .map (|x| self.apply (x))
                    .collect ();
                Term::Tuple (TupleTerm {
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
            Term::Tuple (TupleTerm {
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
    pub fn reify_var (&self, var: &VarTerm) -> Term {
        let term = Term::Var (var.clone ());
        let term = self.apply (&term);
        let new_subst = Subst::new ();
        let local_subst = new_subst.localize_by_term (&term);
        local_subst.apply (&term)
    }
}

impl Subst {
    pub fn occur_check (
        &self,
        var: &VarTerm,
        term: &Term,
    ) -> bool {
        let term = self.walk (term);
        match term {
            Term::Var (var1) => {
                var == &var1
            }
            Term::Tuple (tuple) => {
                for term in &tuple.body {
                    if self.occur_check (var, term) {
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
#[derive (PartialEq, Eq, Hash)]
pub enum Prop {
    Disj (Vec <String>),
    Conj (Vec <Term>, Vec <Query>),
}

impl Prop {
    fn duplicate (
        &self,
        var_map: &mut HashMap <VarTerm, VarTerm>,
    ) -> Prop {
        match self {
            Prop::Disj (_name_vec) => {
                self.clone ()
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

impl Prop {
    fn apply (
        self,
        args: &Vec <Term>,
        mut subst: Subst,
    ) -> Option <(Vec <Vec <Arc <Query>>>, Subst)> {
        match self {
            Prop::Disj (name_vec) => {
                let mut query_matrix = Vec::new ();
                for name in name_vec {
                    let query = Arc::new (Query {
                        name: name,
                        args: args.clone (),
                    });
                    query_matrix.push (vec! [query]);
                }
                Some ((query_matrix, subst))
            }
            Prop::Conj (terms, query_vec) => {
                if args.len () != terms.len () {
                    eprintln! ("- [warning] Prop::apply");
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
                let query_matrix = vec! [
                    query_vec
                        .into_iter ()
                        .map (|x| Arc::new (x))
                        .collect ()
                ];
                Some ((query_matrix, subst))
            }
        }
    }
}

impl ToString for Prop {
    fn to_string (&self) -> String {
        match self {
            Prop::Disj (name_vec) => {
                format! (
                    "disj ({})",
                    vec_to_string (&name_vec, " "))
            }
            Prop::Conj (terms, query_vec) => {
                format! (
                    "conj ({}) {}",
                    vec_to_string (&terms, " "),
                    if query_vec.len () == 0 {
                        format! ("{{}}")
                    } else {
                        format! (
                            "{{ {} }}",
                            vec_to_string (&query_vec, " "))
                    })
            }
        }
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

impl ToString for Query {
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
    prop_dic: Dic <Prop>,
}

impl Wissen {
    pub fn new () -> Self {
        Wissen {
            prop_dic: Dic::new (),
        }
    }
}

impl Wissen {
    fn find_prop (&self, name: &str) -> Option <Prop> {
        if let Some (
            prop
        ) = self.prop_dic.get (name) {
            Some (prop.fresh ())
        } else {
            None
        }
    }
}

impl Wissen {
    pub fn proving <'a> (
        &'a self,
        query_vec: &Vec <Query>,
    ) -> Proving <'a> {
        let mut query_queue = VecDeque::new ();
        for query in query_vec {
            query_queue.push_back (Arc::new (query.clone ()))
        }
        let proof = Proof {
            wissen: self,
            tree_stack: vec! [DeductionTree {
               conj_name: "PROOF".to_string (),
               arity: query_vec.len (),
               body: Vec::new (),
            }],
            subst: Subst::new (),
            query_queue,
        };
        Proving {
            proof_queue: vec! [proof] .into (),
        }
    }
}

impl ToString for Wissen {
    fn to_string (&self) -> String {
        let mut s = String::new ();
        for (name, prop) in self.prop_dic.iter () {
            s += name;
            s += " = ";
            s += &prop.to_string ();
            s += "\n";
        }
        format! ("<wissen>\n{}</wissen>\n", s)
    }
}

impl Wissen {
    pub fn prop (&mut self, name: &str, prop: &Prop) {
       self.prop_dic.ins (name, Some (prop.clone ()));
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
            if let Statement::Prop (
                name, prop
            ) = statement {
                self.prop (name, prop);
            }
        }
        let mut output_vec = Vec::new ();
        for statement in &statement_vec {
            if let Statement::Query (
                counter, query_vec
            ) = statement {
                let mut proving = self.proving (query_vec);
                let subst_vec = proving.take_subst (*counter);
                output_vec.push (WissenOutput::Query {
                    counter: *counter,
                    query_vec: query_vec.clone (),
                    subst_vec,
                });
            }
            if let Statement::Prove (
                counter, query_vec
            ) = statement {
                let mut proving = self.proving (query_vec);
                let qed_vec = proving.take_qed (*counter);
                output_vec.push (WissenOutput::Prove {
                    counter: *counter,
                    query_vec: query_vec.clone (),
                    qed_vec,
                });
            }
        }
        Ok (output_vec)
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub enum WissenOutput {
    Query {
        counter: usize,
        query_vec: Vec <Query>,
        subst_vec: Vec <Subst>,
    },
    Prove {
        counter: usize,
        query_vec: Vec <Query>,
        qed_vec: Vec <Qed>,
    },
}

fn collect_var_from_query_vec (
    query_vec: &Vec <Query>
) -> HashSet <VarTerm> {
    let mut var_set = HashSet::new ();
    for query in query_vec {
        for var in collect_var_from_term_vec (&query.args) {
            var_set.insert (var);
        }
    }
    var_set
}

fn collect_var_from_term (
    term: &Term
) -> HashSet <VarTerm> {
    match term {
        Term::Var (var) => {
            let mut var_set = HashSet::new ();
            var_set.insert (var.clone ());
            var_set
        }
        Term::Tuple (tuple) => {
            collect_var_from_term_vec (&tuple.body)
        }
    }
}

fn collect_var_from_term_vec (
    term_vec: &Vec <Term>
) -> HashSet <VarTerm> {
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
                counter,
                query_vec,
                subst_vec,
            } => {
                let mut s = String::new ();
                s += "<query-output>\n";
                s += &vec_to_lines (&query_vec);
                s += "- expecting ";
                s += &counter.to_string ();
                s += " results\n";
                let var_set = collect_var_from_query_vec (
                    query_vec);
                for subst in subst_vec {
                    for var in &var_set {
                        s += &var.to_string ();
                        s += " = ";
                        s += &subst.reify_var (var) .to_string ();
                        s += "\n";
                    }
                    s += "\n";
                }
                s += "</query-output>";
                s
            }
            WissenOutput::Prove {
                counter,
                query_vec,
                qed_vec,
            } => {
                let mut s = String::new ();
                s += "<prove-output>\n";
                s += &vec_to_lines (&query_vec);
                s += "- expecting ";
                s += &counter.to_string ();
                s += " results\n";
                let var_set = collect_var_from_query_vec (
                    query_vec);
                for qed in qed_vec {
                    s += "<deduction-tree>\n";
                    s += &qed.deduction_tree.to_string ();
                    s += "\n";
                    s += "</deduction-tree>\n";
                    for var in &var_set {
                        s += &var.to_string ();
                        s += " = ";
                        s += &qed.subst.reify_var (var) .to_string ();
                        s += "\n";
                    }
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
#[derive (PartialEq, Eq, Hash)]
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
#[derive (PartialEq, Eq, Hash)]
pub enum Statement {
    Prop (String, Prop),
    Query (usize, Vec <Query>),
    Prove (usize, Vec <Query>),
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
                ProofStep::Finished => {
                    if let Some (
                        deduction_tree
                    ) = proof.tree_stack.pop () {
                        return Some (Qed {
                            subst: proof.subst,
                            deduction_tree,
                        });
                    } else {
                        eprintln! ("next_qed");
                        panic! ("!!!!!!!!!!!!!!!!!!!!!!!!!");
                    }
                }
                ProofStep::MoreTodo (proof_queue) => {
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
    query_queue: VecDeque <Arc <Query>>,
}

impl <'a> Proof <'a> {
    fn step (&mut self) -> ProofStep <'a> {
        if let Some (query) = self.query_queue.pop_front () {
            if let Some (
                prop
            ) = self.wissen.find_prop (&query.name) {
                let mut proof_queue = VecDeque::new ();
                let backup_prop = prop.clone ();
                if let Some (
                    (query_matrix, new_subst)
                ) = prop.apply (&query.args, self.subst.clone ()) {
                    for query_vec in query_matrix {
                        let mut proof = self.clone ();
                        proof.subst = new_subst.clone ();
                        proof.record_deduction_step (
                            &query,
                            &backup_prop);
                        let rev = query_vec.into_iter () .rev ();
                        for query in rev {
                            // the order must be kept
                            //   to record_deduction_step
                            proof.query_queue.push_front (query);
                        }
                        proof_queue.push_back (proof);
                    }
                    ProofStep::MoreTodo (proof_queue)
                } else {
                    ProofStep::Fail
                }
            } else {
                eprintln! ("- [warning] Proof::step");
                eprintln! ("  undefined prop : {}", query.name);
                ProofStep::Fail
            }
        } else {
            ProofStep::Finished
        }
    }
}

impl <'a> ToString for Proof <'a> {
    fn to_string (&self) -> String {
        let query_vec: &Vec <Query> = &self.query_queue
            .iter ()
            .map (|x| (**x) .clone ())
            .collect ();
        format! (
            "<proof>\n\
            <query_queue>\n\
            {}</query_queue>\n\
            <subst>\n\
            {}</subst>\n\
            </proof>\n",
            vec_to_lines (query_vec),
            self.subst.to_string ())
    }
}

impl <'a> Proof <'a> {
    fn record_deduction_step (
        &mut self,
        query: &Query,
        prop: &Prop,
    ) {
        self.converge_deduction_tree ();
        if let Prop::Conj (
            _, query_vec
        ) = prop {
            let new = DeductionTree {
                conj_name: query.name.clone (),
                arity: query_vec.len (),
                body: Vec::new (),
            };
            if let Some (mut old) = self.tree_stack.pop () {
                old.body.push (new);
                self.tree_stack.push (old);
            } else {
                eprintln! ("- record_deduction_step");
                eprintln! ("  query = {}", query.to_string ());
                eprintln! ("  prop = {}", prop.to_string ());
                panic! ("!!!!!!!!!!!!!!!!!!!!!!!!!");
            }
        }
    }
}

impl <'a> Proof <'a> {
    fn converge_deduction_tree (&mut self) {
        loop {
            let last = self.tree_stack.pop () .unwrap ();
            if last.full_p () && ! self.tree_stack.is_empty () {
                let mut next = self.tree_stack.pop () .unwrap ();
                next.body.push (last);
                self.tree_stack.push (next);
            } else {
                self.tree_stack.push (last);
                return;
            }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum ProofStep <'a> {
    Finished,
    MoreTodo (VecDeque <Proof <'a>>),
    Fail,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub struct Qed {
    subst: Subst,
    deduction_tree: DeductionTree,
}

const WISSEN_GRAMMAR: &'static str = r#"
Statement::Prop = { prop-name? "=" Prop }
Statement::Query = { "query" '(' num? ')' '{' list (Query) '}' }
Statement::Prove = { "prove" '(' num? ')' '{' list (Query) '}' }
Prop::Disj = { "disj" '(' list (prop-name?) ')' }
Prop::Conj = { "conj" '(' list (Term) ')' '{' list (Query) '}' }
Term::Var = { unique-var-name? }
Term::Tuple = { tuple-name? '(' list (Term) ')' }
Query::Tuple = { prop-name? '(' list (Term) ')' }
"#;

fn note_about_wissen_grammar () -> ErrorMsg {
    ErrorMsg::new ()
        .head ("wissen grammar :")
        .lines (WISSEN_GRAMMAR)
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
                .line (&format! ("but found : {}", symbol))
                .span (mexp.span ())
                .note (note_about_wissen_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .line ("expecting prop name")
            .line (&format! ("but found : {}", mexp.to_string ()))
            .span (mexp.span ())
            .wrap_in_err ()
    }
}

fn mexp_to_disj_prop <'a> (
    mexp: &Mexp <'a>,
) -> Result <Prop, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol: "disj",
            ..
        },
        arg: Arg::Tuple {
            body,
            ..
        },
        ..
    } = mexp {
        Ok (Prop::Disj (mexp_vec_to_prop_name_vec (body)?))
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_wissen_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_query <'a> (
    mexp: &Mexp <'a>,
) -> Result <Query, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Sym {
            symbol,
            ..
        },
        arg: Arg::Tuple {
            body,
            ..
        },
        ..
    } = mexp {
        if symbol.ends_with ("-t") {
            Ok (Query {
                name: symbol.to_string (),
                args: mexp_vec_to_term_vec (body)?,
            })
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name")
                .line ("which must end with `-t`")
                .line (&format! ("but found : {}", symbol))
                .span (mexp.span ())
                .note (note_about_wissen_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_wissen_grammar ())
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
        arg: Arg::Tuple {
            body,
            ..
        },
        ..
    } = mexp {
        if symbol.ends_with ("-c") {
            Ok (Term::tuple (
                symbol,
                mexp_vec_to_term_vec (body)?))
        } else {
            ErrorInCtx::new ()
                .line ("expecting tuple name")
                .line ("which must end with `-c`")
                .line (&format! ("but found : {}", symbol))
                .span (mexp.span ())
                .note (note_about_wissen_grammar ())
                .wrap_in_err ()
        }
    } else if let Mexp::Sym {
        symbol,
        ..
    } = mexp {
        if symbol.ends_with ("-c") {
            Ok (Term::tuple (symbol, vec! []))
        } else if symbol.ends_with ("-t") {
            ErrorInCtx::new ()
                .line ("expecting tuple name or var")
                .line ("but found prop name which end with `-t`")
                .line (&format! ("prop name : {}", symbol))
                .span (mexp.span ())
                .note (note_about_wissen_grammar ())
                .wrap_in_err ()
        } else {
            Ok (Term::var_no_id (symbol))
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_wissen_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_conj_prop <'a> (
    mexp: &Mexp <'a>,
) -> Result <Prop, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "conj",
                ..
            },
            arg: Arg::Tuple {
                body: body1,
                ..
            },
            ..
        },
        arg: Arg::Block {
            body: body2,
            ..
        },
        ..
    } = mexp {
        Ok (Prop::Conj (mexp_vec_to_term_vec (body1)?,
                        mexp_vec_to_query_vec (body2)?))
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_wissen_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_prop <'a> (
    mexp: &Mexp <'a>,
) -> Result <Prop, ErrorInCtx> {
    mexp_to_disj_prop (mexp)
        .or (mexp_to_conj_prop (mexp))
}

fn mexp_to_prop_statement <'a> (
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
            Ok (Statement::Prop (
                symbol.to_string (),
                mexp_to_prop (rhs)?))
        } else {
            ErrorInCtx::new ()
                .line ("expecting prop name")
                .line ("which must end with `-t`")
                .line (&format! ("but found : {}", symbol))
                .span (mexp.span ())
                .note (note_about_wissen_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_wissen_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_query_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    if let Mexp::Apply {
        head: box Mexp::Apply {
            head: box Mexp::Sym {
                symbol: "query",
                ..
            },
            arg: Arg::Tuple {
                body: body1,
                ..
            },
            ..
        },
        arg: Arg::Block {
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
                    .line ("fail to parse usize num in `query`")
                    .line (&format! ("symbol : {}", symbol))
                    .span (mexp.span ())
                    .note (note_about_wissen_grammar ())
                    .wrap_in_err ();
            }
            Ok (Statement::Query (
                result.unwrap (),
                mexp_vec_to_query_vec (body2)?))
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse query's first arg")
                .span (mexp.span ())
                .note (note_about_wissen_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_wissen_grammar ())
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
            arg: Arg::Tuple {
                body: body1,
                ..
            },
            ..
        },
        arg: Arg::Block {
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
                    .line (&format! ("symbol : {}", symbol))
                    .span (mexp.span ())
                    .note (note_about_wissen_grammar ())
                    .wrap_in_err ();
            }
            Ok (Statement::Prove (
                result.unwrap (),
                mexp_vec_to_query_vec (body2)?))
        } else {
            ErrorInCtx::new ()
                .line ("fail to parse query's first arg")
                .span (mexp.span ())
                .note (note_about_wissen_grammar ())
                .wrap_in_err ()
        }
    } else {
        ErrorInCtx::new ()
            .head ("syntex error")
            .span (mexp.span ())
            .note (note_about_wissen_grammar ())
            .wrap_in_err ()
    }
}

fn mexp_to_statement <'a> (
    mexp: &Mexp <'a>,
) -> Result <Statement, ErrorInCtx> {
    mexp_to_prop_statement (mexp)
        .or (mexp_to_query_statement (mexp))
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

fn mexp_vec_to_query_vec <'a> (
    mexp_vec: &Vec <Mexp <'a>>,
) -> Result <Vec <Query>, ErrorInCtx> {
    let mut vec = Vec::new ();
    for mexp in mexp_vec {
        vec.push (mexp_to_query (&mexp)?);
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
        &Term::tuple ("pair-c", vec! [
            u.clone (),
            v.clone (),
        ]),
        &Term::tuple ("pair-c", vec! [
            v.clone (),
            Term::tuple ("hi-c", vec! []),
        ])) .unwrap ();
    assert_eq! (subst.len (), 2);
}

#[test]
fn test_love () {
    let mut wissen = Wissen::new ();
    let prop = Prop::Conj (
        vec! [Term::tuple ("you-c", vec! [])],
        vec! []);
    wissen.prop ("love-t", &prop);
    let query = Query {
        name: "love-t".to_string (),
        args: vec! [Term::var ("u")],
    };
    let mut proving = wissen.proving (&vec! [query]);
    assert! (proving.next_qed () .is_some ());
    assert! (proving.next_qed () .is_none ());
}

#[test]
fn test_list_append () {
    let mut wissen = Wissen::new ();
    let list_append_t = Prop::Disj (
        vec! [
            "zero-append-t".to_string (),
            "succ-append-t".to_string (),
        ]);
    wissen.prop ("list-append-t", &list_append_t);
    let succ = Term::var ("succ");
    let zero_append_t = Prop::Conj (
        vec! [
            Term::tuple ("null-c", vec! []),
            succ.clone (),
            succ
        ],
        vec! []);
    wissen.prop ("zero-append-t", &zero_append_t);
    let car = Term::var ("car");
    let cdr = Term::var ("cdr");
    let succ = Term::var ("succ");
    let o_cdr = Term::var ("o-cdr");
    let succ_append_t = Prop::Conj (
        vec! [
            Term::tuple ("cons-c",
                         vec! [car.clone (),
                               cdr.clone ()]),
            succ.clone (),
            Term::tuple ("cons-c",
                         vec! [car,
                               o_cdr.clone ()]),
        ],
        vec! [
            Query {
                name: "list-append-t".to_string (),
                args: vec! [
                    cdr,
                    succ,
                    o_cdr,
                ],
            }
        ]);
    wissen.prop ("succ-append-t", &succ_append_t);
    let query = Query {
        name: "list-append-t".to_string (),
        args: vec! [Term::var ("x"),
                    Term::var ("y"),
                    Term::var ("z")],
    };
    let mut proving = wissen.proving (&vec! [query]);
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
            error.report (ctx);
        }
    }
    Ok (())
}
