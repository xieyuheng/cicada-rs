#![feature (uniform_paths)]
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
use wissen::{
    Wissen,
    // WissenOutput,
};

fn main () {
    let matches = cmd::App::new ("wissen // wis")
        .author (cmd::crate_authors! ())
        .version (cmd::crate_version! ())
        .setting (cmd::AppSettings::ArgRequiredElseHelp)
        .arg (cmd::Arg::with_name ("FILE")
              .help ("paths to `.wis` files")
              .multiple (true))
        .get_matches ();
    let mut wissen = Wissen::new ();
    if let Some (paths) = matches.values_of ("FILE") {
        for path_str in paths {
            let path = Path::new (path_str);
            if path.is_file () {
                let input = fs::read_to_string (path) .unwrap ();
                match wissen.wis (&input) {
                    Ok (output_vec) => {
                        for output in output_vec {
                            println! ("{}", output.to_string ());
                        }
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
}
