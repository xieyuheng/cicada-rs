#![feature (uniform_paths)]
#![feature (bind_by_move_pattern_guards)]
#![feature (box_patterns)]
#![feature (box_syntax)]

#![allow (dead_code)]

use error_report::{
    // ErrorMsg,
    ErrorCtx,
    // ErrorInCtx,
};
use cicada::{
    Module,
    // WissenOutput,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run (input: &str) -> String {
    let mut module = Module::new ();
    match module.run (input) {
        Ok (()) => {
            module.report ()
        }
        Err (error) => {
            error.report (ErrorCtx::new ()
                          .body (&input))
        }
    }
}
