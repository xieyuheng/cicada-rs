use std::fs;
use std::io;
use std::fs::DirEntry;
use std::path::Path;

fn visit_dir (
    dir: &Path,
    cb: &Fn (&DirEntry),
) -> io::Result <()> {
    if dir.is_dir () {
        for entry in fs::read_dir (dir)? {
            let entry = entry?;
            let path = entry.path ();
            if path.is_file () {
                cb (&entry);
            }
        }
        Ok (())
    } else {
        Err (io::Error::new (
            io::ErrorKind::InvalidInput,
            "path must be a path of dir"))
    }
}

fn run_all_scripts_in_dir (dir: &Path) -> io::Result <()> {
    visit_dir (dir, &|entry| {
        let path = entry.path ();
        let _env = jojo_core::load (&path);
    })
}

#[test]
fn test_jojo_semantic () -> io::Result <()> {
    run_all_scripts_in_dir (Path::new ("tests/jojo/semantic"))
}

#[test]
fn test_jojo_datatype () -> io::Result <()> {
    run_all_scripts_in_dir (Path::new ("tests/jojo/datatype"))
}

#[test]
fn test_jojo_syntax () -> io::Result <()> {
    run_all_scripts_in_dir (Path::new ("tests/jojo/syntax"))
}

#[test]
fn test_jojo_module_system () -> io::Result <()> {
    run_all_scripts_in_dir (Path::new ("tests/jojo/module-system"))
}
