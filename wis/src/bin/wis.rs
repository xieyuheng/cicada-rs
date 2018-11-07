#![feature (uniform_paths)]
#![feature (bind_by_move_pattern_guards)]
#![feature (box_patterns)]
#![feature (box_syntax)]

#![allow (dead_code)]

use std::path::Path;
use std::collections::VecDeque;
use std::io;
use std::fs;
use clap as cmd;
use rustyline::{
    Editor,
    error::{
        ReadlineError,
    },
};
use error_report::{
    // ErrorMsg,
    ErrorCtx,
    // ErrorInCtx,
};
use wissen::{
    Wissen,
    Proving,
};

static COLORED_PROMPT: &'static str = "\x1b[1;32m>\x1b[0m ";

fn repl (wissen: &mut Wissen) {
    let mut rl = Editor::<()>::new ();
    loop {
        let readline = rl.readline(COLORED_PROMPT);
        match readline {
            Ok (code) => {
                match wissen.wis (&code) {
                    Ok (proving_vec) => {
                        proving_vec_loop (proving_vec);
                    }
                    Err (error) => {
                        let ctx = ErrorCtx::new ()
                            .body (&code);
                        error.report (ctx);
                    }
                }
            },
            Err (ReadlineError::Interrupted) => {
                break
            },
            Err (ReadlineError::Eof) => {
                break
            },
            Err (err) => {
                eprintln!("- readline error : {:?}", err);
                break
            },
        }
    }
}

fn proving_vec_loop (proving_vec: Vec <Proving>) {
    if proving_vec.is_empty () {
        return;
    }
    let mut rl = Editor::<()>::new ();
    let mut queue: VecDeque <Proving> = proving_vec.into ();
    loop {
        let readline = rl.readline(".: ");
        match readline {
            Ok (command) => {
                match command.as_str () {
                    "." => {
                        if let Some (
                            mut proving
                        ) = queue.pop_front () {
                            if let Some (
                                subst
                            ) = proving.next_subst () {
                                println! ("- {}", subst.to_string ());
                                queue.push_front (proving);
                            } else {
                                println! ("- one proving finished")
                            }
                        } else {
                            break;
                        }
                    }
                    "/" => {
                        let _proving = queue.pop_front ();
                        println! ("- drop proving")
                    }
                    _ => {}
                }
            },
            Err (ReadlineError::Interrupted) => {
                break
            },
            Err (ReadlineError::Eof) => {
                break
            },
            Err (err) => {
                eprintln!("- readline error : {:?}", err);
                break
            },
        }
    }
}

fn main () -> io::Result <()> {
    let matches = cmd::App::new ("wissen // wis")
        .author (cmd::crate_authors! ())
        .version (cmd::crate_version! ())
        .arg (cmd::Arg::with_name ("FILE")
              .help ("paths to `.wis` files")
              .multiple (true))
        .get_matches ();
    let mut wissen = Wissen::new ();
    if let Some (paths) = matches.values_of ("FILE") {
        for path_str in paths {
            let path = Path::new (path_str);
            if path.is_file () {
                let code = fs::read_to_string (path)?;
                match wissen.wis (&code) {
                    Ok (proving_vec) => {
                        proving_vec_loop (proving_vec);
                    }
                    Err (error) => {
                        let ctx = ErrorCtx::new ()
                            .source (path_str)
                            .body (&code);
                        error.report (ctx);
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
    repl (&mut wissen);
    Ok (())
}
