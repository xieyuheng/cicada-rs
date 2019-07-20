

use std::path::Path;
use std::io;
use clap as cmd;

fn main () -> io::Result <()> {
    let matches = cmd::App::new ("jojo // jo")
        .author (cmd::crate_authors! ())
        .version (cmd::crate_version! ())
        .setting (cmd::AppSettings::ArgRequiredElseHelp)
        .arg (cmd::Arg::with_name ("FILE")
              .help ("paths to `.jo` files")
              .multiple (true))
        .get_matches ();
    if let Some (paths) = matches.values_of ("FILE") {
        for path in paths {
            jojo::load (Path::new (&path));
        }
    }
    Ok (())
}
