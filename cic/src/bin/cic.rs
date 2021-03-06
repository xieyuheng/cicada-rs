
#![feature (bind_by_move_pattern_guards)]
#![feature (box_patterns)]
#![feature (box_syntax)]

#![allow (dead_code)]

use std::path::Path;
use std::fs;
use clap as cmd;
use error_report::{
    // ErrorMsg,
    ErrorCtx,
    // ErrorInCtx,
};
use cicada::{
    Module,
    report_obj_dic,
};

fn main () {
    let matches = cmd::App::new ("cicada // cic")
        .author (cmd::crate_authors! ())
        .version (cmd::crate_version! ())
        .setting (cmd::AppSettings::ArgRequiredElseHelp)
        .arg (cmd::Arg::with_name ("FILE")
              .help ("path to `.cic` file"))
        .get_matches ();
    let mut module = Module::new ();
    if let Some (path_str) = matches.value_of ("FILE") {
        let path = Path::new (path_str);
        if path.is_file () {
            let input = fs::read_to_string (path) .unwrap ();
            match module.run (&input) {
                Ok (obj_dic) => {
                    print! ("{}", report_obj_dic (&obj_dic));
                }
                Err (error) => {
                    let ctx = ErrorCtx::new ()
                        .source (path_str)
                        .body (&input);
                    error.print (ctx);
                    std::process::exit (1);
                }
            }
        } else {
            eprintln! ("- input path is not to a file");
            eprintln! ("  path = {:?}", path);
            std::process::exit (1);
        }
    }
}
