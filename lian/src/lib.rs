#![feature (uniform_paths)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]
#![allow (non_camel_case_types)]

use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Heap {
    memory: Vec <Term>,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Term {
    Struct {
        addr: usize,
    },
    Funtor {
        name: String,
        arity: usize,
    },
    Ref {
        addr: usize,
    },
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Ins {
    PUT_STRUCTURE {},//f=n Xi
    SET_VARIABLE {},//Xi
    SET_VALUE {},//Xi
}


