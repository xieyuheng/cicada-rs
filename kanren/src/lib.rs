#![feature (uniform_paths)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use error_report::{
    Span,
    ErrorMsg,
    ErrorCtx,
    ErrorInCtx,
};
use mexp_parser::{
    SyntaxTable,
    Mexp,
    Arg,
};

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub enum Exp {
    DVar { id: Uuid, name: String },
    TVar { id: Uuid, name: String },
    Type { level: usize },
    TypeCons {  },
    DataCons {  },
}
