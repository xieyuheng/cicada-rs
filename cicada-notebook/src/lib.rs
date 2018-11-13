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
use wissen::{
    Wissen,
    // WissenOutput,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wis (input: &str) -> String {
    let mut wissen = Wissen::new ();
    let mut s = String::new ();
    match wissen.wis (input) {
        Ok (output_vec) => {
            for output in output_vec {
                s += &output.to_string ();
            }
            s
        }
        Err (error) => {
            error.report (ErrorCtx::new ()
                          .body (&input))
        }
    }
}
