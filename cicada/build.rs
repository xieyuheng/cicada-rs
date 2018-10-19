use std::io;

fn main () -> io::Result <()> {
    md_tangle_engine::tangle_all_before_build ()?;
    org_tangle_engine::tangle_all_before_build ()?;
    Ok (())
}
