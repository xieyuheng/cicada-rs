
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
    report_obj_dic,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CicadaModule {
    module: Module
}

#[wasm_bindgen]
impl CicadaModule {
    pub fn clone (&self) -> Self {
        CicadaModule { module: self.module.clone () }
    }
}

#[wasm_bindgen]
impl CicadaModule {
    pub fn new () -> CicadaModule {
        let module = Module::new ();
        CicadaModule { module }
    }
}

#[wasm_bindgen]
impl CicadaModule {
    pub fn run (&mut self, input: &str) -> String {
        match self.module.run (input) {
            Ok (obj_dic) => {
                report_obj_dic (&obj_dic)
            }
            Err (error) => {
                error.report (ErrorCtx::new ()
                              .body (&input))
            }
        }
    }
}
