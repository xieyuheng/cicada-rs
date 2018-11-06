use std::path::Path;
use std::io;

use clap as cmd;

fn main () -> io::Result <()> {
    let matches = cmd::App::new ("jojo")
        .author (cmd::crate_authors! ())
        .version (cmd::crate_version! ())
        .arg (cmd::Arg::with_name ("FILE")
              .help ("Paths to script files")
              .multiple (true))
        .after_help ("REPL:  type `jojo` to run the repl")
        .get_matches ();
    if matches.occurrences_of ("FILE") == 0 {
        jojo_cli::repl::repl ()
    } else {
        if let Some (paths) = matches.values_of ("FILE") {
            for path in paths {
                jojo::load (Path::new (&path));
            }
        }
        Ok (())
    }
}
