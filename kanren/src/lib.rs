#![feature (uniform_paths)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

use std::sync::Arc;
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
