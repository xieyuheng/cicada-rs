#![feature (uniform_paths)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

use std::collections::VecDeque;
use std::str::FromStr;
use std::fmt;
use regex::Regex;
use dic::Dic;

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Sym {
    Str {
        string: String,
    },
    Ref {
        rule_name: String,
    },
    RefChoice {
        rule_name: String,
        choice_name: String,
    },
    Call {
        rule_name: String,
        arg_dic: Dic <Sym>,
    },
    CallChoice {
        rule_name: String,
        choice_name: String,
        arg_dic: Dic <Sym>,
    },
    Union {
        exp_vec: Vec <Sym>,
    },
}

/// A rule is a dic of choices.
/// - single-choice-rule is not handled specially.
#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Rule {
    rule_name: String,
    ty: Ty,
    choice_dic: Dic <Choice>,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Ty {
    Str,
    Gr,
    Var {
        name: String,
    },
    Arrow {
        ante: Vec <Ty>,
        succ: Box <Ty>,
    },
}

/// A choice is a vec of exps.
#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Choice {
    rule_name: String,
    choice_name: String,
    ty: Ty,
    arg_dic: Dic <Sym>,
    exp_vec: Vec <Sym>,
}

/// `Sta` for statement
#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Sta {

}

/// A Grammar contains a dic of rules.
#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Grammar {
    rule_dic: Dic <Rule>,
}

impl Grammar {
    pub fn new () -> Self {
        Grammar { rule_dic: Dic::new () }
    }
}

#[derive (Debug)]
pub struct GrammarError;

impl Grammar {
    fn exe (
        &mut self,
        sta: Sta,
    ) -> Result <(), GrammarError> {
        unimplemented! ()
    }
}

impl Grammar {
    fn assign_choice (
        &mut self,
        sta: Sta,
    ) -> Result <(), GrammarError> {
        unimplemented! ()
    }
}

impl Grammar {
    fn assign_type (
        &mut self,
        sta: Sta,
    ) -> Result <(), GrammarError> {
        unimplemented! ()
    }
}

impl Grammar {
    fn assign_alias (
        &mut self,
        sta: Sta,
    ) -> Result <(), GrammarError> {
        unimplemented! ()
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct ParseTree {
    head: Choice,
    body: Vec <ParseTree>,
}
